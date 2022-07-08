fn main() {
    // Variables
    let edad : u8 = 28;
    let nombre: String = "Carlos".to_string();
    println!("Hola soy {} y tengo {} años", nombre, edad);

    // Entrada de datos
    println!("Ingrese su nombre: ");
    let mut nombre_usuario: String = String::new();
    std::io::stdin().read_line(&mut nombre_usuario).unwrap();

    println!("Ingrese su edad: ");
    let mut edad_usuario: String = String::new();
    std::io::stdin().read_line(&mut edad_usuario).unwrap();
    let edad_usuario_int: u8 = edad_usuario.trim().parse().unwrap();

    println!("Hola {}. Tienes {} de edad", nombre_usuario.trim(), edad_usuario_int);

    // Condicionales
    if edad_usuario_int >= 18 {
        println!("Eres mayor de edad");
    } else {
        println!("Eres menor de edad");
    }

    // Ciclo Loop
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        if input.trim().eq("1234") {
            break;
        }
        println!("Ingrese una contraseña correcta");
    }
}
