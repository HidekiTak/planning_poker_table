pub struct PlanningPokerCss;

impl PlanningPokerCss {
    pub const CONTENT: &'static str = r#"select{
  padding:8px;
  -webkit-appearance:select;
  appearance:select;
}

input[type="button"], input[type="submit"], input[type="reset"]{
  padding: 8px;
  -webkit-appearance: button;
  appearance: button;
  min-width:48px;
}

td {
  padding:4px;
}
#title{
 font-size:xx-large;
 font-weight:bold;
 padding:8px;
}
#table_name {
 white-space: nowrap;
}

.period_button{
  margin:8px;
  min-width:48px;
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

.invite_btn{
 float:right;
 margin:4px;
 display:none;
}

#invite_menu {
 position:absolute;
 padding:4px;
 background-color:white;
 padding:4px;
 clear:both;
 display:none;
}

#invite_menu>div {
 border:silver solid thin;
 padding:4px;
}

#invite_menu input{
 padding:8px;
 width:100%;
 margin:2px 0px;
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

#select select{
 padding:4px;
}
.edit_button{
 float:right;
 margin:8px;
}
.input_name{
  width:240px;
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
    pub const ETAG: &'static str = "btyTquqPsUlgIDVLX0Ua2w";
}
