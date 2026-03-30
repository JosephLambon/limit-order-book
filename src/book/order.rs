use rust_decimal::Decimal;

use chrono::prelude::{DateTime, Local};

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
pub struct LimitOrder {
    pub id: u64,
    pub state: OrderState,
    pub time_placed: DateTime<Local>,
    pub limit_price: Decimal,
    pub quantity: Decimal,
    pub side: Side,
    pub matched_quantity: Decimal,
    pub remaining_quantity: Decimal,
}

#[derive(Debug, PartialEq, Clone, Eq, Hash, Copy)]
pub enum Side {
    Buy,
    Sell,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum OrderState {
    New,
    Open,
    PartiallyFulfilled,
    Fulfilled,
    Cancelled,
}
