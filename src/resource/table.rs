pub struct TableHtml;

impl TableHtml {
    pub const CONTENT: &'static str = r#"<!DOCTYPE html>
<html lang="ja">
<head>
    <meta charset="utf-8"/>
    <meta content="text/html; charset=UTF-8" http-equiv="Content-Type"/>
    <meta name="viewport" content="width=device-width,initial-scale=1">
    <title>Planning Poker Table</title>
    <link rel="stylesheet" href="./css/planning_poker.css">
</head>
<body>
<input type="button" class="invite_btn" id="qr_url_1" value="QRCode">
<input type="button" class="invite_btn" id="invite" value="招待">
<div id="invite_menu">
    <div>
        <div>
            <input type="button" id="copy_url" value="リンクコピー">
            <input type="button" id="qr_url_2" value="QRCode">
        </div>
    </div>
</div>
<div id="qr" style="display:none">
    <canvas id="qr_img"></canvas>
</div>
<div id="title">Planning Poker Table <span id="table_name"></span></div>
<table id="contents">
    <tr>
        <td>
            <div style="display:inline-block;margin:12px"><input type="text" id="agenda" placeholder="議題"></div>
            <div class="edit_button">
                <input type="button" id="set_agenda" class="agenda_button" value="議題設定">
                <input type="button" id="clear_agenda" class="agenda_button" value="Clear"></div>
        </td>
    </tr>
    <tr>
        <td id="cards" class="cards">
            <div id="status"></div>
        </td>
    </tr>
    <tr>
        <td>
            <table>
                <tr>
                    <td id="period_buttons" class="period_buttons buttons"></td>
                </tr>
                <tr>
                    <td class="action_buttons">
                        <input type="button" class="action_button on_open" value="open">
                        <input type="button" class="action_button" value="reset">
                    </td>
                </tr>
            </table>
        </td>
    </tr>
    <tr>
        <td id="select">
            <div style="display:inline-block">選択項目</div>
            <div style="display:inline-block">
                <select id="sel_opt">
                    <option value="0,1,2,3,5,8,13,21,∞,?">0,1,2,3,5,8,13,21,∞,?</option>
                    <option value="0,0.5,1,2,3,5,8,13,20,40,∞,?">0,0.5,1,2,3,5,8,13,20,40,∞,?</option>
                    <option value="ぐー,ちょき,ぱー">ぐー,ちょき,ぱー</option>
                    <option value="">手入力</option>
                </select>
                <br/><input type="text" id="sel_val" name="values" placeholder="カンマ区切りで選択項目を指定">
            </div>
            <div class="edit_button"><input type="submit" id="sel_btn" value="設定" name="options"></div>
        </td>
    </tr>
</table>

<script src="./js/planning_poker.js"></script>
</body>
</html>"#;

    #[allow(unused)]
    pub const ETAG: &'static str = "Bvui-YILFqRPczviO3JkTw";
}
