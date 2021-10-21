
function opt_changed(ev){
 var sel=ev.target
 sel_val.style.display=(sel.selectedIndex>=sel.options.length-1)?"initial":"none"
}

(function(){
 document.getElementById("sel_val").style.display="none"
 document.getElementById("sel_opt").addEventListener('change',opt_changed)
})()
