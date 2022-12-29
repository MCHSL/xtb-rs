#![feature(concat_idents)]
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

use crate::transaction::*;
use crate::types::*;
//use casey::pascal;

pub trait Command {}
pub trait CResponse {}

#[derive(Serialize)]
pub struct BaseCommand<S: Serialize> {
    command: &'static str,
    arguments: S,
}

impl<S: Serialize> Command for BaseCommand<S> {}

#[derive(Deserialize)]
#[serde(untagged)]
#[allow(dead_code)]
pub enum Response<D> {
    Success {
        status: bool,
        #[serde(rename = "returnData")]
        return_data: D,
    },
    Error {
        status: bool,
        #[serde(rename = "errorCode")]
        error_code: String,
        #[serde(rename = "errorDescr")]
        error_desc: String,
    },
    //hhh
    LoginSuccess {
        status: bool,
        #[serde(rename = "streamSessionId")]
        stream_session_id: String,
    },
}

impl<D> CResponse for Response<D> {}

#[derive(Serialize)]
struct GetTradeRecordsArguments {
    orders: Vec<OrderId>,
}

type GetTradeRecordsCommand = BaseCommand<GetTradeRecordsArguments>;
impl GetTradeRecordsCommand {
    fn new(orders: Vec<OrderId>) -> Self {
        Self {
            command: "getTradeRecords",
            arguments: GetTradeRecordsArguments { orders },
        }
    }
}

#[derive(Serialize)]
struct GetTradesArguments {
    #[serde(rename = "openedOnly")]
    opened_only: bool,
}

type GetTradesCommand = BaseCommand<GetTradesArguments>;
impl GetTradesCommand {
    pub(crate) fn new(opened_only: bool) -> Self {
        Self {
            command: "getTrades",
            arguments: GetTradesArguments { opened_only },
        }
    }
}

#[derive(Serialize)]
struct TradeTransInfo {
    cmd: OrderSide,
    #[serde(rename = "customComment")]
    custom_comment: String,
    expiration: u64,
    offset: usize,
    order: OrderId,
    price: f64,
    sl: f64,
    symbol: String,
    tp: f64,
    #[serde(rename = "type")]
    typ: OrderType,
    volume: f64,
}

#[derive(Serialize)]
pub struct TradeTransactionArguments {
    #[serde(rename = "tradeTransInfo")]
    trade_trans_info: TradeTransInfo,
}

pub type TradeTransactionCommand = BaseCommand<TradeTransactionArguments>;
impl TradeTransactionCommand {
    pub fn new(order: Transaction) -> Self {
        Self {
            command: "tradeTransaction",
            arguments: TradeTransactionArguments {
                trade_trans_info: TradeTransInfo {
                    cmd: order.side.unwrap_or(OrderSide::Buy),
                    custom_comment: order.comment.unwrap_or_default().to_owned(),
                    expiration: 9999999999999u64,
                    offset: 0,
                    order: OrderId::from(order.id),
                    price: 1.0,
                    sl: 0.0,
                    symbol: order.symbol.unwrap_or_default().to_owned(),
                    tp: 0.0,
                    typ: order.kind.unwrap(),
                    volume: order.volume,
                },
            },
        }
    }
}

#[derive(Deserialize)]
pub struct TradeTransactionResponse {
    pub order: OrderId,
}

#[derive(Serialize)]
pub struct LoginArguments<'a> {
    #[serde(rename = "userId")]
    user_id: usize,
    password: &'a str,
    #[serde(rename = "appName")]
    app_name: &'a str,
}

pub type LoginCommand<'a> = BaseCommand<LoginArguments<'a>>;
impl<'a> LoginCommand<'a> {
    pub fn new(user_id: usize, password: &'a str) -> Self {
        Self {
            command: "login",
            arguments: LoginArguments {
                user_id,
                password,
                app_name: "",
            },
        }
    }
}

macro_rules! streaming_commands {
    ($($name:ident, $cmd:expr;)*) => {
        $(#[derive(Serialize)]
        pub struct $name<'a> {
			command: &'static str,
			#[serde(rename = "streamSessionId")]
			stream_session_id: &'a str,
		}

		impl<'a> Command for $name<'a> {}

		impl<'a> $name<'a> {
			pub fn new(stream_session_id: &'a str) -> Self {
				Self {
					command: $cmd,
					stream_session_id
				}
			}
		})*
    };
}

macro_rules! streaming_commands_with_fields {
	($($name:ident, $cmd:expr => { $($element: ident: $ty: ty),* };)*) => {
        $(
		#[derive(Serialize)]
        pub struct $name<'a> {
			command: &'static str,
			#[serde(rename = "streamSessionId")]
			stream_session_id: &'a str,
			$($element: $ty),*
		}

		impl<'a> Command for $name<'a> {}

		impl<'a> $name<'a> {
			pub fn new(stream_session_id: &'a str, $($element: $ty),*) -> Self {
				Self {
					command: $cmd,
					stream_session_id,
					$($element),*
				}
			}
		}
    )*
    };
}

streaming_commands! {
    GetBalanceStreamCommand, "getBalance";
    GetTradesStreamCommand, "getTrades";
    StopBalanceStreamCommand, "stopBalance";
}

streaming_commands_with_fields! {
    GetCandlesStreamCommand, "getCandles" => { symbol: String };
}

#[derive(Deserialize)]
#[serde(tag = "command")]
pub enum StreamingMessage {
    balance { data: BalanceRecord },
    trade { data: TradeRecord },
}
