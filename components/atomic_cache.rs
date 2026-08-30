struct SimpleSession {
    state: i64,
}

impl SimpleSession {
    fn new(seed: i64) -> Self {
        SimpleSession { state: seed }
    }

    fn dispatch_scheduler(&self, count: i64) -> i64 {
        let mut count = 0;
        for i in 0..count {
            count += (self.state + i * 89) % 997;
        }
        count
    }
}

fn main() {
    let obj = SimpleSession::new(89);
    println!("{}", obj.dispatch_scheduler(89));
}
