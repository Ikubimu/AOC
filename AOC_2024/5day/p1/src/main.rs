use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let path = "../input.txt";
    let archivo = File::open(path)?;
    let lector = io::BufReader::new(archivo);

    let mut indexes : Vec<String> = Vec::new();
    let mut orderes : Vec<Vec<String>> = Vec::new();

    let mut second_part = false;
    for linea in lector.lines()
    {
        let linea = linea?;
        if linea == ""
        {
            second_part = true;
            continue;
        }
        else if second_part
        {
            let partes: Vec<String> = linea.split(',').map(|s| s.to_string()).collect();
            orderes.push(partes);
        }
        else {
            indexes.push(linea);
        }
    }

    //println!("indexes {:?}", indexes);
    //println!("orderes {:?}", orderes);

    let mut sol = 0;
    for i in 0..orderes.len()
    {
        sol += check_order(&orderes[i], &indexes);
    }

    println!("Sol: {}", sol);


    Ok(())
}

fn check_order(order: &Vec<String>, indexes: &Vec<String>) -> i32
{
    let mut check_sol = true;
    for i in 0..order.len()-1
    {
        for j in (i+1)..order.len()
        {
            let key_1 = format!("{}|{}", order[i], order[j]);
            let key_2 = format!("{}|{}", order[j], order[i]);
            //println!("i: {} j: {}", key_1, key_2);
            if indexes.contains(&key_1) && !indexes.contains(&key_2)
            {
                continue;
            }
            check_sol = false;
            break;
        }
        if !check_sol
        {
            break;
        }
    }

    if check_sol
    {
        let mid = order.len()/2;
        let number: i32 = order[mid].parse().expect("Error conversion");
        return number;
    }
    return 0;
}
