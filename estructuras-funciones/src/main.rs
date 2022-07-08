use std::vec::Vec;
use std::io::stdin;

fn sum(a: i32, b: i32) -> i32 {
    return a + b
}

fn sum_mutable(a: &mut i32, b: i32) {
    *a = *a + b
}

fn main() {
    println!("Introduce 3 nombres");

    let mut nombres: Vec<String>= Vec::new();
    for _ in 0..3 {
        let mut nombre = String::new();
        stdin().read_line(&mut nombre).unwrap();
        nombres.push(nombre.trim().to_string());
    }

    println!("{:?}", nombres);
    println!("Primer nombre: {}", nombres[0]);
    println!("Cantidad de valores: {}", nombres.len());

    for nombre in nombres {
        println!("El nombre es {}", nombre);
    }

    let hola = ["H", "o", "l", "a"];
    println!("Array inmutable {:?}", hola);

    println!("7+8={}", sum(7, 8));

    let mut num = 0;
    sum_mutable(&mut num, 15);
    println!("0+15 = {}", num);
}
