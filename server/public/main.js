'use strict';

(function() {
    const socket = new WebSocket("https://magia-server-38847751193.asia-northeast1.run.app");

    socket.onopen = () => {
        console.log('Connected to server');

        socket.send('Hello from client');
    };
    
    socket.onmessage = (event) => {
        console.log('Message from server: ', event.data);
    };
})();