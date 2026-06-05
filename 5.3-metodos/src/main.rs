# [derive(Debug)]

// Structu seria similar a una clase
struct Rectangle {
    width: u32,
    height: u32,
}

// la implementacion o metodo de la clase Rectangle
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

// main
fn main() {

    // Creamos una instancia de la clase Rectangle
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    // Mostramos su area
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}
