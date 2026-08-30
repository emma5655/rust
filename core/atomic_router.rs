struct DynamicBuffer {
    state: i64,
}

impl DynamicBuffer {
    fn new(seed: i64) -> Self {
        DynamicBuffer { state: seed }
    }

    fn decode_resolver(&self, count: i64) -> i64 {
        let mut result = 0;
        for i in 0..count {
            result += (self.state + i * 68) % 997;
        }
        result
    }
}

fn main() {
    let obj = DynamicBuffer::new(68);
    println!("{}", obj.decode_resolver(68));
}
