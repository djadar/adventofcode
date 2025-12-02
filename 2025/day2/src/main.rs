use std::fs::{File, OpenOptions};
use std::io::{self, BufRead};
use std::path::Path;

struct Range {
    count_items: usize,
    count_invalids: usize,
}
impl Range {
    fn new() -> Self {
        Range {
            count_items: 0,
            count_invalids: 0,
        }
    }

    fn get_invalids1(&mut self, start: usize, end:usize) -> Vec<usize>{
        println!("Get invalids from {} to {}", start, end);
        self.count_items += end - start + 1;
        let mut invalids_numbers: Vec<usize> = vec![];
        // an invalid id is made a sequence of digits repeted twice
        for number in start..=end {
            let s = number.to_string();
            let len = s.len();
            let half_len = len / 2;
            let first_half = &s[0..half_len];
            let second_half = &s[half_len..len];
            if first_half == second_half {
                println!("Found invalid number: {}", number);
                self.count_invalids += 1;
                invalids_numbers.push(number);
            }
        }
        invalids_numbers
    }

    fn get_invalids(&mut self, start: usize, end:usize) -> Vec<usize>{
        println!("Get invalids from {} to {}", start, end);
        self.count_items += end - start + 1;
        let mut invalids_numbers: Vec<usize> = vec![];
        // an invalid id is made a sequence of digits repeated more than twice
        for number in start..=end {
            //println!("number {}", number);
            let s = number.to_string();
            let len = s.len();
            let half_len = len / 2;
            
            let mut sub_length=1;
            let mut invalid= false;

            while !invalid && sub_length <= half_len {
                if len%sub_length==0 {
                    let subparts = len/sub_length; //:Vec<&str> = Vec::new();
                    let first_part = &s[0..sub_length];
                    //println!("subparts {} first_part {}", subparts, first_part);
                    invalid = true;
                    for k in 1..subparts {
                        //println!("")
                        if &s[k*sub_length..(k+1)*sub_length] != first_part{
                            invalid = false;
                            break
                        } 
                    }
                }
                
                sub_length+=1;
            }

            if invalid {
                println!("Found invalid number: {}", number);
                self.count_invalids += 1;
                invalids_numbers.push(number);
            }
        }
        invalids_numbers
    }

}



pub fn read_document(file_path: &str) -> io::Result<Vec<usize>> {
    let path = Path::new(file_path);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut range = Range::new();
    let mut results = Vec::new();
    for line in reader.lines() {
        let line = line?;
        // each line is in the format Lx where L is a letter and x a number
        
        println!("Read line: {}", line);
        // get all numbers int the line
        // the format is 11-22,95-115,998-1012,1188511880-1188511890,222220-222224, 1698522-1698528,446443-446449,38593856-38593862,565653-565659, 824824821-824824827,2121212118-2121212124
        let numbers: Vec<&str> = line.split(',').collect();
        for number in numbers {
            let num_range: Vec<&str> = number.split('-').collect();
            if num_range.len() == 2 {
                let start = num_range[0].trim().parse().unwrap();
                let end = num_range[1].trim().parse().unwrap();
                let v = range.get_invalids(start, end);
                results.extend(v);
            }
        }       
    }
    println!("Total items processed: {}", range.count_items);
    println!("Total invalids found: {}", range.count_invalids);
    Ok(results)
}

fn test(){
    let start=11; 
    let end = 22; 
    let mut range = Range::new();
    let v = range.get_invalids(start, end);
    let v = range.get_invalids(95, 115);
    
}

fn main() {

    let file_path = std::env::args().nth(1).expect("no pattern given");
    if file_path=="try"{
        test();
        return;
    }

    let invalids = read_document(&file_path).expect("Failed to read document");
    println!("Invalids: {:?}", invalids);
    let mut sum=0;
    for invalid in invalids {
        sum+=invalid;
    }
    println!("Final answer is {}", sum);

}
