fn main() {
    let m = FloatIter::new(1.0, 0.1, 2.0)
        .into_iter()
        //here x is converted to String from float
        .map(|x| format!("{x:.1}"))
        //here x is converted to formatted float from string
        .filter_map(|x| x.parse::<f64>().ok())
        .collect::<Vec<_>>();

    for i in &m {
        println!("{:?}", i);
    }

    for i in FloatIter::new(2.0, 0.3, 3.0).into_iter() {
        println!("Square root {i:.1}");
    }
}
#[derive(Debug)]
struct FloatIter {
    start: f64,
    incr: f64,
    end: f64,
}
impl FloatIter {
    fn new(start: f64, incr: f64, end: f64) -> Self {
        Self { start, incr, end }
    }
}
impl Iterator for FloatIter {
    type Item = f64;
    fn next(&mut self) -> Option<Self::Item> {
        let end = self.start;
        if end >= self.end {
            return None;
        } else {
            self.start += self.incr;
            Some(end)
        }
    }
}
