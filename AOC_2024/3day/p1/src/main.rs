use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let path = "../input.txt";
    let archivo = File::open(path)?;
    let lector = io::BufReader::new(archivo);

    let mut control = 0;
    let mut vec:Vec<i32> = Vec::new();

    for linea in lector.lines()
    {
        let linea = linea?;
        control = 0;
        let mut num1 = String::from("");
        let mut num2 = String::from("");

        for car in linea.chars()
        {
            let eval = control_func(car, control);
            if eval == 0
            {
                control = 0;
                num1.clear();
                num2.clear();
            }
            if eval == 1
            {
                control += 1;
            }
            if eval == 2
            {
                num1.push(car);
            }
            if eval == 3
            {
                num2.push(car);
            }
            if eval == 4
            {
                if let Ok(x) = num1.parse::<i32>()
                {
                    vec.push(x);
                }
                if let Ok(x) = num2.parse::<i32>()
                {
                    vec.push(x);
                }
                control = 0;
                num1.clear();
                num2.clear();

            }
        }
    }

    let mut i = 0;
    let mut sol = 0;
    while i<vec.len()
    {
        sol += vec[i]*vec[i+1];
        i+=2;
    }
    //println!("Sol: {:?}", vec);
    println!("Sol: {}", sol);

    Ok(())
}

fn control_func(car: char, idx: i32) -> i32
{

    match idx{
        0 if car == 'm' => 1,
        1 if car == 'u' => 1,
        2 if car == 'l' => 1,
        3 if car == '(' => 1,
        4 if car >= '0' && car <= '9' => 2,
        4 if car == ',' => 1,
        5 if car >= '0' && car <= '9' => 3,
        5 if car == ')' => 4,
        _=> 0,
    }
}

