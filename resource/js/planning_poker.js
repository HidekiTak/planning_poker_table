var options=[]
var on_update=false

function vote(e){
  socket.send('{"type":"vote","value":"' + e.target.value+'"}')
}

function action(e){
  socket.send('{"type":"' + e.target.value+'"}')
}

function set_agenda(e){
  var agenda=document.getElementById("agenda")
  if(!agenda)return;
  agenda = agenda.value;

 if("clear_agenda"===e.target.id || !agenda){
  socket.send('{"type":"clear_agenda"}')
 }else{
  socket.send(JSON.stringify( {"type":"set_agenda","value":agenda}))
 }
}

function toCard(open,key,vote){
  var elm=document.createElement("DIV");
  elm.id=key
  elm.className=vote.me?"card me":open?"card":vote.value?"card voted":"card waiting"

  var name=document.createElement("DIV")
  name.className="name"
  name.innerText=vote.name
  elm.appendChild(name)

  var value=document.createElement("DIV")
  value.class="value"
  value.innerText=vote.vote?vote.vote:!open&&vote.me?"vote!!":"?"
  var content=document.createElement("DIV")
  content.className="content"
  content.appendChild(value)
  elm.appendChild(content)
  return elm
}

function toPeriodButton(d,v){
  var elm=document.createElement("INPUT");
  elm.type="button"
  elm.className="period_button on_open"
  elm.value=v
  elm.disabled=d
  elm.addEventListener('click',vote)
  return elm
}

function set_mess(data){
  on_update=true
  document.title="Planning Poker Table - "+data.table_name + " -"
  document.getElementById("table_name").innerText="- "+data.table_name + " -"

  var agenda=document.getElementById("agenda")
  if(agenda.value != data.agenda){
    agenda.value = data.agenda
  }

  var status=document.getElementById("status")
  if(status && status.innerHTML!==data.status){
    status.innerHTML=data.status
    var disabled=data.status=="open"?"disabled":null
    var butns=document.getElementsByClassName("on_open")
    for(var i=0;i<butns.length;i++){
        butns[i].disabled=disabled
    }
  }

  var table=document.getElementById("cards")
  var new_table=document.createElement("DIV");
  if(data.votes){
    for(var idx in data.votes){
      var vote=data.votes[idx]
      new_table.appendChild(toCard(data.status==="open", "p_"+vote.id,vote))
    }
  }
  var old_table=table.lastChild
  if(old_table){
    table.removeChild(old_table)
  }
  table.appendChild(new_table)

  table=document.getElementById("period_buttons")
  new_table=document.createElement("DIV");
  if(options.length!==data.options.length || !data.options.every(function(value, index) { return value === options[index]})){
    var vs=""
    options=data.options;
    if(options && 0<options.length){
      var d=data.status=="open"?"disabled":null
      for(var idx in options){
        var opt=options[idx]
        new_table.appendChild(toPeriodButton(d,opt))
        if(0<vs.length){
         vs=vs+","
        }
        vs=vs+opt
      }
    }
    sel_set(vs)
    var old_table=table.firstChild
    if(old_table){
      table.removeChild(old_table)
    }
    table.appendChild(new_table)
  }
  on_update=false
  return
}

function opt_index(v,sel){
  for(var i = 0; i < sel.options.length-1; i++) {
    if(sel.options[i].value===v){
     return i
    }
  }
  return sel.options.length-1
}

function sel_set(v){
  var sel=document.getElementById("sel_opt")
  var selected=opt_index(v,sel)
  var free=selected===sel.options.length-1
  var changed=sel.selectedIndex!==selected
  var txt=document.getElementById("sel_val")
  if(changed){
    sel.selectedIndex=selected
    txt.style.display=free?"initial":"none"
  }
  if(changed||free){
    txt.value=v
  }
}

function sel_changed(ev){
  var txt=document.getElementById("sel_val")
  if(ev.target.value){
    txt.style.display="none"
    txt.value=ev.target.value
  }else{
    txt.style.display="initial"
    var vs=options?options.join(","):""
    var sel=document.getElementById("sel_opt")
    txt.value=(opt_index(vs,sel)<sel.options.length-1)?"":vs
    if(!on_update){
     txt.focus()
    }
  }
  var btn=document.getElementById("sel_btn")
  btn.disabled=ev.target.value.trim()?"":"disabled";
}

function set_button_event(d,name,fnc){
  var elms=d.getElementsByClassName(name)
  elms=Array.prototype.filter.call(elms, function(e){
    return e.nodeName === 'INPUT' && e.type=="button"
  });
  for (let i=0; i<elms.length; i++) {
    elms[i].addEventListener('click',fnc)
  }
}

