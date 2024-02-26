use super::Register;

struct System {
    r1: Register,
    r2: Register,
    r3: Register
}

impl System {
    fn new(_key: &[u8]) -> Self {
        let r1 = Register::new(19, 0x72000, 0, (2<<19)-1);
        let r2 = Register::new(22, 0x300000, 0, (2<<22)-1);
        let r3 = Register::new(23, 0x700080, 0, (2<<23)-1);
        Self { r1, r2, r3 }
    }
}