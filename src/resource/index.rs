pub struct IndexHtml;

impl IndexHtml {
    pub const CONTENT: &'static str = r#"<!DOCTYPE html>
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
            <td>table name:</td>
            <td><input type="text" class="input_name" name="room"></td>
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
"#;

    #[allow(unused)]
    pub const ETAG: &'static str = "wpARRrz5GM5hzFGBdwGK9A";
}
