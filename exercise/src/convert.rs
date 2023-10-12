fn multiply(x: i16, y:i16) -> i16 {
    x * y
}

fn main() {
    let x: i32 = 15;
    let y: i16 = 1000;

    let z = x.into();

    println!("{x} * {y} = {}", multiply(z, y));
}
