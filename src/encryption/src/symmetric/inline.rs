pub mod a5_1;
pub mod a5_2;

struct Register {
    size: u8,
    scrambler: u32,
    value_start: u32,
    value: u32,
    control: u32
}

impl Register {
    fn new(size: u8, scrambler: u32, value: u32, control: u32) -> Self {
        Self {
            size, scrambler, value, control,
            value_start: value
        }
    }
    
    fn get_exit_bit(&self) -> u32 {
        self.value & (1<<self.size)
    }
    
    fn shift(&mut self) {
        let mut and_unit: u32 = 1<<self.size;
        let mut new_value = self.value & and_unit;
        for _ in 0..self.size {
            new_value >>= 1;
            and_unit >>= 1;
            if self.scrambler & and_unit != 0 {
                new_value ^= self.value;
                new_value &= and_unit;
            }
        }
        self.value >>= 1;
        self.value |= new_value;
    }
    
    fn get_control_bit(&self) -> u8 {
        if self.value & self.control != 0 { 1 } else { 0 }
    }
}
