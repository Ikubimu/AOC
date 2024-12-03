use std::fs::File;
use std::io::{self, BufRead};
use std::os::linux::raw;

fn main() {
    let mut raw_data = match convert_input("../input.txt") {
        Ok((list1, list2)) => {
            (list1, list2)
        }
        Err(e) => {
            eprintln!("Error al procesar el archivo: {}", e);
            return; // O puedes manejar el error de otra forma si lo prefieres
        }
    };

    // println!("lista1: {:?}", raw_data.0);
    // println!("lista2: {:?}", raw_data.1);
    let mut sol=0;
    for i in 0..raw_data.0.len() {
        let num = raw_data.0[i];
        let count:i32 = raw_data.1.iter().filter(|&&x| x == num).count().try_into().unwrap_or(-1);
        let mul = num*count;
        sol += mul;
    }
    println!("lista2: {:?}", sol);
}


fn convert_input(path: &str) -> Result<(Vec<i32>, Vec<i32>), io::Error> 
{
    let archivo = File::open(path)?;
    let lector = io::BufReader::new(archivo);
    let mut list1:Vec<i32> = Vec::new();
    let mut list2:Vec<i32> = Vec::new();

    // Leer cada línea del archivo
    for linea in lector.lines() {
        let linea = linea?; // Manejar errores en la lectura

        let partes: Vec<&str> = linea.split_whitespace().collect();
        if let (Ok(x), Ok(y)) = (partes[0].parse::<i32>(), partes[1].parse::<i32>()) {
            list1.push(x); // Agregar el valor de x al vector de x
            list2.push(y); // Agregar el valor de y al vector de y
        } else {
            eprintln!("Error al convertir los valores en la línea: {}", linea);
        }
    }

    Ok((list1, list2))
}