use std::fs::{File, OpenOptions};
use std::io::{self, BufRead};
use std::path::Path;
use rayon::prelude::*;
use std::sync::{Arc, Mutex};

#[derive(Clone, Default, Debug)]
struct Machine {
    light_diagram: Vec<char>, //lights
    activated: Vec<String>,
    button_wiring: Vec<Vec<String>>,
    number_of_buttons: usize,
    joltage: Vec<String>,
    number_of_wirings: usize,
    min_press:usize,
}

impl Machine {
    fn new() -> Self {
        Machine {
            light_diagram: Vec::new(),
            activated: Vec::new(),
            button_wiring: Vec::new(),
            number_of_buttons: 0,
            joltage: Vec::new(),
            number_of_wirings: 0,
            min_press: usize::MAX,
        }
    }

    fn all_subsets_parallel(n: usize) -> Vec<Vec<usize>> {
        let total = 1usize << (n + 1); // 2^(N+1)

        (1..total)
            .into_par_iter()
            .map(|mask| {
                let mut subset = Vec::new();
                for i in 0..=n {
                    if (mask & (1 << i)) != 0 {
                        subset.push(i);
                    }
                }
                subset
            })
            .collect()
    }

    fn minimum_press(&mut self) {
        let n = self.number_of_wirings - 1;
        let elements: Vec<usize> = (0..=n).collect();
        let mut length = 10;
        let size: usize = self.joltage
            .par_iter()
            .map(|s| s.parse::<usize>().unwrap())
            .sum();
        let mut length = 1;
        let mut found = false;

        let sum:usize = (0..n).into_par_iter().map(|v| self.button_wiring[v].len()).sum();
        println!(" sum {} size{}", sum, size);

        /* if self.button_wiring.len() < size {
            length = self.button_wiring.len();
        }   */   
        if sum < size {
            length = sum;
        }
        println!("length {}", length);
        //return;
        // Wrap shared state in Arc<Mutex<>>
        let min_press = Arc::new(Mutex::new(self.min_press));
        let found_flag = Arc::new(Mutex::new(false));

        while !*found_flag.lock().unwrap() {
            let total = (n + 1).pow(length as u32);

            (0..total).into_par_iter().for_each(|i| {
                // Check if already found to skip unnecessary work
                if *found_flag.lock().unwrap() {
                    return;
                }

                // Build sequence
                let mut seq = Vec::with_capacity(length);
                let mut num = i;
                for _ in 0..length {
                    seq.push(elements[num % (n + 1)]);
                    num /= n + 1;
                }
                seq.reverse();
                println!("length={}: seq={:?}", length, seq);
                let total_size: usize = seq.par_iter().map(|v| self.button_wiring[*v].len()).sum();
                if total_size !=size {
                    return;
                }
                // Compute subset
                let mut subset = Vec::new();
                let mut press = 0;
                for idx in seq {
                    let vec = self.button_wiring[idx].clone();
                    subset.extend(vec);
                    press += 1;
                }
                

                subset.sort();
                if subset == self.joltage {
                    let mut min_lock = min_press.lock().unwrap();
                    if press < *min_lock {
                        *min_lock = press;
                        *found_flag.lock().unwrap() = true;
                    }
                }

                println!(
                    "length={}: subset {} and press {} min {}",
                    length,
                    subset.len(),
                    press,
                    *min_press.lock().unwrap()
                );
            });

            length += 1;
        }

        // Update struct's min_press
        self.min_press = *min_press.lock().unwrap();
    }

    fn minimum_press_1(&mut self){
        let n = self.number_of_wirings - 1; 
        let elements: Vec<usize> = (0..=n).collect();
        let mut length = 1;
        let mut found = false;
        while !found {
            let total = (n + 1).pow(length as u32); // total sequences of this length
            let mut end = false;
            for i in 0..total {
                
                let mut seq = Vec::with_capacity(length);
                let mut num = i;

                for _ in 0..length {
                    seq.push(elements[num % (n + 1)]);
                    num /= n + 1;
                }

                // The sequence is in reverse order; reverse to match natural order
                seq.reverse();
                println!("seq={:?}", seq);
                let mut subset = Vec::new();
                let mut press = 0;
                for i in seq {
                    let vec = self.button_wiring[i].clone();
                    //println!("vec {:?}", vec);
                    for j in 0..vec.len() {
                        //println!("{}", vec[j]);
                        subset.push(vec[j].clone());
                        
                    }
                }
                press +=1;
                
                subset.sort();
                if subset==self.joltage {
                    if press < self.min_press {
                        self.min_press = press;
                        found==true;
                    }
                }
                
                 
                println!("length={}: subset {:?} and press {} min {}", 
                    length, subset, press, self.min_press);
            }
            
            length+=1;
        }

        
    }

