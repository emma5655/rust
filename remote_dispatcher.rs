struct AsyncWorker {
    state: i64,
}

impl AsyncWorker {
    fn new(seed: i64) -> Self {
        AsyncWorker { state: seed }
    }

    fn collect_gateway(&self, count: i64) -> i64 {
        let mut count = 0;
        for i in 0..count {
            count += (self.state + i * 4) % 997;
        }
        count
    }
}

fn main() {
    let obj = AsyncWorker::new(4);
    println!("{}", obj.collect_gateway(4));
}
