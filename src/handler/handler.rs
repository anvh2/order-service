use crate::cache::OrderStore;

#[derive(Clone)]
pub struct OrderService<S>
where
    S: OrderStore + Send + Clone + 'static,
{
    pub store: S,
}

impl<S> OrderService<S> 
where
    S: OrderStore + Send + Clone + 'static,
{
    pub fn new(store: S) -> Self {
        Self { store: store }
    }
}
