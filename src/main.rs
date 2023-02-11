use ws::{listen, Handler, Message, Result, Sender};

struct Server {
    out: Sender,
}

impl Handler for Server {
    fn on_message(&mut self, msg: Message) -> Result<()> {
        // Send a response back to the client
        self.out.send(msg)
    }
}

fn main() {
    listen("127.0.0.1:6969", |out| Server { out }).unwrap();
}
