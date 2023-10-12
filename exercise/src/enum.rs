fn main() {
    let m1 = Message::Plain("Hello world".to_string());
    let m2 = Message::Mixed{x: 42, m: "I am mixed".to_string()};
    let m3 = Message::Numerical(32, 52);

    println!("{:?}", m1);
    println!("{:?}", m2);
    println!("{:?}", m3);
    m1.test();
    m2.test();
    m3.test();
}

impl Message {
    fn test(&self) {
        println!("In testing {:?}", self);
    }
}

#[derive(Debug)]
enum Message {
    Plain(String),
    Mixed{x: i32, m: String},
    Numerical(i32, i32)
}
