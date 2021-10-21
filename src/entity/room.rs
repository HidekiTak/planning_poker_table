use actix::{Actor, Context};
use actix_web::{web, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use anyhow::*;
use chrono::Utc;
use derive_getters::Getters;
use derive_new::new;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::{Rc, Weak};
use std::sync::{Arc, Mutex, Once};

use crate::entity::{Id, Player, PlayerStatus, RoomStatus};
use crate::web_socket_session::PlanningPokerSession;

#[derive(Debug, Getters, Default)]
pub struct Room {
    id: String,
    name: String,
    show: bool,
    agenda: Mutex<RefCell<String>>,
    options: Mutex<RefCell<Vec<String>>>,
    player_count: usize,
    players: Mutex<RefCell<Vec<Weak<RefCell<Player>>>>>,
    last_touch: i64,
}

impl Room {
    pub fn attend(&mut self, player: &Rc<RefCell<Player>>) {
        let vec = self.players.lock().unwrap();
        let mut v = vec.take();
        v = v.into_iter().filter(|x| x.upgrade().is_some()).collect();
        v.push(Rc::downgrade(player));
        self.player_count = v.len();
        vec.replace(v);
    }

    pub fn exit(&mut self, player_id: &str) -> bool {
        let count: usize;
        {
            let pls = self.players.lock().unwrap();
            let plsx: Vec<Weak<RefCell<Player>>> = pls.take();
            let r: Vec<Weak<RefCell<Player>>> = plsx
                .into_iter()
                .filter(|x| match x.upgrade() {
                    None => false,
                    Some(p) => {
                        let px = p.take();
                        let result = player_id != px.id().as_str();
                        p.replace(px);
                        result
                    }
                })
                .collect();
            count = r.len();
            self.player_count = count;
            pls.replace(r);
        }
        let close_room: bool = count == 0;
        if !close_room {
            self.updated();
        }
        close_room
    }

    pub fn open(&mut self) {
        self.show = true;
        self.updated();
    }

    pub fn reset(&mut self) {
        self.show = false;
        {
            let l = self.players.lock().unwrap();
            let mut x = l.take();
            x.iter_mut().for_each(|p| {
                let z = p.upgrade().unwrap();
                let mut player = z.take();
                player.reset();
                z.replace(player);
            });
            l.replace(x);
        }
        self.updated();
    }

    pub fn set_agenda(&mut self, agenda: &str) {
        let update_needed: bool;
        {
            let rc = self.agenda.lock().unwrap();
            let old = rc.take();
            update_needed = old != agenda;
            if old != agenda {
                rc.replace(agenda.to_string());
            }
        }
        if update_needed {
            self.updated();
        }
    }

    pub fn set_options(&mut self, options: Vec<String>) {
        if !options.is_empty() {
            {
                {
                    let rc = self.options.lock().unwrap();
                    rc.replace(options);
                }
                {
                    let pc = self.players.lock().unwrap();
                    let pls = pc.take();
                    pls.iter().for_each(|p| {
                        let px = p.upgrade().unwrap();
                        let mut player = px.take();
                        player.reset();
                        px.replace(player);
                    });
                    pc.replace(pls);
                }
                self.show = false;
            }
            self.updated();
        }
    }

    pub fn updated(&mut self) {
        let status: RoomStatus = self.freeze();
        let rc_players = self.players.lock().unwrap();
        let mut players = rc_players.take();
        // 各PlayerにSend
        players.iter_mut().for_each(|weak_player| {
            if let Some(player) = weak_player.upgrade() {
                let mut p = player.take();
                p.send(&status);
                player.replace(p);
            }
        });
        rc_players.replace(players);
    }

    pub fn freeze(&self) -> RoomStatus {
        let now: i64 = Utc::now().timestamp_millis();
        let px = self.players.lock().unwrap();
        let p = px.take();
        let plys: Vec<PlayerStatus> = p
            .iter()
            .flat_map(|x| {
                x.upgrade().map(|px| {
                    let p = px.take();
                    let result = PlayerStatus::new(
                        p.id().clone(),
                        p.name().clone(),
                        false,
                        p.vote().clone(),
                    );
                    px.replace(p);
                    result
                })
            })
            .collect();
        px.replace(p);

        let agenda: String = self.agenda.lock().unwrap().clone().into_inner().clone();
        let opts: Vec<String> = self.options.lock().unwrap().clone().into_inner().clone();

        RoomStatus::new(
            self.name.clone(),
            if self.show {
                "open".to_string()
            } else {
                "voting".to_string()
            },
            agenda,
            opts,
            self.player_count,
            plys,
            now,
        )
    }
}

impl Actor for Room {
    type Context = Context<Self>;
}

type RoomDic = Arc<Mutex<Rc<RefCell<HashMap<String, Rc<RefCell<Room>>>>>>>;

#[derive(Clone, Getters, new)]
pub struct RoomContainer {
    rooms: RoomDic,
}

impl RoomContainer {
    pub fn instance() -> Box<RoomContainer> {
        static mut SINGLETON: Option<Box<RoomContainer>> = None;
        static ONCE: Once = Once::new();
        unsafe {
            ONCE.call_once(|| {
                let singleton = RoomContainer {
                    rooms: Arc::new(Mutex::new(Rc::new(RefCell::new(HashMap::new())))),
                };
                SINGLETON = Some(Box::new(singleton));
            });
            SINGLETON.clone().unwrap()
        }
    }

    pub fn preserve(&mut self, room_name: Option<&str>) -> String {
        let l = self.rooms.lock().unwrap();
        let mut map = l.take();
        let mut new_id: String;
        loop {
            new_id = Id::generate("r", room_name);
            if !map.contains_key(&new_id) {
                break;
            }
        }

        let room = Room {
            id: new_id.clone(),
            name: room_name
                .map(|n| n.to_string())
                .unwrap_or_else(|| new_id.clone()),
            show: false,
            agenda: Mutex::new(RefCell::new("".to_string())),
            options: Mutex::new(RefCell::new(vec![])),
            player_count: 0,
            players: Mutex::new(RefCell::new(vec![])),
            last_touch: Utc::now().timestamp(),
        };

        map.insert(new_id.clone(), Rc::new(RefCell::new(room)));
        l.replace(map);
        new_id
    }

    pub fn exit(&mut self, room_id: &str, player_id: &str) {
        let rl = self.rooms.lock().unwrap();
        let mut map = rl.take();
        let remove_room = match map.get(room_id) {
            None => false,
            Some(rc_room) => {
                let mut room: Room = rc_room.take();
                let close_room: bool = room.exit(player_id);
                rc_room.replace(room);
                close_room
            }
        };
        if remove_room {
            map.remove(room_id);
        }
        rl.replace(map);
    }

    pub async fn start_web_socket(
        &mut self,
        req: HttpRequest,
        name: String,
        room_id: &str,
        stream: web::Payload,
    ) -> Result<HttpResponse, actix_http::Error> {
        let l1 = self.rooms.lock().unwrap();
        let x = l1.take();
        if let Some(rc_room) = x.get(room_id) {
            let r = &Rc::clone(rc_room);
            let player: Rc<RefCell<Player>> = Player::enter(r, name);
            let session: PlanningPokerSession = PlanningPokerSession::new(r, &player);

            let (addr, res) = ws::start_with_addr(session, &req, stream)?;
            let mut p = player.take();
            p.set_addr(addr);
            player.replace(p);
            l1.replace(x);
            Ok(res)
        } else {
            l1.replace(x);
            Ok(HttpResponse::NotFound().finish())
        }

        //
        // let result = match r.get(room_id) {
        //     None => Ok(HttpResponse::NotFound().finish()),
        //     Some(rc_room) => {
        //         let player: Rc<RefCell<Player>> = Player::enter(rc_room, name);
        //         let session: PlanningPokerSession = PlanningPokerSession::new(rc_room, &player);
        //
        //         let (addr, res) = ws::start_with_addr(session, &req, stream)?;
        //         let mut p = player.take();
        //         p.set_addr(addr);
        //         player.replace(p);
        //         Ok(res)
        //     }
        // };
        // l1.replace(r);
        // result
    }

    #[allow(dead_code)]
    pub fn edit_with<F>(&mut self, room_id: &str, mut callback: F) -> Result<Rc<RefCell<Room>>>
        where
            F: FnMut(Room) -> Room,
    {
        let l1 = self.rooms.lock().unwrap();
        let r = l1.take();
        let result = match r.get(room_id) {
            None => {
                l1.replace(r);
                anyhow::bail!(format!("not found room: {}", room_id))
            }
            Some(x) => {
                let z: Room = callback(x.take());
                x.replace(z);
                Ok(x.clone())
            }
        };
        l1.replace(r);
        result
    }

    pub fn status_of(&mut self, room_id: &str) -> Result<RoomStatus> {
        let r = self.rooms.lock().unwrap();
        let l = r.take();
        let result = match l.get(room_id) {
            None => {
                r.replace(l);
                anyhow::bail!(format!("not found room: {}", room_id))
            }
            Some(x) => {
                let z: Room = x.take();
                let room_status = z.freeze();
                x.replace(z);
                Ok(room_status)
            }
        };
        r.replace(l);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn freeze_test() {
        let me: Rc<RefCell<Player>> = Rc::new(RefCell::new(Player::new(
            "me".to_string(),
            "player_me".to_string(),
            0,
            None,
            Default::default(),
            Mutex::new(Rc::new(RefCell::new(None))),
        )));

        let player_1: Rc<RefCell<Player>> = Rc::new(RefCell::new(Player::new(
            "1".to_string(),
            "player_1".to_string(),
            0,
            Some("3".to_string()),
            Default::default(),
            Mutex::new(Rc::new(RefCell::new(None))),
        )));
        let player_2: Rc<RefCell<Player>> = Rc::new(RefCell::new(Player::new(
            "2".to_string(),
            "player_2".to_string(),
            0,
            None,
            Default::default(),
            Mutex::new(Rc::new(RefCell::new(None))),
        )));

        let mut container = RoomContainer {
            rooms: Arc::new(Mutex::new(Rc::new(RefCell::new(Default::default())))),
        };

        let room_id: String = container.preserve(Some("test_room"));

        let _ = container
            .edit_with(room_id.as_str(), |mut room| {
                room.attend(&me);
                room.attend(&player_1);
                room.attend(&player_2);
                room
            })
            .unwrap();

        let m: Player = me.take();
        let send_mess: RoomStatus = container
            .status_of(room_id.as_str())
            .unwrap()
            .convert_for(&m);
        me.replace(m);

        println!("{:?}", &serde_json::to_string(&send_mess));

        let _ = container
            .edit_with(room_id.as_str(), |mut room| {
                room.show = true;
                room
            })
            .unwrap();

        let m: Player = me.take();
        let send_mess: RoomStatus = container
            .status_of(room_id.as_str())
            .unwrap()
            .convert_for(&m);
        me.replace(m);
        println!("{:?}", &serde_json::to_string(&send_mess));
    }
}
