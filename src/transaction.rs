use derive_builder::Builder;

use crate::types::*;

#[derive(Default, Builder, Debug)]
pub struct Transaction<'a> {
    #[builder(setter(into, strip_option))]
    pub kind: Option<OrderType>,
    #[builder(setter(into, strip_option), default)]
    pub side: Option<OrderSide>,
    #[builder(setter(into, strip_option), default)]
    pub symbol: Option<&'a str>,
    #[builder(default)]
    pub order: OrderId,
    pub volume: f64,
    #[builder(setter(into, strip_option), default)]
    pub comment: Option<&'a str>,
    #[builder(default)]
    pub typ: usize,

    #[builder(default)]
    expiration: u64,
    #[builder(default)]
    offset: usize,
    #[builder(default)]
    tp: f64,
    #[builder(default)]
    sl: f64,
    #[builder(default)]
    price: f64,
}

impl Symbol {
    pub fn buy(&self, volume: f64) -> Transaction {
        TransactionBuilder::default()
            .symbol(self.0.as_ref())
            .side(OrderSide::Buy)
            .kind(OrderType::Open)
            .volume(volume)
            .build()
            .unwrap()
    }

    pub fn sell(&self, volume: f64) -> Transaction {
        TransactionBuilder::default()
            .symbol(self.0.as_ref())
            .kind(OrderType::Open)
            .side(OrderSide::Sell)
            .volume(volume)
            .build()
            .unwrap()
    }

    pub fn close(&self, position: PositionId, volume: f64) -> Transaction {
        TransactionBuilder::default()
            .symbol(self.0.as_ref())
            .kind(OrderType::Close)
            .side(OrderSide::Sell)
            .order(OrderId(position.0))
            .volume(volume)
            .build()
            .unwrap()
    }
}
