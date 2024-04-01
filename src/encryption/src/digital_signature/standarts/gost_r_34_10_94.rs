mod hash {
    use primitive_types::{U256, U512};

    fn hash_square(m: Vec<u8>) -> U256 {
        let modula = U512::from(2).pow(U512::from(256));
        let mut h: U256 = U256::from(0);
        for mi in m {
            let mi = U512::from(mi);
            let hi = U512::from(h);
            let a = ((hi + mi) * (hi + mi)) % modula;
            h.0[0] = a.0[0];
            h.0[1] = a.0[1];
            h.0[2] = a.0[2];
            h.0[3] = a.0[3];
        }
        h
    }
    #[cfg(test)]
    mod hash_test {
        use super::*;

        #[test]
        fn test_hash_square() {
            let m = vec![2,1,6,2,5,32,4,12,23,2,2,2,2,2,2,2,2,2,2,2,2,2,1,6,2,5,32,4,12,23,2,2,2,2,2,2,2,2,2,2,2,2,2,1,6,2,5,32,4,12,23,2,2,2,2,2,2,2,2,2,2,2,2,2,1,6,2,5,32,4,12,23,2,2,2,2,2,2,2,2,2,2,2,2,2,1,6,2,5,32,4,12,23,2,2,2,2,2,2,2,2,2,2,2,2,2,1,6,2,5,32,4,12,23,2,2,2,2,2,2,2,2,2,2,2,2,2,1,6,2,5,32,4,12,23,2,2,2,2,2,2,2,2,2,2,2,2,2,1,6,2,5,32,4,12,23,2,2,2,2,2,2,2,2,2,2,2,2,];
            let a = hash_square(m);
            println!("{:x}", a);
        }
    }
}
