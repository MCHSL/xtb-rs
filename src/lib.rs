mod commands;
mod json_socket;
mod transaction;
mod types;

use std::sync::{Arc, Mutex};

use flume::select::Selector;
pub use transaction::*;
pub use types::*;

pub use commands::*;
use json_socket::JsonSocket;

const DEFAULT_XAPI_ADDRESS: &str = "xapi.xtb.com";
const DEFAULT_XAPI_DEMO_PORT: usize = 5124;
const DEFAULT_XAPI_DEMO_STREAMING_PORT: usize = 5125;

#[derive(Debug)]
pub enum ErrorKind {
    Disconnected(std::io::Error),
    JsonError(serde_json::Error),
    InvalidResponse,
    ApiError(String, String),
    InternalError,
    WouldBlock,
}

pub type Result<T> = std::result::Result<T, ErrorKind>;

pub struct XTB {
    req_socket: JsonSocket,
    streaming_id: String,
    stream: flume::Receiver<StreamingMessage>,
    stream_socket: Arc<Mutex<JsonSocket>>,
}

impl XTB {
    pub fn connect(user_id: usize, password: &str) -> Result<Self> {
        let mut request_socket = JsonSocket::connect(DEFAULT_XAPI_ADDRESS, DEFAULT_XAPI_DEMO_PORT)?;

        let cmd = LoginCommand::new(user_id, password);
        let response: Response<()> = request_socket.send_recv(&cmd)?;

        use commands::Response::*;
        let streaming_id = match response {
            LoginSuccess {
                stream_session_id, ..
            } => stream_session_id,
            Success { .. } => return Err(ErrorKind::InvalidResponse),
            Response::Error {
                error_code,
                error_desc,
                ..
            } => return Err(ErrorKind::ApiError(error_code, error_desc)),
        };

        let streaming_socket = Arc::new(Mutex::new(JsonSocket::connect(
            DEFAULT_XAPI_ADDRESS,
            DEFAULT_XAPI_DEMO_STREAMING_PORT,
        )?));

        {
            streaming_socket.lock().unwrap().set_nonblocking(true);
        }

        let (stream_sender, stream_receiver) = flume::unbounded();

        let stream_socket_clone = streaming_socket.clone();
        std::thread::spawn(move || loop {
            {
                let mut sock = stream_socket_clone.lock().unwrap();
                match sock.recv() {
                    Ok(msg) => {
                        let msg: StreamingMessage = msg;
                        stream_sender.send(msg).unwrap();
                    }
                    Err(ErrorKind::WouldBlock) => {
                        drop(sock);
                        std::thread::sleep(std::time::Duration::from_millis(100));
                    }
                    Err(e) => {
                        println!("Error: {:?}", e);
                        return;
                    }
                }
            }
        });

        let s = Self {
            req_socket: request_socket,
            streaming_id,
            stream_socket: streaming_socket,
            stream: stream_receiver,
        };

        Ok(s)
    }

    pub fn start_balance(&mut self) -> Result<()> {
        let cmd = StreamingCommand::get_balance(self.streaming_id.clone());
        println!("Locking and sending balance");
        self.stream_socket.lock().unwrap().send(&cmd)?;
        println!("Unlocked and sent balance");
        Ok(())
    }

    pub fn start_trades(&mut self) -> Result<()> {
        let cmd = StreamingCommand::get_trades(self.streaming_id.clone());
        println!("Locking and sending trades");
        self.stream_socket.lock().unwrap().send(&cmd)?;
        println!("Unlocked and sent trades");
        Ok(())
    }

    pub fn messages(&mut self) -> impl Iterator<Item = StreamingMessage> + '_ {
        self.stream.try_iter()
    }

    pub fn handle_messages<F: FnMut(StreamingMessage)>(&mut self, mut cb: F) -> Result<()> {
        self.stream.try_iter().for_each(|msg| {
            cb(msg);
        });
        Ok(())
    }

    pub fn handle_messages_blocking<F: FnMut(StreamingMessage)>(
        &mut self,
        mut cb: F,
    ) -> Result<()> {
        self.stream.iter().for_each(|msg| {
            cb(msg);
        });
        Ok(())
    }

    pub fn send_transaction(&mut self, transaction: Transaction) -> Result<OrderId> {
        let cmd = TradeTransactionCommand::new(transaction);
        let response: Response<TradeTransactionResponse> = self.req_socket.send_recv(&cmd)?;
        match response {
            Response::Success { return_data, .. } => Ok(return_data.order),
            Response::Error {
                error_code,
                error_desc,
                ..
            } => Err(ErrorKind::ApiError(error_code, error_desc)),
            _ => Err(ErrorKind::InvalidResponse),
        }
    }

    pub fn buy<S: Into<Symbol>>(&mut self, symbol: S, volume: f64) -> Result<OrderId> {
        let symbol = symbol.into();
        self.send_transaction(symbol.buy(volume))
    }

    pub fn sell<S: Into<Symbol>>(&mut self, symbol: S, volume: f64) -> Result<OrderId> {
        let symbol = symbol.into();
        self.send_transaction(symbol.sell(volume))
    }

    pub fn close<S: Into<Symbol>>(
        &mut self,
        symbol: S,
        order_id: PositionId,
        volume: f64,
    ) -> Result<()> {
        let symbol = symbol.into();
        self.send_transaction(symbol.close(order_id, volume))?;
        Ok(())
    }
}
