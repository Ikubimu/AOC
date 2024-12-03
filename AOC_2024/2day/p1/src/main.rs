use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let raw_data = match convert_input("../input.txt") {
        Ok(list1) => {
            list1
        }
        Err(e) => {
            eprintln!("Error al procesar el archivo: {}", e);
            return; // O puedes manejar el error de otra forma si lo prefieres
        }
    };

    let mut sol:u32 = 0;

    for i in 590..600
    {
        if raw_data[i][0] > raw_data[i][1]
        {
            let mut check : bool = true;
            for j in 1..raw_data[i].len()
            {
                if (raw_data[i][j-1] > raw_data[i][j]) &&  ((raw_data[i][j-1]-raw_data[i][j])<=3)
                {
                    continue;
                }
                check = false;
                break;
            }
            if check
            {
                sol+=1;
            }
        }
        else if  raw_data[i][0] < raw_data[i][1]{
            let mut check : bool = true;
            for j in 1..raw_data[i].len()
            {
                if (raw_data[i][j-1] < raw_data[i][j]) &&  ((raw_data[i][j]-raw_data[i][j-1])<=3)
                {
                    continue
                }
                check = false;
                break;
            }
            if check
            {
                sol+=1;
            }
        }
        else {
            continue;
        }
    }

    println!("Sol {}", sol);
}



fn convert_input(path: &str) -> Result<Vec<Vec<i32>>, io::Error> 
{
    let archivo = File::open(path)?;
    let lector = io::BufReader::new(archivo);
    let mut list:Vec<Vec<i32>> = Vec::new();

    for linea in lector.lines() {
        let linea = linea?; // Manejar errores en la lectura

        let partes: Vec<&str> = linea.split_whitespace().collect();
        let mut elem:Vec<i32> = Vec::new();
        for i in 0..partes.len()
        {
            
            if let Ok(x) = partes[i].parse::<i32>()
            {
                elem.push(x);
            }
        }
        list.push(elem);
    }

    Ok(list)

}