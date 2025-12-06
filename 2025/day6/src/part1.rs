use std::fs::{File, OpenOptions};
use std::io::{self, BufRead};
use std::path::Path;

const MAX_NUM: usize = 99;
const MIN_NUM: usize = 0;

struct Worksheet {
    matrix: Vec<Vec<usize>>,
    operators: Vec<char>,
    results: Vec<usize>,
    number_of_rows: usize,
    number_of_columns: usize,
}
impl Worksheet {
    fn new() -> Self {
        Worksheet {
            matrix: Vec::new(),
            operators: Vec::new(),
            number_of_rows: 0,
            number_of_columns: 0,
            results: Vec::new(),
        }
    }

    fn get_result_per_column(&mut self) -> usize{
    
        println!("Matrix {}*{}", self.number_of_rows, self.number_of_columns);
        self.results = vec![0;self.number_of_columns];
        println!("results {:?}", self.results);
        for i in 0..self.number_of_columns {
            match self.operators[i] {
                '*' => {
                    self.results[i] = 1;
                    for j in 0..self.number_of_rows {
                        self.results[i] *= self.matrix[j][i];
                    }
              
                },
                '+' => {
                    for j in 0..self.number_of_rows {
                        self.results[i] += self.matrix[j][i];
                    }
                },
                _ => {
                    println!("Non defined symbol {}", self.operators[i]);
                },
            }
            
        }
        println!("results {:?}", self.results);
        
        self.results.iter().sum::<usize>()
    }

                
}



pub fn read_document(file_path: &str) -> io::Result<Worksheet> {
    let path = Path::new(file_path);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut worksheet = Worksheet::new();
    
    
    for line in reader.lines() {
        let line = line?;
        println!("Read line: {}", line);
        
        let mut v:Vec<usize> = Vec::new();
        let mut last_line = false;
        for num_str in line.split_whitespace() {
            if let Ok(num) = num_str.parse::<usize>() {
                v.push(num);
            }else if num_str.len() == 1 {
                let op = num_str.chars().next().unwrap();
                worksheet.operators.push(op);
                last_line = true;
            }
            
        } 
        if !last_line {
            worksheet.number_of_rows += 1;
            worksheet.number_of_columns = v.len();
            worksheet.matrix.push(v);

        }else{
            println!("Operators: {:?}", worksheet.operators);
        }
    }

    println!("Total items processed: {}*{}={}", 
        worksheet.matrix.len(), worksheet.matrix[0].len(), worksheet.number_of_rows * worksheet.number_of_columns);
    Ok(worksheet)
}


fn main() {

    let file_path = std::env::args().nth(1).expect("no pattern given");
    

    let mut worksheet = read_document(&file_path).expect("Failed to read document");
    println!("Worksheet loaded successfully {:?}", worksheet.matrix);
    let result = worksheet.get_result();
    println!("Final result: {}", result);

    if file_path=="test"{
        assert!(result==4277556);
        return;
    }
}
