use actix::*;
use actix_web_actors::ws;
use chrono::Utc;
use serde_json::Value;
use std::cell::RefCell;
use std::ops::Sub;
use std::rc::{Rc, Weak};
use std::sync::Mutex;
use std::time::{Duration, Instant};

use crate::entity::{Player, Table, TableContainer, TableStatus};

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(10);

const CLIENT_TIMEOUT: Duration = Duration::from_secs(20);

#[derive(Debug)]
pub struct PlanningPokerSession {
    table_id: String,
    player_id: String,
    table: Weak<RefCell<Table>>,
    player: Mutex<Rc<RefCell<Player>>>,
    addr: Rc<RefCell<Option<Addr<PlanningPokerSession>>>>,
    /// heartbeat
    hb: Instant,
    update: Instant,
    send: Instant,
}

impl Drop for PlanningPokerSession {
    fn drop(&mut self) {
        TableContainer::instance().exit(&self.table_id, &self.player_id);
    }
}

impl Actor for PlanningPokerSession {
    type Context = ws::WebsocketContext<Self>;

    /// Method is called on actor start. We start the heartbeat process here.
    fn started(&mut self, ctx: &mut Self::Context) {
        let r = self.table.upgrade();
        match r {
            None => {}
            Some(rc_table) => {
                let mut table = rc_table.take();
                table.updated();
                rc_table.replace(table);
            }
        }
        self.heartbeat(ctx);
    }
}

impl actix::prelude::Handler<TableStatus> for PlanningPokerSession {
    type Result = ();

    fn handle(&mut self, msg: TableStatus, ctx: &mut Self::Context) -> Self::Result {
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
                if let Ok(mess) = serde_json::from_str(text.to_string().as_str()) {
                    let _: &Value = &mess;
                    if let Some(typ) = mess.get("type").and_then(|v| v.as_str()) {
                        match typ {
                            "open" => {
                                if let Some(r) = self.table.upgrade() {
                                    let mut table = r.take();
                                    table.open();
                                    r.replace(table);
                                }
                            }
                            "reset" => {
                                if let Some(r) = self.table.upgrade() {
                                    let mut table = r.take();
                                    table.reset();
                                    r.replace(table);
                                }
                            }
                            "clear_agenda" => {
                                if let Some(r) = self.table.upgrade() {
                                    let mut table = r.take();
                                    table.set_agenda("");
                                    r.replace(table);
                                }
                            }
                            "set_agenda" => {
                                if let Some(r) = self.table.upgrade() {
                                    let agenda =
                                        mess.get("value").and_then(|v| v.as_str()).unwrap_or("");
                                    let mut table = r.take();
                                    table.set_agenda(agenda);
                                    r.replace(table);
                                }
                            }
                            "options" => {
                                if let Some(options) =
                                    mess.get("value").and_then(|v| v.as_str()).map(|s| {
                                        s.split(',')
                                            .map(|s| s.trim())
                                            .filter(|s| !s.is_empty())
                                            .map(|s| s.to_string())
                                            .collect()
                                    })
                                {
                                    if let Some(r) = self.table.upgrade() {
                                        let mut table = r.take();
                                        table.set_options(options);
                                        r.replace(table);
                                    }
                                }
                            }
                            "vote" => {
                                let s: Option<&str> = mess.get("value").and_then(|v| v.as_str());
                                if let Some(r) = self.table.upgrade() {
                                    let mut table = r.take();
                                    if !table.show() {
                                        let l = self.player.lock().unwrap();
                                        let mut player = l.take();
                                        player.voting(s);
                                        l.replace(player);
                                        table.updated();
                                    }
                                    r.replace(table);
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
    pub fn new(table: &Rc<RefCell<Table>>, player: &Rc<RefCell<Player>>) -> Self {
        let pc = Rc::clone(player);
        let px = pc.take();
        let player_id: String = px.id().clone();
        pc.replace(px);
        let b = table.take();
        let result = Self {
            table_id: b.id().clone(),
            player_id,
            table: Rc::downgrade(table),
            player: Mutex::new(pc),
            addr: Rc::new(RefCell::new(None)),
            hb: Instant::now(),
            update: Instant::now(),
            send: Instant::now().sub(Duration::new(1000, 0)),
        };
        println!(
            r#"{{"command":"attend","table":"{}","players":{},"at":{}}}"#,
            &b.id(),
            &b.player_count(),
            Utc::now().timestamp_millis()
        );
        table.replace(b);
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
