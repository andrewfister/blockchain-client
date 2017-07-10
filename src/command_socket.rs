extern crate zmq;

pub struct CommandSocket {
    pub ctx: zmq::Context,
    pub server_location: String,
}

impl CommandSocket {
    pub fn send_command(&self, command: String) {
        let socket = self.ctx.socket(zmq::REQ).unwrap();
        let connect_status = socket.connect(format!("ipc://{}/rep-server",
                                                    self.server_location).as_str());

        match connect_status {
            Ok(value) => {
                println!("Connected to blockchain server! {:?}", value);
                socket.send_str(command.as_str(), 0).unwrap();

                let msg = socket.recv_string(0);
                println!("{:?}", msg);
            },
            Err(e) => println!("Failed to connect! Received error: {}", e),
        }
    }
}