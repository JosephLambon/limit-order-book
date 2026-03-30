use std::{
    collections::{HashMap, hash_map::Entry},
    sync::mpsc,
    thread,
};

use chrono::Local;
use rust_decimal::dec;
use tracing::info;

use crate::book::OrderBook;

pub mod event;
use event::*;

// Re-export
pub use event::Command;

#[derive(Eq, PartialEq, Hash)]
pub enum InstrumentKey {
    Btc,
    Eth,
}

pub struct Engine {
    pub senders: HashMap<InstrumentKey, mpsc::Sender<Command>>,
}

impl Engine {
    pub fn new() -> Self {
        Engine {
            senders: HashMap::new(),
        }
    }

    pub fn add_instrument(&mut self, ticker_symbol: InstrumentKey) {
        if let Entry::Vacant(entry) = self.senders.entry(ticker_symbol) {
            let (tx, rx) = mpsc::channel::<Command>();

            entry.insert(tx);

            // Listen for commands until shutdown
            thread::spawn(move || {
                let mut order_book = OrderBook::new();
                let mut temporary_audit_log: Vec<Event> = vec![];

                while let Ok(command) = rx.recv() {
                    match command {
                        Command::PlaceOrder(order) => {
                            // Audit log event
                            temporary_audit_log.push(Event::OrderPlaced(OrderPlacedEvent {
                                id: order.id,
                                state: order.state,
                                time_placed: order.time_placed,
                                time_accepted: Local::now(),
                                limit_price: order.limit_price,
                                quantity: order.quantity,
                                side: order.side,
                                matched_quantity: dec!(0),
                                remaining_quantity: order.remaining_quantity,
                            }));

                            order_book.insert(order);

                            if order_book.check_match() {
                                info!("MATCH FOUND");
                                // Instatiate a 'OrdersMatchedEvent' struct

                                // Audit log OrdersMatched event

                                // Send OrdersMatchedEvent Command to
                            };
                        }
                        Command::CancelOrder(order) => { 
                            temporary_audit_log.push(Event::OrderCancelled(CancellationEvent {
                                id: order.id,
                                state: order.state,
                                time_placed: order.time_placed,
                                time_accepted: order.time_placed,
                                time_cancelled: Local::now(),
                                limit_price: order.limit_price,
                                quantity: order.quantity,
                                side: order.side,
                                matched_quantity: dec!(0),
                                remaining_quantity: order.remaining_quantity,
                            }));
                            info!("CANCELLATION PLACEHOLDER");
                        }
                        Command::Shutdown => {
                            // Audit log shutdown event
                            temporary_audit_log.push(Event::Shutdown);
                            // Execute trade
                            println!("\n\n THREAD CLOSING... \n\n");
                            println!("\n\n Audit log: {:#?} \n\n", temporary_audit_log);
                            break;
                        }
                    }
                }
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::mpsc::TryRecvError;

    use super::*;

    #[test]
    fn add_instrument_inserts_to_senders() {
        let mut engine = Engine::new();

        engine.add_instrument(InstrumentKey::Btc);

        assert_eq!(engine.senders.len(), 1);
    }

    #[test]
    fn add_instrument_does_not_overwrite_existing() {
        let mut engine = Engine::new();

        let (tx, rx) = mpsc::channel();

        engine.senders.insert(InstrumentKey::Btc, tx);

        engine.add_instrument(InstrumentKey::Btc);

        assert_ne!(rx.try_recv().err().unwrap(), TryRecvError::Disconnected);
    }

    #[test]
    fn add_instrument_inserts_separate_thread_per_instrument() {
        let mut engine = Engine::new();

        engine.add_instrument(InstrumentKey::Btc);
        engine.add_instrument(InstrumentKey::Eth);

        assert_eq!(engine.senders.len(), 2);
    }
}
