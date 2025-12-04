use std::fs::{File, OpenOptions};
use std::io::{self, BufRead};
use std::path::Path;

struct Diagram {
    matrix: Vec<Vec<&'static str>>,
    number_of_rows: usize,
    number_of_columns: usize,
    accessible_rolls: usize,
}

impl Diagram {
    fn new() -> Self {
        Diagram {
            number_of_rows: 0,
            number_of_columns: 0,
            matrix: Vec::new(),
            accessible_rolls: 0,
        }
    }

    fn print_matrix(&self){
        for i in 0..self.number_of_rows {
            println!("{:?}", self.matrix[i]);
        }
    }
    fn number_of_accessible_rolls_one_step(&mut self){
        
        for i in 0..self.number_of_rows {
            
            //println!("Line: {:?}", self.matrix[i]);
            for j in 0..self.number_of_columns {
                if self.matrix[i][j] == "@" {
                    //rolls (r,c) has 8 neighbors : (r-1, c-1) (r-1,c) (r-1,c+1) (r,c-1) (r,c+1) (r+1,c-1) (r+1,c) (r+1,c+1)
                    let r = i as isize;
                    let c = j as isize;
                    let adjacent_rolls:Vec<(isize, isize)> = vec![(r-1, c-1), (r-1,c), (r-1,c+1), (r,c-1), (r,c+1), (r+1,c-1), (r+1,c), (r+1,c+1)];
                    let mut k=0; 
                    let mut number_adjacent_rolls=0;
                    while number_adjacent_rolls < 4 && k < 8{
                        let coords = adjacent_rolls[k];
                        if coords.0 >=0 && coords.1>=0 {
                            let x= coords.0 as usize;
                            let y= coords.1 as usize;
                            if x < self.number_of_rows && y < self.number_of_columns {
                                if self.matrix[x][y] == "@" {
                                    number_adjacent_rolls += 1;
                                }
                            }
                        }
                        
                        k+=1;
                    }
                    if number_adjacent_rolls < 4 {
                        self.accessible_rolls +=1;
                        println!("coord({},{})", i, j);
                    }
                }
                //println!("{} accessible rolls", self.accessible_rolls);
                
            }
        }
    }

    fn number_of_accessible_rolls(&mut self, mut step:usize){
        let mut accessible_rolls:Vec<(usize, usize)> = Vec::new();
        for i in 0..self.number_of_rows {
            //println!("Line: {:?}", self.matrix[i]);
            for j in 0..self.number_of_columns {
                if self.matrix[i][j] == "@" {
                    //rolls (r,c) has 8 neighbors : (r-1, c-1) (r-1,c) (r-1,c+1) (r,c-1) (r,c+1) (r+1,c-1) (r+1,c) (r+1,c+1)
                    let r = i as isize;
                    let c = j as isize;
                    let adjacent_rolls:Vec<(isize, isize)> = vec![(r-1, c-1), (r-1,c), (r-1,c+1), (r,c-1), (r,c+1), (r+1,c-1), (r+1,c), (r+1,c+1)];
                    let mut k=0; 
                    let mut number_adjacent_rolls=0;
                    while number_adjacent_rolls < 4 && k < 8{
                        let coords = adjacent_rolls[k];
                        if coords.0 >=0 && coords.1>=0 {
                            let x= coords.0 as usize;
                            let y= coords.1 as usize;
                            if x < self.number_of_rows && y < self.number_of_columns {
                                if self.matrix[x][y] == "@" {
                                    number_adjacent_rolls += 1;
                                }
                            }
                        }
                        
                        k+=1;
                    }
                    if number_adjacent_rolls < 4 {
                        self.accessible_rolls +=1;
                        accessible_rolls.push((i,j));
                        //println!("coord({},{})", i, j);
                    }
                }
            }
        }
        println!("{} accessible rolls", accessible_rolls.len());
        //self.print_matrix();
        // clear accessible rolls
        for coord in &accessible_rolls {
            self.matrix[coord.0][coord.1] = "x";
        }
        if accessible_rolls.len() > 0 {
            step +=1;
            self.number_of_accessible_rolls(step);
        }else{
            println!("Total steps {}", step);
        }

    }


}



pub fn read_document(file_path: &str) -> io::Result<Diagram> {
    let path = Path::new(file_path);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut diagram = Diagram::new();

    for line in reader.lines() {
        let line = line?;
        diagram.number_of_rows += 1;
        //println!("Read line: {}", line);
        
        diagram.number_of_columns = line.len();
        let mut vec=Vec::new();

        //transform line into vector of &str
        for c in line.chars() {
            let s: &'static str = Box::leak(c.to_string().into_boxed_str());
            vec.push(s);
        } 
        
        diagram.matrix.push(vec);
    }
    println!("Total items processed: {}*{}={}", 
        diagram.matrix.len(), diagram.matrix[0].len(), diagram.number_of_rows * diagram.number_of_columns);
    Ok(diagram)
}


fn main() {

    let file_path = std::env::args().nth(1).expect("no pattern given");

    let mut diagram = read_document(&file_path).expect("Failed to read document");
    //println!("Finished reading document: {:?}", diagram.matrix);
    diagram.number_of_accessible_rolls(0);
    println!("Results: {:?}", diagram.accessible_rolls);
   

}
