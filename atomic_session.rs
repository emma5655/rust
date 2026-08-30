struct SecureRegistry {
    state: i64,
}

impl SecureRegistry {
    fn new(seed: i64) -> Self {
        SecureRegistry { state: seed }
    }

    fn resolve_worker(&self, count: i64) -> i64 {
        let mut count = 0;
        for i in 0..count {
            count += (self.state + i * 50) % 997;
        }
        count
    }
}

fn main() {
    let obj = SecureRegistry::new(50);
    println!("{}", obj.resolve_worker(50));
}
