use crate::types::*;

#[derive(Default)]
pub struct Transaction<'a> {
    pub kind: Option<OrderType>,
    pub side: Option<OrderSide>,
    pub symbol: Option<&'a str>,
    pub id: usize,
    pub volume: f64,
    pub comment: Option<&'a str>,

    expiration: u64,
    offset: usize,
    tp: f64,
    sl: f64,
    price: f64,
}

impl<'a> Transaction<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn for_order(order: OrderId) -> Self {
        let mut ret = Self::default();
        ret.id = order.into();
        ret
    }

    pub fn for_position(pos: PositionId) -> Self {
        let mut ret = Self::default();
        ret.id = pos.into();
        ret
    }

    pub fn id<O: Into<usize>>(mut self, a: O) -> Self {
        self.id = a.into();
        self
    }

    pub fn kind(mut self, a: OrderType) -> Self {
        self.kind = Some(a);
        self
    }

    pub fn side(mut self, a: OrderSide) -> Self {
        self.side = Some(a);
        self
    }

    pub fn symbol(mut self, a: &'a str) -> Self {
        self.symbol = Some(a);
        self
    }

    pub fn volume(mut self, a: f64) -> Self {
        self.volume = a;
        self
    }
}
