use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let path = "../input.txt";
    let archivo = File::open(path)?;
    let lector = io::BufReader::new(archivo);

    let mut matrix:Vec<Vec<char>> = Vec::new();
    let mut pos_X:Vec<(i32,i32)> = Vec::new();

    let mut i = 0;
    let mut j = 0;
    for linea in lector.lines()
    {
        i=0;
        let linea = linea?;
        let mut row:Vec<char> = Vec::new();
        for car in linea.chars()
        {
            row.push(car);
            if car == 'A'
            {
                pos_X.push((i,j));
            }
            i+=1;
        }
        matrix.push(row);
        j+=1;
    }
    let mut sol = 0;
    for i in 0..pos_X.len()
    {
        sol += check_matrix(&matrix, pos_X[i]);
    }

    println!("Sol: {}", sol);

    Ok(())
}


fn check_matrix(matrix: &Vec<Vec<char>>, pos: (i32, i32)) -> i32
{
    let x = pos.0;
    let y = pos.1;

    let cadena_1 = String::from("MAS");
    let cadena_2 = String::from("SAM");
    let max_x = matrix[0].len() as i32;
    let max_y = matrix.len() as i32;

    if y-1 >= 0 && x+1 < max_x && y+1 < max_y && x-1 >= 0
    {
        let mut str_1 = String::from("");
        let mut str_2 = String::from("");
        str_1.push(matrix[(y-1) as usize][(x-1) as usize]);
        str_1.push(matrix[(y) as usize][(x) as usize]);
        str_1.push(matrix[(y+1) as usize][(x+1) as usize]);
        
        str_2.push(matrix[(y+1) as usize][(x-1) as usize]);
        str_2.push(matrix[(y) as usize][(x) as usize]);
        str_2.push(matrix[(y-1) as usize][(x+1) as usize]);

        // println!("{:?} {:?}", str_1, str_2);

        if (str_1 == cadena_1 || str_1 == cadena_2) && (str_2 == cadena_1 || str_2 == cadena_2)
        {
            return 1
        }
    }

    return 0;
}
