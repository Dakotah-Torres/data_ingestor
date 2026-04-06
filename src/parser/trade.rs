pub struct Trade {
    pub price: f64,
    pub quant: f64,
}

pub fn parse_trade(p: &str, q: &str) -> Option<Trade>{
    let price = p.parse::<f64>().ok()?;
    let quant =  q.parse::<f64>().ok()?;
    Some(Trade{price, quant})
}
