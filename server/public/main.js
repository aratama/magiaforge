'use strict';

(function() {
    const socket = new WebSocket("ws://localhost:3000");

    socket.onopen = () => {
        console.log('Connected to server');

        socket.send('Hello from client');
    };
    
    socket.onmessage = (event) => {
        console.log('Message from server', event.data);

        socket.send(event.data.toUppserCase());
    };
})();