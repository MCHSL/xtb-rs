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

#[derive(Serialize_repr, Deserialize_repr, Debug)]
#[repr(u8)]
pub enum OrderType {
    Open,
    Close = 2,
    Modify,
    Delete,
}

#[derive(Serialize, Deserialize, PartialEq, Default, Clone, Copy, Debug)]
pub struct OrderId(usize);

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
pub struct PositionId(usize);

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
    comment: String,
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
    pub open_order: OrderId,
    #[serde(rename = "order2")]
    pub close_order: OrderId,
    position: PositionId,
    profit: Option<f64>,
    sl: f64,
    storage: f64,
    symbol: String,
    timestamp: u64,
    tp: f64,
    volume: f64,

    #[serde(rename = "nominalValue")]
    nominal_value: Option<f64>,
    spread: usize,
    taxes: f64,
    state: String,
}

#[derive(Deserialize, Default, Clone, Debug)]
pub struct BalanceRecord {
    balance: f64,
    credit: f64,
    equity: f64,
    margin: f64,
    #[serde(rename = "marginFree")]
    margin_free: f64,
    #[serde(rename = "marginLevel")]
    margin_level: f64,
}
