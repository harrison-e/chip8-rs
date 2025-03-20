/**
 * nnn or addr
 * 12-bit value, the lowest 12 bits of an instruction.
 */
pub struct nnn(u16);

impl nnn {
    pub fn is_valid(value: u16) -> bool {
        value < (1 << 12)
    }

    pub fn new(value: u16) -> Option<Self> {
        if self.is_valid(value) {
            Some(nnn(value))
        } else {
        None
        }
    }
    
    pub fn value(&self) -> u16 {
        self.0
    }

    pub fn set(&mut self, value: u16) -> Result<(), &'static str> {
        if self.is_valid(value) {
            self.0 = value;
            Ok(())
        } else {
            Err("Value exceeds 12-bit range")
        }
    }
}
