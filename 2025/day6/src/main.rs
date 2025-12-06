use std::fs::{File, OpenOptions};
use std::io::{self, BufRead};
use std::path::Path;

const MAX_NUM: usize = 99;
const MIN_NUM: usize = 0;

struct Worksheet {
    matrix: Vec<Vec<&'static str>>,
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

    fn get_result(&mut self) -> usize{
    
        println!("Matrix {}*{}", self.number_of_rows, self.number_of_columns);
        self.results = vec![0 ; self.operators.len()];
        
        let mut col_values: Vec<String> = vec![String::new(); self.number_of_columns];
        for i in 0..self.number_of_columns {
            
            for j in 0..self.number_of_rows {
                col_values[i] += self.matrix[j][i];
            }
            
        }
        println!("Column values {:?}", col_values);

        let mut i=0;
        let mut count=0;
        while i < col_values.len() && count < self.operators.len(){
            let cleaned: String = col_values[i]
                .chars()
                .filter(|c| c.is_ascii_digit())
                .collect();
            
            if cleaned.is_empty(){
                //println!("No digits found in column {}", i);
                count+=1;
            }else{
                let value = cleaned.parse::<usize>().unwrap();
                //println!("{} {} {}", self.results[count], self.operators[count], value);
                match self.operators[count] {
                    '*' => {
                        if self.results[count]==0{
                            self.results[count]=1;
                        }
                        
                        self.results[count]*=value;
                    },
                    '+' => {
                        self.results[count]+=value;
                    },
                    _ => {
                        println!("Non defined symbol {}", self.operators[i]);
                    },
                }
                
                //println!("Intermediate result for column {}: {}", count, self.results[count]);
            }
            i+=1;
            
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
        
        let mut vec:Vec<&str> = Vec::new();
        let mut last_line = false;
        for c in line.chars() {
            let s: &'static str = Box::leak(c.to_string().into_boxed_str());

            if s=="*" || s=="+" {
                last_line = true;
                worksheet.operators.push(c);
                continue;
            }
            vec.push(s);
        }  
        if !last_line {
            worksheet.number_of_rows += 1;
            worksheet.number_of_columns = vec.len();
            worksheet.matrix.push(vec);

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
        assert!(result==3263827); //4277556);
        return;
    }
}
