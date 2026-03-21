use std::collections::BTreeMap;

use rust_decimal::{Decimal, dec};

fn main() {
    println!("\nWelcome. This is Joe's Order Book.");
    println!("==================================\n\n");

    let mut orders: BTreeMap<Decimal, LimitOrder> = BTreeMap::new();

    let order1 = LimitOrder {
        stock: String::from("GOOGL"),
        limit_price: dec!(1234.5600),
        quantity: dec!(50),
        action: LimitOrderAction::Buy,
    };
    let order2 = LimitOrder {
        stock: String::from("AAPL"),
        limit_price: dec!(1123.5698),
        quantity: dec!(50),
        action: LimitOrderAction::Sell,
    };

    orders.insert(order1.limit_price, order1);
    orders.insert(order2.limit_price, order2);

    let mut orders_iterator = orders.iter().enumerate();

    while let Some((index, order)) = orders_iterator.next() {
        println!(
            "Order {index}: {:?} {} shares of {} at limit price £{}",
            order.1.action, order.1.quantity, order.1.stock, order.1.limit_price
        );
    }
}

pub struct LimitOrder {
    stock: String,
    limit_price: Decimal,
    quantity: Decimal,
    action: LimitOrderAction,
}

#[derive(Debug)]
pub enum LimitOrderAction {
    Buy,
    Sell,
}
