use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let mut raw_data = match convert_input("../input.txt") {
        Ok(list1) => {
            list1
        }
        Err(e) => {
            eprintln!("Error al procesar el archivo: {}", e);
            return; // O puedes manejar el error de otra forma si lo prefieres
        }
    };

    let mut sol:u32 = 0;


    let mut i = 0;
    while i < raw_data.len()
    {
        
        let mut check = false;

        for j in 0..raw_data[i].len()
        {
            let mut order_vec = raw_data[i].clone();
            order_vec.remove(j);
            let mut order_pop = order_vec.clone(); 
            order_pop.sort();
            if order_vec == order_pop
            {
                for k in 1..order_vec.len()
                {
                    check = true;
                    if ((order_pop[k]-order_pop[k-1]).abs()>3 || order_pop[k] == order_pop[k-1])
                    {
                        check = false;
                        break;
                    }
                }
            }
            if check
            {
                sol+=1;
                break;
            }
            else {
                order_pop.sort_by(|a, b| b.cmp(a));
                if order_vec == order_pop
                {
                    for k in 1..order_vec.len()
                    {
                        check = true;
                        if ((order_pop[k]-order_pop[k-1]).abs()>3 || order_pop[k] == order_pop[k-1])
                        {
                            check = false;
                            break;
                        }
                    }
                }
            }
            if check{
                sol+=1;
                break;
            }
            
        }
        
        i+=1;
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