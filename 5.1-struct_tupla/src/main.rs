struct Point(i32, i32, i32);

fn main() {
    let origen = Point(0, 0, 0);
    let destino = Point(10, 0, 0);

    // Calculamos la diferencia en el eje X (índice 0)
    let diferencia = destino.0 - origen.0; 
    println!("X = {}", diferencia);
}
