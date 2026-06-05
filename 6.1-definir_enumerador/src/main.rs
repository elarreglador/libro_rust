// #[derive(Debug)] para poder imprimirlo
#[derive(Debug)]
// las opciones no son strings, son valores
enum ModoRed {
    Wifi,
    Ethernet,
    Desconectado,
}

fn main() {
    // Definimos una variable de tipo ModoRed asignándole una sola opcion de enum ModoRed
    // mi_conexion -> nombre de la variable.
    // ModoRed -> El tipo (la categoría a la que pertenece).
    // Wifi -> La variante (el estado específico que has elegido).
    let mi_conexion = ModoRed::Wifi;

    // Imprimimos el valor usando el formato debug {:?}
    println!("El modo de red actual es: {:?}", mi_conexion);

}
