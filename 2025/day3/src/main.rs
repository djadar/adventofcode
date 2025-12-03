use std::fs::{File, OpenOptions};
use std::io::{self, BufRead};
use std::path::Path;

const MAXJOLTS: usize = 12;
struct Joltage {
    count_banks: usize,
}
impl Joltage {
    fn new() -> Self {
        Joltage {
            count_banks: 0,
        }
    }

    fn get_largest_joltage_two_digits(&mut self, bank: &str) -> usize{
        println!("Get max joltage from {}", bank);
        self.count_banks += 1;
        let digits: Vec<usize> = bank
                    .chars()
                    .map(|c| c.to_digit(10).unwrap() as usize)
                    .collect();
        let len = digits.len();
        
        let mut first_digit = digits[0];
        let mut second_digit = digits[1];
        println!("Largest Batterie {}", 
            first_digit*10 + second_digit);

        let mut counter=0;
        let mut pos1=0;
        let mut pos2=1;
        while counter < len-2 {
            if digits[counter+1] > first_digit {
                pos1 = counter+1;
                pos2 = pos1+1;
                first_digit = digits[pos1];
                second_digit = digits[pos2];
                println!("first_digit {}", first_digit);
            }
            for d in pos2+1..len {
                if digits[d] > second_digit {
                    pos2=d;
                    second_digit = digits[pos2];
                    println!("second_digit {}", second_digit);
                } 
            }
            
            counter+=1;
        }
       
        let batterie = first_digit*10 + second_digit;
        println!("Largest Batterie {}", batterie);
        batterie
    }

    fn get_largest_joltage(&mut self, bank: &str) -> usize{
        println!("Get max joltage from {}", bank);
        self.count_banks += 1;
        let digits: Vec<usize> = bank
                    .chars()
                    .map(|c| c.to_digit(10).unwrap() as usize)
                    .collect();
        let len = digits.len();
        println!("Total digits {}", len);

        let mut batteries_digits: Vec<usize> = digits[0..MAXJOLTS].to_vec();
        let mut batterie = batteries_digits.iter()
        .enumerate()
        .map(|(i, &d)| d * 10_usize.pow((MAXJOLTS - i - 1) as u32)).sum();

        println!("Start Largest Batterie {}", batterie);
        //let mut counter=0;
        let mut digits_pos: Vec<usize> = (0..MAXJOLTS).collect();
        
        //while counter < len-MAXJOLTS {
            for i in 0..MAXJOLTS {
                let maxpos = len - (MAXJOLTS - i);
                //println!("maxpos {} digits_pos[{}] {} batteries_digits[{}] {}", 
                  //      maxpos, i, digits_pos[i], i, batteries_digits[i]);
                        
                for d in digits_pos[i]+1..maxpos+1 {
                    if digits[d] > batteries_digits[i] {
                        
                        digits_pos[i]=d;
                        batteries_digits[i] = digits[digits_pos[i]];
                        //println!("{}_digit {} current_pos {}", 
                          //  i, batteries_digits[i], d);

                        //shift following digits positions
                        for j in i+1..MAXJOLTS {
                            digits_pos[j] = d+(j - i);
                            batteries_digits[j] = digits[digits_pos[j]];
                        }
                        
                    } 
                }
            }
            
           /*  batterie = batteries_digits.iter()
                .enumerate()
                .map(|(i, &d)| d * 10_usize.pow((MAXJOLTS - i - 1) as u32)).sum();
            println!("Largest Batterie {}", batterie);
            
            //counter+=1;
         */
       
        batterie = batteries_digits.iter()
                .enumerate()
                .map(|(i, &d)| d * 10_usize.pow((MAXJOLTS - i - 1) as u32)).sum();
                println!("Largest Batterie {}", batterie);
        batterie
    }

}



pub fn read_document(file_path: &str) -> io::Result<Vec<usize>> {
    let path = Path::new(file_path);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut joltage = Joltage::new();
    let mut results = Vec::new();
    for line in reader.lines() {
        let line = line?;
        // each line is in the format Lx where L is a letter and x a number
        
        println!("Read line: {}", line);
        for num_str in line.split_whitespace() {
            let v = joltage.get_largest_joltage(num_str);
            results.push(v);
        } 
    }
    println!("Total items processed: {}", joltage.count_banks);
    Ok(results)
}

fn test(){
    let bank="818181911112111"; 
    let mut joltage = Joltage::new();
    let v = joltage.get_largest_joltage(bank);
    assert_eq!(v, 92);
    let bank="811111111111119"; //"818181911112111"; 
    let mut joltage = Joltage::new();
    let v = joltage.get_largest_joltage(bank);
    assert_eq!(v, 89) ; //92);
    
}

fn main() {

    let file_path = std::env::args().nth(1).expect("no pattern given");
    if file_path=="try"{
        test();
        return;
    }

    let results = read_document(&file_path).expect("Failed to read document");
    println!("Results: {:?}", results);
    let mut sum=0;
    for r in results {
        sum+=r;
    }
    println!("Final answer is {}", sum);

}
