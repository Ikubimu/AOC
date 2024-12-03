use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let path = "../input.txt";
    let archivo = File::open(path)?;
    let lector = io::BufReader::new(archivo);

    let mut control = 0;
    let mut vec:Vec<i32> = Vec::new();
    let mut idx_vec:Vec<i32> = Vec::new();

    let mut idx_en:Vec<usize> = Vec::new();
    let mut idx_un:Vec<usize> = Vec::new();

    let mut contenido = String::new();

    // Lee el archivo línea por línea
    for linea in lector.lines() {
        let linea = linea?;
        contenido.push_str(&linea); 
    }

    let mut idx = 0;

    control = 0;
    let mut num1 = String::from("");
    let mut num2 = String::from("");

    let idx_do: Vec<usize> = contenido.match_indices("do()")
        .map(|(i, _)| i)
        .collect();
    idx_en.extend(idx_do);

    let idx_do: Vec<usize> = contenido.match_indices("don't()")
        .map(|(i, _)| i)
        .collect();
    idx_un.extend(idx_do);


    for car in contenido.chars()
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
            idx_vec.push(idx);

        }

        idx+=1;
    }

   
    let mut arr_enables:Vec<Enabless> = Vec::new();

    for &i in &idx_en {
        arr_enables.push(Enabless {
            idx: i,
            mode: true,
        });
    }
    for &i in &idx_un {
        arr_enables.push(Enabless {
            idx: i,
            mode: false,
        });
    }

    arr_enables.sort_by(|a, b| a.idx.cmp(&b.idx));

    arr_enables.push(Enabless{
        idx: 999999,
        mode: false,
    });

    let mut i = 0;
    let mut sol = 0;
    let mut i_vec = 0;
    let mut i_do = 0;

    let mut into = false;
    while i<vec.len()
    {
        
        while arr_enables[i_do].idx < (idx_vec[i_vec] as usize)
        {
            i_do+=1;
            into = true;
        }
        if into
        {
            if arr_enables[i_do-1].mode
            {
                sol += vec[i]*vec[i+1];
            }
        }
        else 
        {
            sol += vec[i]*vec[i+1];
        }
        
        i_vec += 1;
        i+=2;
    }
    // println!("do: {:?}", idx_en);
    // println!("dont: {:?}", idx_un);
    // for i in 0..arr_enables.len()
    // {
    //     print!(" {} ", arr_enables[i].idx);
    // }
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

struct Enabless
{
    idx: usize,
    mode: bool
}

