struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("Hello, world!, {}", area(&rect1));
}

fn area(rectanggle: &Rectangle) -> u32 {
    rectanggle.width * rectanggle.height
}
