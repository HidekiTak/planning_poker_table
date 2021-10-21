pub struct PlanningPokerCss;

impl PlanningPokerCss {
    pub const CONTENT: &'static str = r#"input[type="button"], input[type="submit"]{
  -webkit-appearance: button;
  padding: 8px;
}
td {
  padding:4px;
}

.period_button{
  margin:8px;
  width:48px;
  float:left;
}
.buttons {
  float:none;
  clear:both;
  padding-top:32px;
}
.buttons td {
  text-align:center;
}
.action_buttons{
  text-align: center;
}
.action_button {
  margin:8px 12px;
  width:80px;
}
.agenda_button{
  margin:4px;
  width:80px;
}
#status{
font-size:x-large;
padding-bottom:12px;
}
#contents{
 border-top:silver solid thin;
 border-left:silver solid thin;
}

table#contents > tbody > tr > td {
 border-right:silver solid thin;
 border-bottom:silver solid thin;
}

#agenda {
  font-size:x-large;
  width: 520px;
}

#sel_opt{
  width:250px;
}
#sel_val{
  width:240px;
  margin-top:8px;
}

.cards{
 padding:24px;
}
.card {
  float:left;
  margin:8px;
  width:80px;
  height:128px;
  padding:8px;
  border:solid silver 1px;
}
.card.me{
  border:solid gray 2px;
  padding:7px;
}
.card.voted{
 background-color:darkgray;
}
.card.waiting {
 background-color:lightgray;
}
.card div {
  text-align:center;
}
.card div.name{
  white-space: nowrap;
  overflow-x: hidden;
}
.card .content {
  height:100px;
  display:grid;
  align-items: flex-end;
}
.card .value {
  font-size:x-large;
}
#select {
 margin-top:8px;
}
#select div {
margin:8px;
}
#select input{
 padding:4px;
}

#select select{
 padding:4px;
}
 .edit_button{
   float:right;
   margin:8px;
 }

@media screen and (max-width:480px) {
 #agenda {
   font-size:x-large;
   width:initial;
 }
 .edit_button{
   float:right;
   margin:4px;
 }
}"#;

    #[allow(unused)]
    pub const ETAG: &'static str = "qsGGoW3KqD95tt0kPi-bdA";
}
