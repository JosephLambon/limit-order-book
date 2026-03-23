use std::collections::{BTreeMap, VecDeque};

use rust_decimal::Decimal;

use chrono::prelude::*;

pub struct OrderBook {
    pub asks: BTreeMap<Decimal, VecDeque<LimitOrder>>,
    pub bids: BTreeMap<Decimal, VecDeque<LimitOrder>>,
}

impl OrderBook {
    pub fn new() -> Self {
        Self {
            asks: BTreeMap::new(),
            bids: BTreeMap::new(),
        }
    }

    pub fn add_to_order_book(&mut self, limit_order: LimitOrder) {
        match limit_order.side {
            Side::Buy => {
                OrderBook::add_to_queue(&mut self.bids, limit_order);
            }
            Side::Sell => {
                OrderBook::add_to_queue(&mut self.asks, limit_order);
            }
        }
    }

    fn add_to_queue(
        order_book_side: &mut BTreeMap<Decimal, VecDeque<LimitOrder>>,
        limit_order: LimitOrder,
    ) {
        order_book_side
            .entry(limit_order.limit_price)
            .or_default()
            .push_back(limit_order);

        println!("Updated Order Book Side: {:#?}\n", order_book_side);
    }
}

impl Default for OrderBook {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct LimitOrder {
    pub time_placed: DateTime<Local>,
    pub limit_price: Decimal,
    pub quantity: Decimal,
    pub side: Side,
}

#[derive(Debug, Clone)]
pub enum Side {
    Buy,
    Sell,
}
