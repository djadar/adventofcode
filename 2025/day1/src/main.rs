use std::fs::{File, OpenOptions};
use std::io::{self, BufRead};
use std::path::Path;

const MAX_NUM: usize = 99;
const MIN_NUM: usize = 0;

struct Dial {
    position: usize,
    count_zeros: usize,
    count_turns: usize,
}
impl Dial {
    fn new() -> Self {
        Dial {
            position: 50,
            count_zeros: 0,
            count_turns: 0,
        }
    }

    fn pointing1(&mut self, num: isize) {
    
        self.count_turns += 1;
        
        let mut n = self.position as isize + num;
        while n > MAX_NUM as isize {
            n -= MAX_NUM as isize + 1;
        } 
        while n < MIN_NUM as isize {
            n += MAX_NUM as isize + 1;
        }

        if n == 0 {
            self.count_zeros += 1;
        }
        self.position = n as usize;
    }

    fn pointing2(&mut self, num: isize) {
    
        self.count_turns += 1;
        
        let mut n = self.position as isize + num;
        let value = MAX_NUM as isize + 1;

        let mut start = self.position as isize;
        while n > MAX_NUM as isize {
            if start!= 0 && (n - value) > MIN_NUM as isize {
                self.count_zeros += 1;
            }
            n -= MAX_NUM as isize + 1;
            start=n;
        } 
        while n < MIN_NUM as isize {
            if start!=0 && (n + value) > MIN_NUM as isize {
                self.count_zeros += 1;
            }

            n += MAX_NUM as isize + 1;
            start=n;
        }
        
        if n == 0 {
            self.count_zeros += 1;
        }

        self.position = n as usize;
    }

    fn pointing(&mut self, num: isize) {
    
        self.count_turns += 1;
        
        let invariant_turns = num.abs() / (MAX_NUM as isize + 1);
        self.count_zeros += invariant_turns as usize;
        let remainder = num % (MAX_NUM as isize + 1);


        let mut n = self.position as isize + remainder;
        let value = MAX_NUM as isize + 1;

        let mut start = self.position as isize;
        while n > MAX_NUM as isize {
            if start!=0 && (n - value) > MIN_NUM as isize {
                self.count_zeros += 1;
            }
            n -= MAX_NUM as isize + 1;
            start=n;
        } 
        while n < MIN_NUM as isize {
            if start!=0 && (n + value) > MIN_NUM as isize {
                self.count_zeros += 1;
            }
            n += MAX_NUM as isize + 1;
            start=n;
        }

        if n == 0 {
            self.count_zeros += 1;
        }
        self.position = n as usize;
    }

    fn get_code(&self) -> usize {
        self.count_zeros
    }
}



pub fn read_document(file_path: &str) -> io::Result<Vec<isize>> {
    let path = Path::new(file_path);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut results = Vec::new();
    for line in reader.lines() {
        let line = line?;
        // each line is in the format Lx where L is a letter and x a number
        
        println!("Read line: {}", line);
        let direction = &line[0..1];
        let num_str = &line[1..];
        match num_str.parse::<usize>() {
            Ok(num) => {
                let signed_num = if direction == "L" {
                    -(num as isize)
                } else {
                    num as isize
                };
                results.push(signed_num);
            },
            Err(e) => eprintln!("Error parsing number: {}", e),
        }
    }

    Ok(results)
}

fn test(){
    let mut dial = Dial::new();
    println!("Dial position: ({}), Zeros: {}, Turns: {}", dial.position, dial.count_zeros, dial.count_turns);
    let rotations = vec![-50, -1, 1, 99, 106, -150, -99, -100, 0, 50];
    for rotation in rotations {
        dial.pointing(rotation as isize);
        println!("Rotation: {} Dial position: ({}), Zeros: {}, Turns: {}", 
            rotation, dial.position, dial.count_zeros, dial.count_turns);
    }

    let mut dial = Dial::new();
    println!("Dial position: ({}), Zeros: {}, Turns: {}", dial.position, dial.count_zeros, dial.count_turns);
    let rotations = vec![1000];
    for rotation in rotations {
        dial.pointing(rotation as isize);
        println!("Rotation: {} Dial position: ({}), Zeros: {}, Turns: {}", 
            rotation, dial.position, dial.count_zeros, dial.count_turns);
    }
}

fn main() {

    let file_path = std::env::args().nth(1).expect("no pattern given");
    if file_path=="try"{
        test();
        return;
    }

    let rotations = read_document(&file_path).expect("Failed to read document");
    let mut dial = Dial::new();
    println!("Dial position: ({}), Zeros: {}, Turns: {}", dial.position, dial.count_zeros, dial.count_turns);
    for rotation in rotations {
        dial.pointing(rotation as isize);
        println!("Rotation: {} Dial position: ({}), Zeros: {}, Turns: {}", 
            rotation, dial.position, dial.count_zeros, dial.count_turns);
    }
    println!("Final code (number of zeros pointed to): {}", dial.get_code());

}
