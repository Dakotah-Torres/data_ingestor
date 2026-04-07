
use crate::models::Trade;
pub struct TradeBuffer {
    pub active: Vec<Trade>,
    pub flush: Vec<Trade>,
    pub capacity: usize,
}

impl TradeBuffer {
    pub fn new(capacity: usize) -> Self {
        TradeBuffer{
            active: Vec::with_capacity(capacity),
            flush: Vec::with_capacity(capacity),
            capacity,
        }
    }

    pub fn push(&mut self, trade: Trade) -> bool{
        self.active.push(trade);
        self.active.len() >= (self.capacity * 9 / 10)
    }

    pub fn swap(&mut self){
        std::mem::swap(&mut self.active,&mut self.flush)
    }

    pub fn drain(&mut self) -> Vec<Trade>{
        std::mem::take(&mut self.flush)
    }
}