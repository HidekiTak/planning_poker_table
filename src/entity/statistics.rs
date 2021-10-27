use crate::entity::Table;
use chrono::Utc;
use derive_getters::Getters;
use derive_new::new;
use once_cell::sync::Lazy;
use std::cell::RefCell;
use std::sync::{Arc, Mutex};

#[derive(Clone, Debug, Default, Getters, new)]
pub struct Statistics {
    opened_tables: usize,
    opened_players: usize,
    closed_tables: usize,
    closed_players: usize,
}

pub struct StatisticsHolder {
    lock: Mutex<i32>,
    data: Arc<RefCell<Statistics>>,
}

impl StatisticsHolder {
    pub fn new() -> Self {
        StatisticsHolder {
            lock: Mutex::new(0),
            data: Arc::new(RefCell::new(Default::default())),
        }
    }
}

impl Statistics {
    fn instance() -> Arc<StatisticsHolder> {
        static mut SINGLETON: Option<Arc<StatisticsHolder>> = None;
        static STATISTICS_LOCK: Lazy<Mutex<i32>> = Lazy::new(|| Mutex::new(0));
        unsafe {
            if let Some(bx) = SINGLETON.clone() {
                return bx;
            }
            let _l = STATISTICS_LOCK.lock().unwrap();
            if let Some(bx) = SINGLETON.clone() {
                return bx;
            }
            SINGLETON = Some(Arc::new(StatisticsHolder::new()));
            SINGLETON.clone().unwrap()
        }
    }

    pub fn open_table_changed(opened_tables: usize, opened_players: usize) -> Statistics {
        let result: Statistics;
        let arc = Statistics::instance();

        let _l = arc.lock.lock().unwrap();
        {
            let mut st: Statistics = arc.data.take();
            st.opened_tables = opened_tables;
            st.opened_players = opened_players;
            result = st.clone();
            arc.data.replace(st);
        }
        result
    }

    pub fn table_closed(table: &Table) -> Statistics {
        let result: Statistics;
        let arc = Statistics::instance();
        let _l = arc.lock.lock().unwrap();
        {
            let mut st: Statistics = arc.data.take();
            st.opened_tables = st.opened_tables.checked_sub(1).unwrap_or_default();
            st.opened_players = st
                .opened_players
                .checked_sub(*table.max_players())
                .unwrap_or_default();
            st.closed_tables = st.closed_tables.checked_add(1).unwrap_or(usize::MAX);
            st.closed_players = st
                .closed_players
                .checked_add(*table.max_players())
                .unwrap_or(usize::MAX);
            result = st.clone();
            arc.data.replace(st);
        }
        println!(
            r#"{{"comand":"statics","open_tables":"{}","open_players":{},"closed_tables":"{}","closed_players":{},"at":{}}}"#,
            result.opened_tables,
            result.opened_players,
            result.closed_tables,
            result.closed_players,
            Utc::now().timestamp_millis(),
        );
        result
    }
}
