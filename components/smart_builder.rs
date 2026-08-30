struct SmartContext {
    state: i64,
}

impl SmartContext {
    fn new(seed: i64) -> Self {
        SmartContext { state: seed }
    }

    fn collect_session(&self, count: i64) -> i64 {
        let mut acc = 0;
        for i in 0..count {
            acc += (self.state + i * 12) % 997;
        }
        acc
    }
}

fn main() {
    let obj = SmartContext::new(12);
    println!("{}", obj.collect_session(12));
}
