import * as ws from 'ws';

async function loop() {
    const wss = new ws.WebSocketServer({ port: 3030 });

    wss.on('connection', (client) => {

        switch (client.protocol.toString()) {
            case "osi-client": {
                client.on('message', (data) => {
                    console.log(data.toString())
                });
            }
            default: client.close();
        }

        client.on('error', console.error);
    });
}

loop().then(() => {
    console.log("Server started")
}).catch((e) => {
    console.error(e)
})