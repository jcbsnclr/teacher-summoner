self.addEventListener("activate", () => {
    console.log("service worker activated");
    clients.claim();
});

self.addEventListener("install", () => {
    console.log("service worker installed");
    self.skipWaiting();
});

self.addEventListener("push", async (event) => {
    try {
        console.log("push notification received");
        let data = event.data.json();
        const options = {
            body: data.body  
        };
        await self.registration.showNotification(data.title, options);
    } catch (error) {
        console.log("service worker error: " + error);
    }
});
