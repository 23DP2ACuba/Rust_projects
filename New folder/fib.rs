use num_bigint::BigInt;
use num_traits::{One, Zero};

fn fib(n: u32) -> BigInt  {
    let mut n = n - 1;
    fn pair(k: u32) -> (BigInt , BigInt) {
        if k == 0 {
            return (BigInt::zero(), BigInt::one());
        }
        let (f, f1) = pair(k / 2);
        
        let f2 = &f * ((&f1 * 2) - &f);
        let f2_1 = &f * &f + &f1 * &f1;
       
        if k % 2 == 0 {
            (f2, f2_1)
        } else {
            (f2_1.clone(), f2 + f2_1)
        }
    }
    pair(n).1
}

pub fn run() {
    println!("{}", fib(1_000)); 
}
