use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Serialize_repr, Deserialize_repr, Clone, Debug)]
#[repr(u8)]
pub enum OrderSide {
    Buy,
    Sell,
    BuyLimit,
    SellLimit,
    BuyStop,
    SellStop,
}

impl Default for OrderSide {
    fn default() -> Self {
        Self::Buy
    }
}

#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum OrderType {
    Open,
    Pending,
    Close,
    Modify,
    Delete,
}

impl Default for OrderType {
    fn default() -> Self {
        Self::Open
    }
}

#[derive(Serialize, Deserialize, PartialEq, Default, Clone, Copy, Debug)]
pub struct OrderId(pub usize);

impl From<usize> for OrderId {
    fn from(value: usize) -> Self {
        Self { 0: value }
    }
}

impl Into<usize> for OrderId {
    fn into(self) -> usize {
        self.0
    }
}

#[derive(Serialize, Deserialize, PartialEq, Default, Clone, Copy, Debug)]
pub struct PositionId(pub usize);

impl From<usize> for PositionId {
    fn from(value: usize) -> Self {
        Self { 0: value }
    }
}

impl Into<usize> for PositionId {
    fn into(self) -> usize {
        self.0
    }
}

#[allow(dead_code)]
#[derive(Deserialize, Default, Clone, Debug)]
#[serde(default)]
pub struct TradeRecord {
    close_price: f64,
    close_time: Option<u64>,
    #[serde(rename = "close_timeString")]
    close_time_string: Option<String>,
    closed: bool,
    #[serde(rename = "cmd")]
    side: OrderSide,
    comment: Option<String>,
    commission: f64,
    #[serde(rename = "customComment")]
    custom_comment: Option<String>,
    digits: usize,
    expiration: Option<u64>,
    #[serde(rename = "expirationString")]
    expiration_string: Option<String>,
    margin_rate: f64,
    offset: u64,
    open_price: f64,
    open_time: Option<u64>,
    #[serde(rename = "open_timeString")]
    open_time_string: Option<String>,
    #[serde(rename = "order")]
    pub order: Option<OrderId>,
    #[serde(rename = "order2")]
    pub order2: OrderId,
    pub position: PositionId,
    pub profit: Option<f64>,
    pub sl: f64,
    pub storage: f64,
    pub symbol: String,
    pub timestamp: u64,
    pub tp: f64,
    pub volume: f64,
    #[serde(rename = "type")]
    pub kind: OrderType,

    #[serde(rename = "nominalValue")]
    nominal_value: Option<f64>,
    spread: usize,
    taxes: f64,
    state: String,
}

#[derive(Deserialize, Default, Clone, Debug)]
pub struct BalanceRecord {
    pub balance: f64,
    pub credit: f64,
    pub equity: f64,
    pub margin: f64,
    #[serde(rename = "marginFree")]
    pub margin_free: f64,
    #[serde(rename = "marginLevel")]
    pub margin_level: f64,
}

pub struct Symbol(pub String);

impl Symbol {
    pub fn new(symbol: &str) -> Self {
        Self(symbol.to_owned())
    }
}

impl From<&str> for Symbol {
    fn from(value: &str) -> Self {
        Self(value.to_owned())
    }
}

impl From<String> for Symbol {
    fn from(value: String) -> Self {
        Self(value)
    }
}
