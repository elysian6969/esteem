use std::io;
use std::net::TcpListener;

pub struct Steam3Server {
    pub listener: TcpListener,
}

impl Steam3Server {
    pub fn bind() -> io::Result<Self> {
        let listener = TcpListener::bind("127.0.0.1:57343")?;

        Ok(Self { listener })
    }
}
