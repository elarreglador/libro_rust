// Creamos la estructura
struct Rectangle {
    width: u32,
    height: u32
}

// Agregamos las funciones asociadas a la estructura
impl Rectangle {
    fn area (&self) -> u32 {
        self.width * self.height
    }

    fn can_hold (&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
        
    
// main
fn main() {
    // Creamos tres instancias de rectangle
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    // como el rectangulo 2 es menor que el 1 el valor es true
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    // como el 3 es mas ancho que el 1, el valor es false
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
