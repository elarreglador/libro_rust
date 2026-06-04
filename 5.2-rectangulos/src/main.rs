struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 10,
        height: 20,
    };

    println!(
        "El area del rectangulo es {} pixeles.",
        area(&rect1)
    );

    println!("Los lados son width: {} y height: {}",rect1.width, rect1.height);
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
