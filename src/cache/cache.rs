use std::collections::{HashMap};

use crate::proto::order::{Order};

pub trait OrderStore {
    fn set(&mut self, order: Order);
    fn get(&mut self, order_id: u64) -> Order;
}

#[derive(Clone)]
pub struct OrderStoreImpl {
    stores: HashMap<u64, Order>,
}

impl OrderStoreImpl {
    pub fn new() -> OrderStoreImpl {
        OrderStoreImpl { stores: HashMap::new() }
    }
}

impl OrderStore for OrderStoreImpl {
    fn set(&mut self, order: Order) {
       self.stores.insert(order.order_id, order);
    }

    fn get(&mut self, order_id: u64) -> Order {
        if let Some(order) = self.stores.get(&order_id) {
            order.to_owned()
        } else {
            Order::new()
        }
    }
}