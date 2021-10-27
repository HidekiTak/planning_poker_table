use actix::Addr;
use chrono::Utc;
use derive_getters::Getters;
use derive_new::new;
use std::cell::RefCell;
use std::rc::{Rc, Weak};
use std::sync::Mutex;

use crate::entity::id::Id;
use crate::entity::{Table, TableStatus};
use crate::web_socket_session::PlanningPokerSession;

#[derive(Debug, Getters, new, Default)]
pub struct Player {
    id: String,
    name: String,
    vote_at: i64,
    vote: Option<String>,
    table: Weak<RefCell<Table>>,
    addr: Mutex<Rc<RefCell<Option<Addr<PlanningPokerSession>>>>>,
}

impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Player {
    pub const COOKIE_NAME: &'static str = "name";

    pub fn enter(table: &Rc<RefCell<Table>>, name: String) -> (bool, Rc<RefCell<Player>>) {
        let id = Id::generate("p", Some(name.as_str()));
        let result = Rc::new(RefCell::new(Player {
            id,
            name,
            vote_at: 0,
            vote: None,
            table: Rc::downgrade(table),
            addr: Mutex::new(Rc::new(RefCell::new(None))),
        }));
        let mut r = table.take();
        let statistics_changed: bool = r.attend(&result);
        table.replace(r);
        (statistics_changed, result)
    }

    pub fn set_addr(&mut self, addr: Addr<PlanningPokerSession>) {
        let rc = self.addr.lock().unwrap();
        rc.replace(Some(addr));
    }

    pub fn send(&mut self, status: &TableStatus) {
        let mutex = self.addr.lock().unwrap();
        let s = mutex.take();
        if let Some(a) = s {
            let mess: TableStatus = status.convert_for(self);
            a.do_send(mess);
            mutex.replace(Some(a));
        } else {
            mutex.replace(None);
        }
    }

    pub fn voting(&mut self, value: Option<&str>) {
        self.vote = value.map(|v| v.to_string());
        self.vote_at = Utc::now().timestamp_millis();
    }

    pub fn reset(&mut self) {
        self.vote_at = 0;
        self.vote = None;
    }
}
