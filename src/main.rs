pub mod book;
mod engine;
use std::{process, thread};

use book::*;

use tracing::{Level, info, trace};

use core::time::Duration;
use chrono::{Local};
use rust_decimal::dec;

use engine::*;

fn main() {
    tracing_subscriber::fmt::fmt()
        .with_max_level(Level::TRACE)
        .init();

    info!("Welcome. This is Joe's Order Book.");
    info!("==================================\n\n");

    let mut engine = Engine::new();
    engine.add_instrument(InstrumentKey::Btc);
    engine.add_instrument(InstrumentKey::Eth);

    let bid1 = LimitOrder {
        id: 1,
        time_placed: Local::now(),
        limit_price: dec!(1234.5600),
        quantity: dec!(50),
        side: Side::Buy,
    };
    let bid2 = LimitOrder {
        id: 2,
        time_placed: Local::now(),
        limit_price: dec!(1234.5600),
        quantity: dec!(50),
        side: Side::Buy,
    };
    let bid3 = LimitOrder {
        id: 3,
        time_placed: Local::now(),
        limit_price: dec!(1234.5320),
        quantity: dec!(50),
        side: Side::Buy,
    };
    let ask1 = LimitOrder {
        id: 4,
        time_placed: Local::now(),
        limit_price: dec!(1123.5698),
        quantity: dec!(50),
        side: Side::Sell,
    };
    let ask2 = LimitOrder {
        id: 5,
        time_placed: Local::now(),
        limit_price: dec!(1123.5696),
        quantity: dec!(50),
        side: Side::Sell,
    };
    let ask3 = LimitOrder {
        id: 6,
        time_placed: Local::now(),
        limit_price: dec!(1123.5698),
        quantity: dec!(50),
        side: Side::Sell,
    };

    let tx_btc = engine.senders.get(&InstrumentKey::Btc)
        .unwrap_or_else(|| { process::exit(1) })
        .clone();

    let tx_eth = engine.senders.get(&InstrumentKey::Eth)
        .unwrap_or_else(|| { process::exit(1) })
        .clone();

    tx_btc.send(bid1.clone());
    tx_btc.send(bid2.clone());
    tx_btc.send(bid3.clone());
    tx_btc.send(ask1.clone());
    tx_btc.send(ask2.clone());
    tx_btc.send(ask3.clone());

    tx_eth.send(bid1);
    tx_eth.send(bid2);
    tx_eth.send(bid3);
    tx_eth.send(ask1);
    tx_eth.send(ask2);
    tx_eth.send(ask3);

    thread::sleep(Duration::from_secs(10));
}
