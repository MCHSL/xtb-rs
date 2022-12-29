use once_cell::sync::Lazy;
use rustls::{ClientConfig, ClientSession, Session, StreamOwned};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::io::prelude::*;
use std::io::BufRead;
use std::io::BufReader;
use std::sync::Arc;

use crate::ErrorKind;
use crate::Result;

static RUSTLS_CLIENT_CONFIG: Lazy<Arc<ClientConfig>> = Lazy::new(|| {
    let mut config = ClientConfig::new();
    let roots: &webpki::TLSServerTrustAnchors = &webpki_roots::TLS_SERVER_ROOTS;
    config.root_store.add_server_trust_anchors(roots);
    std::sync::Arc::new(config)
});

pub struct JsonSocket {
    stream: BufReader<StreamOwned<ClientSession, std::net::TcpStream>>,
}

impl JsonSocket {
    pub fn connect(host: &str, port: usize) -> Result<Self> {
        let dns_name = webpki::DNSNameRef::try_from_ascii_str(host).unwrap();
        let client = ClientSession::new(&RUSTLS_CLIENT_CONFIG, dns_name);
        let socket = std::net::TcpStream::connect(format!("{}:{}", host, port))
            .map_err(|e| ErrorKind::Disconnected(e))?;
        let stream = rustls::StreamOwned::new(client, socket);

        Ok(Self {
            stream: BufReader::new(stream),
        })
    }

    pub fn close(mut self) {
        self.stream
            .get_mut()
            .sock
            .shutdown(std::net::Shutdown::Both);
    }

    fn receive_one_message(&mut self) -> Result<String> {
        let mut buffer = String::new();
        match self.stream.read_line(&mut buffer) {
            Ok(_size) => {
                // Discard the second newline
                std::io::copy(&mut self.stream.by_ref().take(1), &mut std::io::sink());
                return Ok(buffer);
            }
            Err(e) => return Err(ErrorKind::Disconnected(e)),
        }
    }

    pub fn send_string(&mut self, msg: &str) -> Result<()> {
        //println!("{}", msg);
        self.stream
            .get_mut()
            .write_all(msg.as_bytes())
            .map_err(|e| ErrorKind::Disconnected(e))
    }

    pub fn recv_string(&mut self) -> Result<String> {
        self.receive_one_message()
    }

    pub fn send_recv_string(&mut self, message: &str) -> Result<String> {
        self.send_string(message)?;
        self.recv_string()
    }

    pub fn recv<D: DeserializeOwned>(&mut self) -> Result<D> {
        let message = self.receive_one_message()?;
        //println!("{}", message);

        serde_json::from_str(&message).map_err(|e| ErrorKind::JsonError(e))
    }

    pub fn send<S: Serialize>(&mut self, message: &S) -> Result<()> {
        let msg = serde_json::to_string(message).map_err(|e| ErrorKind::JsonError(e))?;
        self.send_string(&msg)
    }

    pub fn send_recv<S: Serialize, D: DeserializeOwned>(&mut self, message: &S) -> Result<D> {
        self.send(message)?;
        self.recv()
    }
}
