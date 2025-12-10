use std::fs::{File, OpenOptions};
use std::io::{self, BufRead};
use std::path::Path;
use rayon::prelude::*;


#[derive(Clone, Default, Debug)]
struct Machine {
    light_diagram: Vec<char>, //lights
    activated: Vec<String>,
    button_wiring: Vec<Vec<String>>,
    number_of_buttons: usize,
    joltage: Vec<usize>,
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

    fn minimum_press(&mut self){
        //let mut number_to_activated = Vec::new();
        
        //println!("{:?}", Self::all_subsets_parallel(self.number_of_wirings));
        /* for list in self.button_wiring {
            let mut number_press = 0;

        } */
        let n = self.number_of_wirings - 1;
        let mut total = 1usize << (n + 1); // 2^(N+1)

        for mask in 1..total {
            let mut subset = Vec::new();
            let mut press = 0;
            for i in 0..=n {
                if (mask & (1 << i)) != 0 {
                    //permutation i 
                    //subset.push(i);
                    let vec = self.button_wiring[i].clone();
                    println!("vec {:?}", vec);
                    for j in 0..vec.len() {
                        //println!("{}", vec[j]);
                        if subset.contains(&vec[j]) {
                            //println!("B {:?}", subset);
                            //subset.remove(j);
                            subset.retain(|x| *x != vec[j]);
                            //println!("A {:?}", subset);
                        }else{
                            subset.push(vec[j].clone());
                        }
                    }
                    press +=1;
                    if press >= self.min_press {
                        break;
                    }
                } 
            }
            subset.sort();
            if subset==self.activated {
                if press < self.min_press {
                    self.min_press = press;
                }
            }
            println!("{}: subset {:?} and press {} min {}", 
                mask, subset, press, self.min_press);
            
        }
        
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
            //break;
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
                    for i in 0.. coordinates.len() {
                        match coordinates[i].parse::<usize>() {
                            Ok(num) => {
                                vec.push(num)
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
    //println!("Ingredients found: {:?}", ingredients.available_ids);
     
    let result = manual.fewest_button_press(); //get_fresh_available_ingredients();
    
    println!("Final answer is {}", result);

    if file_path=="test"{
        let answer = 7;
        assert!(result==answer); 
        return;
    }

}
