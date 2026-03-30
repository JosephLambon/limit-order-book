use chrono::{DateTime, Local};
use rust_decimal::Decimal;

use crate::book::{LimitOrder, Side, order::OrderState};

pub enum Command {
    PlaceOrder(LimitOrder),
    CancelOrder(LimitOrder),
    Shutdown,
}
#[derive(Debug)]
pub enum Event {
    OrderPlaced(OrderPlacedEvent),
    OrderCancelled(CancellationEvent),
    // OrdersMatched(OrdersMatchedEvent),
    Shutdown,
}

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
pub struct OrderPlacedEvent {
    pub id: u64,
    pub state: OrderState,
    pub time_placed: DateTime<Local>,
    pub time_accepted: DateTime<Local>,
    pub limit_price: Decimal,
    pub quantity: Decimal,
    pub side: Side,
    pub matched_quantity: Decimal,
    pub remaining_quantity: Decimal,
}

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
pub struct CancellationEvent {
    pub id: u64,
    pub state: OrderState,
    pub time_placed: DateTime<Local>,
    pub time_accepted: DateTime<Local>,
    pub time_cancelled: DateTime<Local>,
    pub limit_price: Decimal,
    pub quantity: Decimal,
    pub side: Side,
    pub matched_quantity: Decimal,
    pub remaining_quantity: Decimal,
}
