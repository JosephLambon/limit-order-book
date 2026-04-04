use std::{
    collections::{HashMap, hash_map::Entry}, sync::mpsc, thread
};

use chrono::Local;
use rust_decimal::dec;
use tracing::{debug, error};

use crate::{book::OrderBook};

pub mod event;
use event::*;

// Re-export
pub use event::EngineCommand;

#[derive(Eq, PartialEq, Clone, Copy, Hash)]
pub enum InstrumentKey {
    Btc,
    Eth,
}

pub struct Engine {
    pub senders: HashMap<InstrumentKey, mpsc::Sender<EngineCommand>>
}

impl Engine {
    pub fn new() -> Self {
        Engine {
            senders: HashMap::new()
        }
    }

    pub fn add_instrument(&mut self, ticker_symbol: InstrumentKey) {
        if let Entry::Vacant(entry) = self.senders.entry(ticker_symbol) {
            let (tx, rx) = mpsc::channel::<EngineCommand>();

            entry.insert(tx);

            // Listen for commands until shutdown
            thread::spawn(move || {
                let mut order_book = OrderBook::new();
                let mut temporary_audit_log: Vec<EngineEvent> = vec![];

                while let Ok(command) = rx.recv() {
                    match command {
                        EngineCommand::PlaceOrder(order) => {
                            // Audit log event
                            temporary_audit_log.push(EngineEvent::OrderPlaced(OrderPlacedEvent {
                                id: order.id,
                                state: order.state,
                                placed_at: order.placed_at,
                                accepted_at: Local::now(),
                                limit_price: order.limit_price,
                                quantity: order.quantity,
                                side: order.side,
                                quantity_traded: dec!(0),
                                quantity_remaining: order.quantity_remaining,
                            }));

                            order_book.insert(order);

                            while let Some(result) = order_book.match_sides() {
                                debug!("Match found.");
                                order_book.orders_placed += 1;

                                let match_event = EngineEvent::OrdersMatched(
                                    OrdersMatchedEvent {
                                        id: order_book.orders_placed,
                                        matched_at: Local::now(),
                                        ask_id: result.ask_id,
                                        bid_id: result.bid_id,
                                        ask_price: result.ask_price,
                                    }
                                );
                                
                                // Audit log OrdersMatched event
                                temporary_audit_log.push(match_event.clone());

                                // Send OrdersMatchedEvent EngineCommand to executor
                                let result = order_book.process(&match_event);

                                // IF result success, add to temp log.
                                if let Ok(event) = result {
                                    debug!("Trade successfully executed.");
                                    temporary_audit_log.push(event);
                                } else {
                                    //  error handling
                                    error!("Unable to execute trade.");
                                }
                            };
                        }
                        EngineCommand::CancelOrder(order) => {
                            temporary_audit_log.push(EngineEvent::OrderCancelled(
                                CancellationEvent {
                                    id: order.id,
                                    state: order.state,
                                    placed_at: order.placed_at,
                                    accepted_at: order.placed_at,
                                    cancelled_at: Local::now(),
                                    limit_price: order.limit_price,
                                    quantity: order.quantity,
                                    side: order.side,
                                    quantity_traded: dec!(0),
                                    quantity_remaining: order.quantity_remaining,
                                },
                            ));
                            debug!("CANCELLATION PLACEHOLDER");
                        }
                        EngineCommand::Shutdown => {
                            // Audit log shutdown event
                            temporary_audit_log.push(EngineEvent::Shutdown);
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
