use std::fs::{File, OpenOptions};
use std::io::{self, BufRead};
use std::path::Path;

const MAX_IER: usize = 1000;


#[derive(Clone, Default, Debug)]
struct Machine {
    light_diagram: Vec<char>,
    button_wiring: Vec<Vec<usize>>,
    joltage: Vec<usize>,
    number_of_buttons: usize,
    number_of_wirings: usize,
}

impl Machine {
    fn new() -> Self {
        Machine {
            light_diagram: Vec::new(),
            button_wiring: Vec::new(),
            number_of_buttons: 0,
            joltage: Vec::new(),
            number_of_wirings: 0,
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

            } else if num_str.starts_with('(') && num_str.ends_with(')') {
                //println!("2");
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
    /* 
    a3Dspace.do_shortest_circuits();
    let result = a3Dspace.get_three_largest_circuits_size(); //get_fresh_available_ingredients();
    
    println!("Final answer is {}", result);

    if file_path=="test"{
        let answer = 25272; //40;
        assert!(result==answer); 
        return;
    } */

}
