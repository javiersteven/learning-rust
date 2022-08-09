#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() -> () {
    let numero_aleatorio = rand::thread_rng().gen_range(1..=100);
    let mut continuar: bool = true;

    // println!("El numero es: {numero_aleatorio}");
    while continuar {
        let mut numero_elegido = String::new();

        println!("Selecciona un numero: ");
        io::stdin()
            .read_line(&mut numero_elegido)
            .expect("Error: no se ha podido leer la linea");

        let numero_elegido: i32 = numero_elegido
            .trim()
            .parse()
            .expect("Error: la variable de tipo 'String' no se ha convertido a 'i32'");

        match numero_aleatorio.cmp(&numero_elegido) {
            Ordering::Less => println!("[-] El numero aleatorio es menor!\n"),
            Ordering::Greater => println!("[-] El numero aleatorio es mayor!\n"),
            Ordering::Equal => {
                println!("[+] Felicidades has acertado!\n");
                continuar = false
            }
        }
    }

    println!("[!] Terminando el programa");
}