function loadQRGen(callback){
  var ga=document.createElement('script')
  ga.type='text/javascript'
  ga.src="https://cdn.jsdelivr.net/npm/qrcode@latest/build/qrcode.min.js"
  ga.addEventListener('load',callback)
  var s = document.getElementsByTagName('script')[0]
  s.parentNode.insertBefore( ga, s );
}

function showCanvas(canvas){
  hide_invite()
  var qr=document.getElementById("qr")
  qr.style="border:silver solid thin;padding:8px;display:initial;position: fixed;top: 50%;left: 50%;transform: translate(-50%, -50%);z-index:999"
}

function hide_qr(){
  var qr=document.getElementById("qr")
  qr.style.display="none"
}
function showQR(){
  if(typeof QRCode ==="undefined"){
    loadQRGen(showQR)
  } else {
    const canvas = document.getElementById("qr_img")
    QRCode.toCanvas(canvas, location.href, {}, err => {
      if(err) {alert(err) }else{ showCanvas(canvas)}
    })
  }
}

function hide_invite(){
  var invite_menu=document.getElementById("invite_menu")
  invite_menu.style.display="none"
}

function toggle_invite(){
 var btn=document.getElementById("invite")
  var invite_menu=document.getElementById("invite_menu")
  var hidden=invite_menu.style.display==="none"
  if(!hidden){
    invite_menu.style.display="none"
    return
  }
  var pos=getElementTopRight(btn)
  invite_menu.style.display="initial"
  invite_menu.style.top=pos.top+'px'
  invite_menu.style.left=(pos.left-invite_menu.offsetWidth)+'px'
}
function getElementTopRight(elm) {
    var top = elm.offsetHeight+2;
    var left = elm.offsetWidth/2;
    while(elm.tagName != "BODY") {
        top += elm.offsetTop;
        left += elm.offsetLeft;
        elm = elm.offsetParent;
    }
    return { top: top, left: left };
}

function copy_clipboard(){
  if(navigator.clipboard){
    navigator.clipboard.writeText( location.href);
  }
  hide_invite()
}

function copy_command(){
  if(document.execCommand){
    var textarea = document.createElement('textarea');
    textarea.style.position ='absolute';
    textarea.style.opacity = 0;
    textarea.style.pointerEvents = 'none';
    textarea.value = location.href;
    document.body.appendChild(textarea);
    textarea.focus();
    textarea.setSelectionRange(0, 999999);
    document.execCommand('copy');
    textarea.parentNode.removeChild(textarea);
  }
  hide_invite()
}
function copy_func(){
  if(navigator.clipboard){
    return copy_clipboard
  }
  if(document.execCommand){
    return copy_command
  }
  return null
}

(function(){
  var fn=function(name,fnc){
    var elms=document.getElementsByClassName(name)
    elms=Array.prototype.filter.call(elms, function(e){
      return e.nodeName === 'INPUT' && e.type=="button"
    });
    for (let i=0; i<elms.length; i++) {
      elms[i].addEventListener('click',fnc)
    }
  }
  fn("action_button",action)
  fn("agenda_button",set_agenda)

  var cf=copy_func()
  if(!cf){
    var qr_url=document.getElementById("qr_url_1")
    qr_url.style.display="initial"
    qr_url.addEventListener('click',showQR)
  }else{
    var invite=document.getElementById("invite")
    invite.style.display="initial"
    invite.addEventListener('click',toggle_invite)

    var copy_url=document.getElementById("copy_url")
    copy_url.addEventListener('click',cf)

    var qr_url=document.getElementById("qr_url_2")
    if(qr_url){
     qr_url.addEventListener('click',showQR)
    }
  }

  var qr_img=document.getElementById("qr_img")
  if(qr_img){
    qr_img.addEventListener('click',hide_qr)
  }

  var txt=document.getElementById("sel_val")
  txt.addEventListener('keyup',function(ev){
    var btn=document.getElementById("sel_btn")
    var rep=ev.target.value.replace("、",",")
    if(rep != ev.target.value){
      ev.target.value = rep
    }else{
    btn.disabled=ev.target.value.trim()?"":"disabled";
    }
  })
  var sel=document.getElementById("sel_opt")
  sel.addEventListener('change',sel_changed)
  sel_changed({"target":sel})
  var btn=document.getElementById("sel_btn")
  btn.addEventListener('click',function(ev){
    var v=document.getElementById("sel_val")
    socket.send(JSON.stringify({"type":"options","value":v.value}))
  })
})()

const socket = new WebSocket('ws://' + window.location.host + window.location.pathname + "/ws");
socket.addEventListener('message', function (event) {
  if(!event.data) return
  var data = JSON.parse(event.data);
  setTimeout(() => { set_mess(data)}, 0)
});
socket.addEventListener('close', function (event) {
  if(location.href.startsWith("http")){
    alert("This connection has been closed");
    location.href="/";
  }
});
