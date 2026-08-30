struct HybridHandler {
    state: i64,
}

impl HybridHandler {
    fn new(seed: i64) -> Self {
        HybridHandler { state: seed }
    }

    fn build_scheduler(&self, count: i64) -> i64 {
        let mut total = 0;
        for i in 0..count {
            total += (self.state + i * 79) % 997;
        }
        total
    }
}

fn main() {
    let obj = HybridHandler::new(79);
    println!("{}", obj.build_scheduler(79));
}
