import * as ws from 'ws';

async function loop() {
    const wss = new ws.WebSocketServer({ port: 3030 });

    wss.on('connection', (client) => {
        client.on('error', console.error);

        client.on('message', (data) => {
            console.log(data.toString())
        });
    });
}

loop().then(() => {
    console.log("Server started")
}).catch((e) => {
    console.error(e)
})