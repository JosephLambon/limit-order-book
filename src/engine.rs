use std::{
    collections::HashMap, 
    sync::mpsc::{channel,Sender},
    thread
};


use tracing::info;

use crate::book::{LimitOrder, OrderBook};

#[derive(Eq, PartialEq, Hash)]
pub enum InstrumentKey {
    Btc,
    Eth,
}

pub struct Engine {
    pub senders: HashMap<InstrumentKey, Sender<LimitOrder>>,
}

impl Engine {
    pub fn new() -> Self {
        Engine {
            senders: HashMap::new()
        }
    }

    pub fn add_instrument(&mut self, ticker_symbol: InstrumentKey) {
        let (tx, rx) = channel::<LimitOrder>();
        self.senders.insert(ticker_symbol, tx);

        // Listen for orders until channel closes
        thread::spawn(move || {
            let mut order_book = OrderBook::new();

            while let Ok(order) = rx.recv() {
                order_book.insert(order);
                if order_book.check_match() {
                    info!("MATCH FOUND");
                };
            }
        });
    }
}
