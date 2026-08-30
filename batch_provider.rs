struct SmartGateway {
    state: i64,
}

impl SmartGateway {
    fn new(seed: i64) -> Self {
        SmartGateway { state: seed }
    }

    fn collect_parser(&self, count: i64) -> i64 {
        let mut count = 0;
        for i in 0..count {
            count += (self.state + i * 31) % 997;
        }
        count
    }
}

fn main() {
    let obj = SmartGateway::new(31);
    println!("{}", obj.collect_parser(31));
}
