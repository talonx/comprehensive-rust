struct Rectangle {
    width: u32,
    height: u32,


}

impl Rectangle {
    fn area(&self) -> u32 {
        return self.width * self.height;
    }

    fn inc_width(&mut self, delta: u32) {
        self.width += delta;
    }

    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle{width: width, height: height}
    }

    fn square(width: u32) -> Rectangle {
        Rectangle{width: width, height: width}
    }
}


fn main() {
    //let mut rec = Rectangle{width: 30, height: 40};
    let mut rec = Rectangle::new(30, 40);
    println!("old area {}", rec.area());
    rec.inc_width(5);
    println!("old area {}", rec.area());
}
