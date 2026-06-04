// Definimos la estructura User que actuará como nuestro modelo de datos
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

/// Función constructora para crear un nuevo usuario.
/// Toma el email y el username, y devuelve una instancia de User.
fn build_user(email: String, username: String) -> User {
    User {
        active: true,         // Por defecto, todos los usuarios nuevos están activos
        username,             // Usamos "field init shorthand" (equivale a username: username)
        email,                // Lo mismo para el email
        sign_in_count: 1,     // Inicializamos el contador de inicios de sesión en 1
    }
}

fn main() {
    // Instanciamos el usuario llamando a nuestra función constructora
    let user1 = build_user(
        String::from("pedro@mail.com"),
        String::from("Pedro"),
    );

    // Imprimimos los campos de la estructura individualmente
    // El macro println! permite formatear estos datos en la consola
    println!("{}", user1.active);
    println!("{}", user1.username);
    println!("{}", user1.email);
    println!("{}", user1.sign_in_count);
}
