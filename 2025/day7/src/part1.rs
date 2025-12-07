use std::fs::{File, OpenOptions};
use std::io::{self, BufRead};
use std::path::Path;

const MAX_NUM: usize = 99;
const MIN_NUM: usize = 0;

struct Tachyonsheet {
    matrix: Vec<Vec<&'static str>>,
    number_of_splits: usize,
    number_of_rows: usize,
    number_of_columns: usize,
    beams: Vec<(usize, usize)>, //teachyons beams positions
    max_splits: usize,
}
impl Tachyonsheet {
    fn new() -> Self {
        Tachyonsheet {
            matrix: Vec::new(),
            number_of_splits: 0,
            number_of_rows: 0,
            number_of_columns: 0,
            beams: Vec::new(),
            max_splits: 0,
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
                    self.number_of_splits = self.position_beam(source, 0);
                }
            }
        }
        
        for i in 0..self.number_of_rows {
            println!("{:?}", self.matrix[i]);
        }
        
    }

    fn position_beam(&mut self, position: (usize,usize), mut number_of_splits: usize) -> usize{
        let (mut x, mut y) = position;
        print!("Positioning beam at ({},{}) ", x, y);
        if x < self.number_of_rows && y < self.number_of_columns {
           
            let pos = (x,y);
            let symbol = self.matrix[x][y];
            print!("{} ", symbol);
            if  symbol == "S" {
                let pos1 = (pos.0+1, pos.1);

                if pos1.0 < self.number_of_rows && pos1.1 < self.number_of_columns {
                    number_of_splits += self.position_beam(pos1, 0);
                }
            }
            else if symbol == "." {
                self.matrix[pos.0][pos.1] = "|";
                print!("| at {:?} ", pos);
                self.beams.push(pos);
                
                let pos1 = (pos.0+1, pos.1);

                if pos1.0 < self.number_of_rows && pos1.1 < self.number_of_columns {
                    number_of_splits += self.position_beam(pos1, 0);
                }

            }
            else if symbol == "^" {
                number_of_splits +=1;

                let pos1 = (pos.0, pos.1-1);
                let pos2 = (pos.0, pos.1+1);

                if pos1.0 < self.number_of_rows && pos1.1 < self.number_of_columns {
                    self.beams.push(pos1);
                    number_of_splits += self.position_beam(pos1, 0);
                
                }
                
                if pos2.0 < self.number_of_rows && pos2.1 < self.number_of_columns {
                    self.beams.push(pos2);
                    number_of_splits += self.position_beam(pos2, 0);
                }
                
            }
        }
        println!("Number of splits: {}", number_of_splits);
        return number_of_splits;
        
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
        
        let mut vec:Vec<&str> = Vec::new();
        
        for c in line.chars() {
            let s: &'static str = Box::leak(c.to_string().into_boxed_str());

            if s=="^"  {
                tachyonsheet.max_splits += 1;
            }
            vec.push(s);
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
    let result = tachyonsheet.number_of_splits;
    println!("Final result: {}", result);

    if file_path=="test"{
        let answer = 21;
        assert!(result==answer); 
        return;
    }
}
