const WebSocket = require('ws');
const port = process.env.PORT || 3000;
const server = new WebSocket.Server({ port });
console.log(`Server started on port ${port}`);

// https://www.npmjs.com/package/ws
server.on('connection', (ws, req) => {

  const ip = req.socket.remoteAddress;

  console.log('Client connected, ip address:', ip);

  ws.on('error', (e) => { 
    console.error(e); 
  });

  ws.on('message', (message, isBinary) => {
    console.log('received: %s', message);
    
    const data = message.toString().toUpperCase();
    // ws.send(`Hello, you sent -> ${message.toString().toUpperCase()}`);

    server.clients.forEach(function each(client) {
      if (client.readyState === WebSocket.OPEN) {
        client.send(data, { binary: isBinary });
      }
    });
  });
});