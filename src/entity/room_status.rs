use crate::entity::Player;
use actix::Message;
use derive_getters::Getters;
use derive_new::new;
use serde::Serialize;

#[derive(Debug, Serialize, new, Clone, PartialEq, Getters)]
pub struct PlayerStatus {
    id: String,
    name: String,
    me: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    vote: Option<String>,
}

#[derive(Debug, Serialize, new, Clone, PartialEq, Getters, Message)]
#[rtype(result = "()")]
pub struct RoomStatus {
    room_name: String,
    status: String,
    agenda: String,
    options: Vec<String>,
    player_count: usize,
    votes: Vec<PlayerStatus>,
    at: i64,
}

impl RoomStatus {
    pub fn convert_for(&self, player: &Player) -> RoomStatus {
        let open = self.status == "open";
        let plyrs: Vec<PlayerStatus> = self
            .votes
            .iter()
            .map(|p| {
                if player.id() == p.id() {
                    PlayerStatus {
                        id: p.id().clone(),
                        name: p.name().clone(),
                        me: true,
                        vote: p.vote().clone(),
                    }
                } else {
                    PlayerStatus {
                        id: p.id().clone(),
                        name: p.name().clone(),
                        me: false,
                        vote: if open {
                            p.vote().clone()
                        } else if p.vote().is_none() {
                            None
                        } else {
                            Some("voted".to_string())
                        },
                    }
                }
            })
            .collect();

        RoomStatus::new(
            self.room_name.clone(),
            self.status.clone(),
            self.agenda.clone(),
            self.options.clone(),
            self.player_count,
            plyrs,
            self.at,
        )
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;
    use std::sync::Mutex;
    // use serde_json::Value;
    use super::*;

    #[test]
    fn convert_test() {
        let me: Player = Player::new(
            "me".to_string(),
            "自分".to_string(),
            0,
            None,
            Default::default(),
            Mutex::new(Rc::new(RefCell::new(None))),
        );
        let mut status = RoomStatus {
            room_name: "Room1".to_string(),
            status: "open".to_string(),
            agenda: "今日の昼食".to_string(),
            options: vec!["0".to_string(), "1".to_string(), "2".to_string()],
            player_count: 4,
            votes: vec![
                PlayerStatus::new("me".to_string(), "自分".to_string(), true, None),
                PlayerStatus::new(
                    "me".to_string(),
                    "自分".to_string(),
                    true,
                    Some("2".to_string()),
                ),
                PlayerStatus::new("p1".to_string(), "Player1".to_string(), false, None),
                PlayerStatus::new(
                    "p2".to_string(),
                    "Player2".to_string(),
                    false,
                    Some("2".to_string()),
                ),
            ],
            at: 10,
        };
        let result = status.convert_for(&me);
        assert_eq!(&status, &result);

        status.status = "voting".to_string();
        let result = status.convert_for(&me);
        assert_eq!(
            &RoomStatus {
                room_name: "Room1".to_string(),
                status: "voting".to_string(),
                agenda: "今日の昼食".to_string(),
                options: vec!["0".to_string(), "1".to_string(), "2".to_string()],
                player_count: 4,
                votes: vec![
                    PlayerStatus::new("me".to_string(), "自分".to_string(), true, None),
                    PlayerStatus::new(
                        "me".to_string(),
                        "自分".to_string(),
                        true,
                        Some("2".to_string()),
                    ),
                    PlayerStatus::new("p1".to_string(), "Player1".to_string(), false, None),
                    PlayerStatus::new(
                        "p2".to_string(),
                        "Player2".to_string(),
                        false,
                        Some("voted".to_string()),
                    ),
                ],
                at: 10,
            },
            &result
        );
    }
}
