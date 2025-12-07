use std::fs::{File, OpenOptions};
use std::io::{self, BufRead};
use std::path::Path;

use std::collections::HashMap;


struct Tachyonsheet {
    matrix: Vec<Vec<char>>,
    possibilities: HashMap<(usize,usize), usize>,
    number_of_rows: usize,
    number_of_columns: usize,
    beams: Vec<(usize, usize)>, //teachyons beams positions
    max_splits: usize,
}
impl Tachyonsheet {
    fn new() -> Self {
        Tachyonsheet {
            matrix: Vec::new(),
            possibilities: HashMap::new(),
            number_of_rows: 0,
            number_of_columns: 0,
            beams: Vec::new(),
            max_splits: 0,
        }
    }

    fn count_splits(&mut self) -> usize{
    
        println!("Matrix {}*{}", self.number_of_rows, self.number_of_columns);
        
        let mut source = (0,0);
        for i in 0..self.number_of_rows {
            for j in 0..self.number_of_columns {
                if self.matrix[i][j]=='S' {
                    println!("Started the beam at position ({},{})", i, j);
                    source = (i,j);
                    break;
                }
            }
        }
        
        self.count_paths(source)
    }

    //count all possible paths from S to the bottom
    fn count_paths(&mut self, position: (usize,usize)) -> usize{
        let (x, y) = position;
        let mut result: usize=0;
        print!("Counting paths at ({},{}) ", x, y);
        
        if let Some(&val) = self.possibilities.get(&position) {
            println!("-> found memoized: {}", val);
            return val;
        }
        
        if x < self.number_of_rows && y < self.number_of_columns {
            let symbol = self.matrix[x][y];
            if symbol == 'S' || symbol == '|' {
                result += self.count_paths((x + 1, y));
            } else if symbol == '^' {
                result += self.count_paths((x, y - 1)) + self.count_paths((x, y + 1));
            } else if symbol == '.' {
                result += self.count_paths((x + 1, y));
            }
        }
        
        
        if x == self.number_of_rows -1 {
            result = 1;
        }
        self.possibilities.insert(position, result);
        return result;
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

    let result = tachyonsheet.count_splits();
    println!("Final result: {}", result);

    if file_path=="test"{
        let answer = 40;
        assert!(result==answer); 
        return;
    }
}
