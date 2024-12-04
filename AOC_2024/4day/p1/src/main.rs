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
            if car == 'X'
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

    let cadena = String::from("MAS");
    let max_x = matrix[0].len() as i32;
    let max_y = matrix.len() as i32;

    let mut sol = 0;

    let mut strs : Vec<String> = Vec::new();

    if x+3 < max_x
    {
        let mut str = String::from("");
        str.push(matrix[(y) as usize][(x+1) as usize]);
        str.push(matrix[(y) as usize][(x+2) as usize]);
        str.push(matrix[(y) as usize][(x+3) as usize]);
        strs.push(str);
    }

    if x-3 >= 0
    {
        let mut str = String::from("");
        str.push(matrix[(y) as usize][(x-1) as usize]);
        str.push(matrix[(y) as usize][(x-2) as usize]);
        str.push(matrix[(y) as usize][(x-3) as usize]);
        strs.push(str);
    }

    if y+3 < max_y
    {
        let mut str = String::from("");
        str.push(matrix[(y+1) as usize][(x) as usize]);
        str.push(matrix[(y+2) as usize][(x) as usize]);
        str.push(matrix[(y+3) as usize][(x) as usize]);
        strs.push(str);
    }

    if y-3 >= 0
    {
        let mut str = String::from("");
        str.push(matrix[(y-1) as usize][(x) as usize]);
        str.push(matrix[(y-2) as usize][(x) as usize]);
        str.push(matrix[(y-3) as usize][(x) as usize]);
        strs.push(str);
    }

    if y-3 >= 0 && x-3 >= 0
    {
        let mut str = String::from("");
        str.push(matrix[(y-1) as usize][(x-1) as usize]);
        str.push(matrix[(y-2) as usize][(x-2) as usize]);
        str.push(matrix[(y-3) as usize][(x-3) as usize]);
        strs.push(str);
    }

    if y+3 < max_y && x+3 < max_x
    {
        let mut str = String::from("");
        str.push(matrix[(y+1) as usize][(x+1) as usize]);
        str.push(matrix[(y+2) as usize][(x+2) as usize]);
        str.push(matrix[(y+3) as usize][(x+3) as usize]);
        strs.push(str);
    }

    if y+3 < max_y && x-3 >= 0
    {
        let mut str = String::from("");
        str.push(matrix[(y+1) as usize][(x-1) as usize]);
        str.push(matrix[(y+2) as usize][(x-2) as usize]);
        str.push(matrix[(y+3) as usize][(x-3) as usize]);
        strs.push(str);
    }

    if y-3 >= 0 && x+3 < max_x
    {
        let mut str = String::from("");
        str.push(matrix[(y-1) as usize][(x+1) as usize]);
        str.push(matrix[(y-2) as usize][(x+2) as usize]);
        str.push(matrix[(y-3) as usize][(x+3) as usize]);
        strs.push(str);
    }

    for i in 0..strs.len()
    {
        if strs[i] == cadena{
            sol+=1;
        }
    }


    return sol;
}
