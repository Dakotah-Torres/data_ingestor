
use std::fs::File;
use std::io::{BufWriter, Write};
use std::time:: {SystemTime, UNIX_EPOCH};

pub struct DataBuffer {
    messages: Vec<String>,
    capacity: usize,
    cap_trigger: f32,
}

impl DataBuffer {
    pub fn new(capacity: usize, trigger: f32) -> Self {
        DataBuffer{
            messages: Vec::with_capacity(capacity),
            capacity: capacity, 
            cap_trigger: trigger,
        }
    }
    pub fn push_message(&mut self, message: String){
        self.messages.push(message);
    }

    pub fn capacity_check(&self) -> usize{
        let current_cap = self.messages.len();
        current_cap
    }

    pub fn trigger_swap(&self) -> bool {
      self.messages.len() >= (self.capacity as f32 * self.cap_trigger) as usize 
    }

    pub fn save_and_clean(&mut self, stream_type: &str, symbol: &str) -> Result<String, anyhow::Error> {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis()
            .to_string();
        
        let file_path = format!("{}_{}_{}.bin", stream_type, symbol, timestamp);
        let file = File::create(&file_path)?;

        let mut writer = BufWriter::new(file);

        bincode::serialize_into(&mut writer, &self.messages).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

        writer.flush()?;
        self.messages.clear();
        Ok(file_path)
    }
}