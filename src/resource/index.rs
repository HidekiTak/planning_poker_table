use crate::entity::Statistics;
use derive_getters::Getters;
use sha2::{Digest, Sha256};
use std::cell::RefCell;
use std::sync::{Arc, Once};

#[derive(Getters, Clone)]
pub struct IndexHtml {
    opened_tables: usize,
    opened_players: usize,
    closed_tables: usize,
    closed_players: usize,
    content: String,
    etag: String,
}

impl IndexHtml {
    pub fn instance() -> IndexHtml {
        let i = IndexHtml::instance2();
        i.as_ref().clone().into_inner()
    }

    fn instance2() -> Arc<RefCell<IndexHtml>> {
        static mut SINGLETON: Option<Arc<RefCell<IndexHtml>>> = None;
        static ONCE: Once = Once::new();
        unsafe {
            ONCE.call_once(|| {
                SINGLETON = Some(Arc::new(RefCell::new(IndexHtml::format(
                    Statistics::default(),
                ))))
            });
            SINGLETON.clone().unwrap()
        }
    }

    pub fn update_count(statistics: Statistics) {
        let index_html = IndexHtml::format(statistics);
        let instance = IndexHtml::instance2();
        instance.replace(index_html);
    }

    fn format(statistics: Statistics) -> IndexHtml {
        let content: String = format!(
            r#"<!DOCTYPE html>
<html lang="ja">
<head>
    <meta charset="utf-8"/>
    <meta content="text/html; charset=UTF-8" http-equiv="Content-Type"/>
    <meta name="viewport" content="width=device-width,initial-scale=1">
    <title>Planning Poker Table</title>
    <link rel="stylesheet" href="/css/planning_poker.css">
</head>
<body>
<h1>Planning Poker Table</h1>
<form method="post">
    <table>
        <tr>
            <td colspan="2" style="text-align:right;font-size:smaller">
                {opened_tables} tables/ {opened_players} players now on this server
            </td>
        </tr>
        <tr>
            <td>table name:</td>
            <td><input type="text" class="input_name" name="table"></td>
        </tr>
        <tr>
            <td>your name:</td>
            <td><input type="text" class="input_name" name="name"></td>
        </tr>
        <tr>
            <td>options:</td>
            <td>
                <select id="sel_opt" name="sel_opt">
                    <option value="0,1,2,3,5,8,13,21,∞,?">0,1,2,3,5,8,13,21,∞,?</option>
                    <option value="0,0.5,1,2,3,5,8,13,20,40,∞,?">0,0.5,1,2,3,5,8,13,20,40,∞,?</option>
                    <option value="ぐー,ちょき,ぱー">ぐー,ちょき,ぱー</option>
                    <option value="">手入力</option>
                </select>
                <br/><input type="text" id="sel_val" name="sel_val" placeholder="カンマ区切りで選択項目を指定">
            </td>
        </tr>
        <tr>
            <td colspan="2" style="text-align:center">
                <input type="submit" value="開始" style="padding:4px 12px">
            </td>
        </tr>
    </table>
</form>
<script src="/js/index.js"></script>
</body>
</html>
"#,
            opened_tables = statistics.opened_tables(),
            opened_players = statistics.opened_players(),
        );

        let etag: String = base64_url::encode(Sha256::digest(content.as_bytes()).as_slice());
        IndexHtml {
            opened_tables: *statistics.opened_tables(),
            opened_players: *statistics.opened_players(),
            closed_tables: *statistics.closed_tables(),
            closed_players: *statistics.closed_players(),
            content,
            etag,
        }
    }
}
