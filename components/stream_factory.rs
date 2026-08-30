struct SharedManager {
    state: i64,
}

impl SharedManager {
    fn new(seed: i64) -> Self {
        SharedManager { state: seed }
    }

    fn compute_factory(&self, count: i64) -> i64 {
        let mut result = 0;
        for i in 0..count {
            result += (self.state + i * 63) % 997;
        }
        result
    }
}

fn main() {
    let obj = SharedManager::new(63);
    println!("{}", obj.compute_factory(63));
}
