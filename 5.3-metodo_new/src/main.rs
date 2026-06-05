// Estructura rectangle
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// funciones asociadas a Rectangle
impl Rectangle {
    // metodo que crea un cuadrado
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

// main
fn main() {
    // creamos el cuadrado de 3 de lado
    // se usa :: cuando queremos acceder a un metodo estatico (no pertenece auna instancia sino a la estructura)
    let sq = Rectangle::square(3);

    println!("sq: {:?}", sq)
}
