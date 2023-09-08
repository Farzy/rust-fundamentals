use std::io::Read;
use std::net::TcpListener;

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self {
            addr
        }
    }

    pub fn run(self) {
        println!("Listening on {}", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    println!("Got a connection: {:?}", stream);
                    let mut buffer = [0; 1024];
                    stream.read(&mut buffer);
                },
                Err(e) => println!("Failed to establish a connection: {}", e),
            }
        }
    }
}
