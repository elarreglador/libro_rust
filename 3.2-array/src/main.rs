use std::io;


// Funciona si no pides un valor superior a index = 4
// Si pides el valor 5 Rust no permite acceder a una posicion 
// de memoria no valida, evitando asi un problema de seguridad.

fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
