fn main(){
    let mut p = Point::new();
    p.setX(1.2);
    p.setY(2.3);
    p.setZ(3.4);
    
    println!("{} {} {} ",p.getX(),p.getY(),p.getZ());
    
}
struct Point{
    x:f64,
    y:f64,
    z:f64
}
impl Point{
 fn new()->Self{
     Self{
         x:Default::default(),
         y:Default::default(),
         z:Default::default()
     }
 }

    fn setX(&mut self,val:f64){
        self.x=val;
    }
    fn setY(&mut self,val:f64){
        self.y = val;
    }
    fn setZ(&mut self,val:f64){
        self.z=val;
    }
    fn getX(&self)->f64{
        self.x
    }
    fn getY(&self)->f64{
        self.y
    }
    fn getZ(&self)->f64{
        self.z
    }
}
