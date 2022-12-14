use std::{io::{self, BufRead}, fs::File, path::Path};

fn main() {
    if let Ok(lines) = read_lines("./input.txt"){
        let mut current_max: i32 = 0;
        let mut current_int: i32 = 0;
        let mut int_array: Vec<i32> = Vec::new();
        for line in lines {
            if let Ok(ip) = line {
                //Aqui va la logica
                if let Ok(num_line) = ip.parse::<i32>(){
                    current_int += num_line;
                }
                else {
                    int_array.push(current_int);
                    if current_int > current_max {
                        current_max = current_int;
                    }
                    current_int = 0;
                }
            }
        }
        int_array.sort();
        int_array.reverse();
        println!("{} {} {}", int_array[0], int_array[1], int_array[2]);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}