use std::fs::{File, OpenOptions};
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use rayon::prelude::*;

struct Tachyonsheet {
    matrix: Vec<Vec<char>>,
    number_of_possibilities: usize,
    number_of_rows: usize,
    number_of_columns: usize,
    beams: Vec<(usize, usize)>, //teachyons beams positions
    max_splits: usize,
    memo: Arc<Mutex<HashMap<(usize,usize), usize>>>, // added memoization for parallel recursion
}

impl Tachyonsheet {
    fn new() -> Self {
        Tachyonsheet {
            matrix: Vec::new(),
            number_of_possibilities: 0,
            number_of_rows: 0,
            number_of_columns: 0,
            beams: Vec::new(),
            max_splits: 0,
            memo: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    fn count_splits(&mut self){
    
        println!("Matrix {}*{}", self.number_of_rows, self.number_of_columns);
        
        let mut source = (0,0);
        for i in 0..self.number_of_rows {
            for j in 0..self.number_of_columns {
                if self.matrix[i][j]=="S" {
                    println!("Started the beam at position ({},{})", i, j);
                    source = (i,j);
                    break;
                }
            }
        }
        self.number_of_possibilities = self.count_paths(source); //self.position_beam(source, 0);
        for i in 0..self.number_of_rows {
            println!("{:?}", self.matrix[i]);
        }
        
    }

    
    //count all possible paths from S to the bottom (parallelized)
    fn count_paths(&self, position: (usize,usize)) -> usize {
        let (x, y) = position;

        // bounds check
        if x >= self.number_of_rows || y >= self.number_of_columns {
            return 0;
        }

        // terminal row -> one path
        if x == self.number_of_rows - 1 {
            return 1;
        }

        // check memo
        if let Some(&v) = self.memo.lock().unwrap().get(&position) {
            return v;
        }

        let symbol = self.matrix[x][y];
        let result = match symbol {
            'S' | '|' | '.' => {
                // single continuation down
                self.count_paths((x + 1, y))
            }
            '^' => {
                // split: compute left and right in parallel
                let left_closure = || {
                    if y == 0 {
                        0
                    } else {
                        self.count_paths((x, y - 1))
                    }
                };
                let right_closure = || {
                    if y + 1 >= self.number_of_columns {
                        0
                    } else {
                        self.count_paths((x, y + 1))
                    }
                };
                let (left, right) = rayon::join(left_closure, right_closure);
                left + right
            }
            _ => 0,
        };

        // store in memo and return
        self.memo.lock().unwrap().insert(position, result);
        result
    }
  
}


pub fn read_document(file_path: &str) -> io::Result<Tachyonsheet> {
    let path = Path::new(file_path);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut tachyonsheet = Tachyonsheet::new();
    
    
    for line in reader.lines() {
        let line = line?;
        println!("Read line: {}", line);
        
        let mut vec: Vec<char> = Vec::new();
        
        for c in line.chars() {
            if c == '^'  {
                tachyonsheet.max_splits += 1;
            }
            vec.push(c);
        } 
        
        tachyonsheet.number_of_rows += 1;
        tachyonsheet.number_of_columns = vec.len();
        tachyonsheet.matrix.push(vec);
    }

    println!("Total items processed: {}*{}={} with maw splits {}", 
        tachyonsheet.matrix.len(), tachyonsheet.matrix[0].len(), 
        tachyonsheet.number_of_rows * tachyonsheet.number_of_columns, 
        tachyonsheet.max_splits);
    Ok(tachyonsheet)
}


fn main() {

    let file_path = std::env::args().nth(1).expect("no pattern given");
    

    let mut tachyonsheet = read_document(&file_path).expect("Failed to read document");
    println!("tachyonsheet loaded successfully {:?}", tachyonsheet.matrix);

    tachyonsheet.count_splits();
    let result = tachyonsheet.number_of_possibilities;
    println!("Final result: {}", result);

    if file_path=="test"{
        let answer = 40;
        assert!(result==answer); 
        return;
    }
}
