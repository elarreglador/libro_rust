#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "El area es de {}",
        area(&rect1)
    );

    // Esto solo funciona porque hemos puesto #[derive(Debug)] arriba
    println!("rect1 en una linea: {:?}", rect1);
    println!("rect en varias lineas: {:#?}",rect1);

    // dbg! imprime en el flujo de consola de error estándar (stderr)
    dbg!(&rect1);

}
