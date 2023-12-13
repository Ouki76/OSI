extern crate websocket;

mod system;

fn main() {
    let client = websocket::ClientBuilder::new("ws://localhost:3030")
        .unwrap()
        .add_protocol("osi-client")
        .connect_insecure()
        .unwrap();

    let (_receiver, mut sender) = client.split().unwrap();

    let mut sys = system::System::new();

    loop {
        sender
            .send_message(&websocket::Message::text(sys.get_json_system_info()))
            .unwrap();

        std::thread::sleep(std::time::Duration::from_secs(5));
        sys.refresh();
    }
}
