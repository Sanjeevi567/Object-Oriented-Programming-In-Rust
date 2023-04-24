use std::io::prelude::*;
fn main() {
    let chiain = Chaining::new()
        .set_bool(true)
        .set_string("Chaining")
        .write_vec(&[1, 2, 3, 45])
        .end_chain();
}
struct Chaining {
    n1: bool,
    n2: String,
    n3: Vec<u8>,
}

impl Chaining {
    fn new() -> Self {
        Self {
            n1: Default::default(),
            n2: Default::default(),
            n3: Default::default(),
        }
    }
    fn set_bool(&mut self, b: bool) -> &mut Self {
        self.n1 = b;
        self
    }
    fn set_string(&mut self, s: &str) -> &mut Self {
        self.n2 = s.to_string();
        self
    }
    fn write_vec(&mut self, u: &[u8]) -> &mut Self {
        self.n3.write(u);
        self
    }
    fn end_chain(&mut self) {
        println!("{} {} {:?}", self.n1, self.n2, self.n3);
    }
}
