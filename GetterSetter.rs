fn main() {
    let mut p = Point::new();

    //Explicitly passing
    p.setX(1.2);
    p.setY("Hello".to_string());
    p.setZ('a');

    println!("{} {} {} ", p.getX(), p.getY(), p.getZ());

    let F64 = 56.7;
    let string = String::from("Helloooo");
    let Char = 'c';

    let p = Point::new1(F64, string, Char);
    println!("{} {} {} ", p.getX(), p.getY(), p.getZ());
}
struct Point {
    x: f64,
    y: String,
    z: char,
}
impl Point {
//Static function
    fn new() -> Self {
        //explcitly setting dafault values.
        Self {
            x: Default::default(),
            y: Default::default(),
            z: Default::default(),
        }
    }
  //Another static function on purpose.
    //explcitly getting inputs from outside.
    fn new1(x: f64, y: String, z: char) -> Self {
        Self { x, y, z }
    }

    fn setX(&mut self, val: f64) {
        self.x = val;
    }
    fn setY(&mut self, val: String) {
        self.y = val;
    }
    fn setZ(&mut self, val: char) {
        self.z = val;
    }
    fn getX(&self) -> f64 {
        self.x
    }
    fn getY(&self) -> &str {
        &self.y
    }
    fn getZ(&self) -> char {
        self.z
    }
}
