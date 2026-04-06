pub mod trade;

pub fn extract_price(raw: &str) -> Option<&str> {
    let key = "\"p\":\"";
    let start = raw.find(key)? + key.len();
    let end = raw[start..].find('\"')? + start;
    Some(&raw[start..end])
}

pub fn extract_quantity(raw: &str) -> Option<&str> {
    let key = "\"q\":\"";
    let start = raw.find(key)? + key.len();
    let end = raw[start..].find('\"')? + start;
    Some(&raw[start..end])
}