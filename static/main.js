"use strict";

let class_id = parseInt(document.currentScript.getAttribute("classid"));
console.log("class id = " + class_id);

function update_list(dismissed) {
  // url to retrieve
  let path = "/class/" + class_id + "/teacher?raw=true";
  
  // if dismissed isn't undefined, then we need to dismiss a ticket too
  if (dismissed !== undefined)
    path += `&dismissed=${dismissed}`;

  // create XHTTP request
  const xhttp = new XMLHttpRequest();

  // callback when request ready
  xhttp.onreadystatechange = function() {
    // if request is ready (readyState == 4) and status is OK (200)
    if (this.readyState == 4 && this.status == 200) {
      // replace list on page with list from server
     document.getElementById("ticket-list").innerHTML = this.responseText;
    }
  };

  // send the request
  xhttp.open("GET", path);
  xhttp.send();
}

// refresh list every 5 seconds
function refresh() {
  update_list();
  setTimeout(() => { refresh() }, 2500);
}

refresh();
