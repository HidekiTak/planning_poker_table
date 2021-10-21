use actix::*;
use actix_web_actors::ws;
use serde_json::Value;
use std::cell::RefCell;
use std::ops::Sub;
use std::rc::{Rc, Weak};
use std::sync::Mutex;
use std::time::{Duration, Instant};

use crate::entity::{Player, Room, RoomContainer, RoomStatus};

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(10);

const CLIENT_TIMEOUT: Duration = Duration::from_secs(20);

#[derive(Debug)]
pub struct PlanningPokerSession {
    room_id: String,
    player_id: String,
    room: Weak<RefCell<Room>>,
    player: Mutex<Rc<RefCell<Player>>>,
    addr: Rc<RefCell<Option<Addr<PlanningPokerSession>>>>,
    /// heartbeat
    hb: Instant,
    update: Instant,
    send: Instant,
}

impl Drop for PlanningPokerSession {
    fn drop(&mut self) {
        RoomContainer::instance().exit(&self.room_id, &self.player_id);
        //
        // let mutex = self.player.lock().unwrap();
        // let mut player = mutex.take();
        // player.exit();
        // mutex.replace(player);
    }
}

impl Actor for PlanningPokerSession {
    type Context = ws::WebsocketContext<Self>;

    /// Method is called on actor start. We start the heartbeat process here.
    fn started(&mut self, ctx: &mut Self::Context) {
        let r = self.room.upgrade();
        match r {
            None => {}
            Some(rc_room) => {
                let mut room = rc_room.take();
                room.updated();
                rc_room.replace(room);
            }
        }
        self.heartbeat(ctx);
    }
}

impl actix::prelude::Handler<RoomStatus> for PlanningPokerSession {
    type Result = ();

    fn handle(&mut self, msg: RoomStatus, ctx: &mut Self::Context) -> Self::Result {
        if let Ok(json) = serde_json::to_string(&msg) {
            ctx.text(json)
        }
    }
}

/// Handler for `ws::Message`
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for PlanningPokerSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        // process WebSocket messages
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
                self.hb = Instant::now();
            }
            Ok(ws::Message::Text(text)) => {
                if let Ok(mess) = serde_json::from_str(text.as_str()) {
                    let _: &Value = &mess;
                    if let Some(typ) = mess.get("type").map(|v| v.as_str()).flatten() {
                        match typ {
                            "open" => {
                                if let Some(r) = self.room.upgrade() {
                                    let mut room = r.take();
                                    room.open();
                                    r.replace(room);
                                }
                            }
                            "reset" => {
                                if let Some(r) = self.room.upgrade() {
                                    let mut room = r.take();
                                    room.reset();
                                    r.replace(room);
                                }
                            }
                            "clear_agenda" => {
                                if let Some(r) = self.room.upgrade() {
                                    let mut room = r.take();
                                    room.set_agenda("");
                                    r.replace(room);
                                }
                            }
                            "set_agenda" => {
                                if let Some(r) = self.room.upgrade() {
                                    let agenda = mess
                                        .get("value")
                                        .map(|v| v.as_str())
                                        .flatten()
                                        .unwrap_or("");
                                    let mut room = r.take();
                                    room.set_agenda(agenda);
                                    r.replace(room);
                                }
                            }
                            "options" => {
                                if let Some(options) =
                                    mess.get("value").map(|v| v.as_str()).flatten().map(|s| {
                                        s.split(',')
                                            .map(|s| s.trim())
                                            .filter(|s| !s.is_empty())
                                            .map(|s| s.to_string())
                                            .collect()
                                    })
                                {
                                    if let Some(r) = self.room.upgrade() {
                                        let mut room = r.take();
                                        room.set_options(options);
                                        r.replace(room);
                                    }
                                }
                            }
                            "vote" => {
                                let s: Option<&str> =
                                    mess.get("value").map(|v| v.as_str()).flatten();
                                if let Some(r) = self.room.upgrade() {
                                    let mut room = r.take();
                                    if !room.show() {
                                        let l = self.player.lock().unwrap();
                                        let mut player = l.take();
                                        player.voting(s);
                                        l.replace(player);
                                        room.updated();
                                    }
                                    r.replace(room);
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
            Ok(ws::Message::Binary(_)) => {}
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => ctx.stop(),
        }
    }
}

impl PlanningPokerSession {
    pub fn new(room: &Rc<RefCell<Room>>, player: &Rc<RefCell<Player>>) -> Self {
        let pc = Rc::clone(player);
        let px = pc.take();
        let player_id: String = px.id().clone();
        pc.replace(px);
        let b = room.take();
        let result = Self {
            room_id: b.id().clone(),
            player_id,
            room: Rc::downgrade(room),
            player: Mutex::new(pc),
            addr: Rc::new(RefCell::new(None)),
            hb: Instant::now(),
            update: Instant::now(),
            send: Instant::now().sub(Duration::new(1000, 0)),
        };
        room.replace(b);
        result
    }

    fn heartbeat(&self, ctx: &mut <Self as Actor>::Context) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                ctx.stop();
                return;
            }
            ctx.ping(b"");
        });
    }
}
