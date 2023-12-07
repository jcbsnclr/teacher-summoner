"use strict";

// extract `class_id` from script tag
let class_id = parseInt(document.currentScript.getAttribute("classid"));
console.log("class id = " + class_id);

// updates the list of tickets, optionally dismissing a given ticket
function update_list(dismissed) {
    // if dismissed isn't undefined, then we need to dismiss a ticket too
    let dismissal = dismissed !== undefined ? `&dismissed=${dismissed}` : "";
    // url to retrieve
    let path = `/class/${class_id}/teacher?raw=true${dismissal}`;
    
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

// refresh list every 2.5 seconds
function refresh() {
    update_list();
    setTimeout(() => { refresh() }, 2500);
}

refresh();

// code for subscribing for push notifications

// kindly lifted from https://github.com/leotaku/web-push-native/blob/master/example/assets/index.js
function base64UrlDecode(base64String) {
  var padding = "=".repeat((4 - (base64String.length % 4)) % 4);
  var base64 = (base64String + padding).replace(/\-/g, "+").replace(/_/g, "/");

  var rawData = window.atob(base64);
  var outputArray = new Uint8Array(rawData.length);

  for (var i = 0; i < rawData.length; ++i) {
    outputArray[i] = rawData.charCodeAt(i);
  }
  return outputArray;
}

async function fetchVapidKey() {
  return fetch("/api/vapid.json").then((resp) => resp.json());
}

async function subToPush(keys) {
    const registration = await navigator.serviceWorker.register("/static/service-worker.js");
    await registration.update();

    await registration.pushManager.getSubscription()
        .then((sub) => sub.unsubscribe());

    const pushSub = await registration.pushManager.subscribe({
        userVisibleOnly: true,
        applicationServerKey: keys.vapid_key
    });
    console.log("received subscription: ", pushSub);

    return pushSub;
}

function requestPermission() {
    return new Promise((resolve, reject) => {
        const result = Notification.requestPermission((result) => {
            resolve(result);
        });

        if (result)
            result.then(resolve, reject);
    })
    .then((result) => {
        if (result != "granted") {
            throw new Error("permission not granted");
        }
    });
}

function browserSupported() {
    if (!('serviceWorker' in navigator)) {
        console.log("service workers unsupported");
        return false;
    }
    if (!('PushManager' in window)) {
        console.log("push API unsupported");
        return false;
    }

    return true;
}

async function subscribe() {
    if (browserSupported()) {
        // await registerNotifications();

        let keys = await fetchVapidKey();
        await requestPermission();

        let sub = await subToPush(keys);

        await fetch(`/class/${class_id}/register`, {
            method: "POST",
            headers: {
                "Content-Type": "application/json"
            },
            body: JSON.stringify(sub)
        });
    }
}

fetchVapidKey().then((key) => {
    console.log(key.vapid_key);
});