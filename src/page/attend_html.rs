pub struct AttendHtml;

/// ルームにPlayerが参加
impl AttendHtml {
    pub fn content(table_name: &str) -> String {
        format!(
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
            <td>table name:</td>
            <td>{table_name}</td>
        </tr>
        <tr>
            <td>your name:</td>
            <td><input type="text" name="name"></td>
        </tr>
        <tr>
            <td colspan="2" style="text-align:center"><input type="submit" value="参加" style="padding:4px 12px"></td>
        </tr>
    </table>
</form>
</body>
</html>
"#,
            table_name = table_name
        )
    }
}
