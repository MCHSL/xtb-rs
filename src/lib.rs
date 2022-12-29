mod commands;
mod json_socket;
mod transaction;
mod types;

use commands::*;
use json_socket::JsonSocket;

use std::sync::mpsc;

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
}

pub type Result<T> = std::result::Result<T, ErrorKind>;

struct XTBStreamHandler {
    sock: JsonSocket,
    chan: mpsc::Sender<String>,
}

struct XTB {
    req_socket: JsonSocket,
    stream_handler: XTBStreamHandler,
    stream: mpsc::Receiver<String>,
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

        let streaming_socket =
            JsonSocket::connect(DEFAULT_XAPI_ADDRESS, DEFAULT_XAPI_DEMO_STREAMING_PORT)?;

        let s = Self {
            req_socket: request_socket,
        };

        Ok(s)
    }
}
