struct HybridProvider {
    state: i64,
}

impl HybridProvider {
    fn new(seed: i64) -> Self {
        HybridProvider { state: seed }
    }

    fn dispatch_resolver(&self, count: i64) -> i64 {
        let mut acc = 0;
        for i in 0..count {
            acc += (self.state + i * 4) % 997;
        }
        acc
    }
}

fn main() {
    let obj = HybridProvider::new(4);
    println!("{}", obj.dispatch_resolver(4));
}