    fn minimum_press_2(&mut self) {
        let n = self.number_of_wirings - 1;
        let elements: Vec<usize> = (0..=n).collect();
        let size: usize = self.joltage
        .par_iter()
        .map(|s| s.parse::<usize>().unwrap())
        .sum();
        let mut length = 1;
        let mut found = false;

        let sum = elements.par_iter().map(|v| self.button_wiring[*v].len()).sum();
        println!(" sum {} size{}", sum, size);
                
        if sum < size {
            length = sum;
        }
        // Wrap min_press in Arc<Mutex<>> for thread-safe updates
        let min_press = Arc::new(Mutex::new(self.min_press));

        while !found {
            let total = (n + 1).pow(length as u32);
            //let end = Arc::new(Mutex::new(false));

            // Parallel iteration over i
            (0..total).into_par_iter().for_each(|i| {
                let mut seq = Vec::with_capacity(length);
                let mut num = i;

                for _ in 0..length {
                    seq.push(elements[num % (n + 1)]);
                    num /= n + 1;
                }
                seq.reverse();
                let total_size: usize = seq.par_iter().map(|v| self.button_wiring[*v].len()).sum();
                println!("seq={:?}, total_size{}", seq, total_size);
                /* if total_size !=size {
                    return;
                } */
                let mut subset = Vec::new();
                let mut press = 0;

                for idx in seq {
                    let vec = self.button_wiring[idx].clone();
                    for v in vec {
                        subset.push(v);
                    }
                    press += 1;
                }

                /* if subset.len() > self.joltage.len() {
                    *end.lock().unwrap() = true;
                } else { */
                    subset.sort();
                    if subset == self.joltage {
                        let mut min_lock = min_press.lock().unwrap();
                        if press < *min_lock {
                            *min_lock = press;
                            found==true;
                        }
                    }
                //}

                println!(
                    "length={}: subset {:?} and press {} min {}",
                    length,
                    subset,
                    press,
                    *min_press.lock().unwrap()
                );
            });
/* 
            if *end.lock().unwrap() {
                found = true;
            }
 */
            length += 1;
        }

        // Update self.min_press at the end
        self.min_press = *min_press.lock().unwrap();
    }
}

#[derive(Clone, Default, Debug)]
struct Manual {
    machines: Vec<Machine>,
    number_of_machines: usize,
}

impl Manual {
    fn new() -> Self {
        Manual {
            machines: Vec::new(),
            number_of_machines: 0,
        }
    }

    fn fewest_button_press(&mut self) ->usize{
        let mut result=0;
        for machine in &mut self.machines {
            machine.minimum_press();
            result += machine.min_press;
            break;
        }
        result
    }
    
}

pub fn read_document(file_path: &str) -> io::Result<Manual> {
    let path = Path::new(file_path);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut manual = Manual::new();
    for line in reader.lines() {
        let line = line?;
        //println!("Read line: {}", line);
        let mut machine = Machine::new();

        for num_str in line.split_whitespace() {
            if num_str.starts_with('[') && num_str.ends_with(']') {
                //println!("1");
                let inner = &num_str[1..num_str.len() - 1];
                machine.light_diagram = inner.chars().collect();
                machine.number_of_buttons = inner.chars().count(); 
                let positions: Vec<String> = inner
                .char_indices()       // gives (index, char)
                .filter(|&(_, c)| c == '#')
                .map(|(i, _)| i.to_string())   // convert usize → String
                .collect();
                machine.activated = positions;

            } else if num_str.starts_with('(') && num_str.ends_with(')') {
                //println!("2");
                let inner = &num_str[1..num_str.len() - 1]; 
                let coordinates: Vec<&str> = inner.split(',').collect();
                if coordinates.len()> 0 {
                    let mut vec = Vec::new();
                    for i in 0.. coordinates.len() {
                        vec.push(coordinates[i].to_string());
                        /* match coordinates[i].parse::<usize>() {
                            Ok(num) => {
                                vec.push(num)
                            },
                            Err(e) => eprintln!("Error parsing number: {}", e),
                        } */
                    }

                    machine.button_wiring.push(vec);
                    machine.number_of_wirings +=1;
                }

            }else if num_str.starts_with('{') && num_str.ends_with('}') {
                //println!("3");
                let inner = &num_str[1..num_str.len() - 1]; 
                
                let coordinates: Vec<&str> = inner.split(',').collect();
                if coordinates.len()> 0 {
                    
                    let mut vec = Vec::new();
                    for i in 0..coordinates.len() {
                        match coordinates[i].parse::<usize>() {
                            Ok(num) => {
                                vec.extend(vec![i.to_string(); num]);
                            },
                            Err(e) => eprintln!("Error parsing number: {}", e),
                        }
                    }
                    
                    machine.joltage = vec;
                }

            }else{
                println!("Not known");
            }
            
        } 

        if machine.number_of_buttons>0 {
            manual.machines.push(machine);
            manual.number_of_machines +=1
        }
    }
    println!("Total processed: {}", manual.number_of_machines);
    Ok(manual)
}


fn main() {

    let file_path = std::env::args().nth(1).expect("no pattern given");

    let mut manual = read_document(&file_path).expect("Failed to read document");
    println!("Manual {:?}", manual);
    
    let result = manual.fewest_button_press(); 
    
    println!("Final answer is {}", result);

    if file_path=="test"{
        let answer = 33;
        assert!(result==answer); 
        return;
    }

}
