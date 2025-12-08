use std::fs::{File, OpenOptions};
use std::io::{self, BufRead};
use std::path::Path;

const MAX_IER: usize = 1000;

struct A3Dspace {
    coordinates: Vec<(usize, usize, usize)>,
    circuits: Vec<Vec<(usize, usize, usize)>>,
    distances: Vec<((usize, usize, usize), (usize, usize, usize), f64)>,
    remaining_coordinates: usize,
}
impl A3Dspace {
    fn new() -> Self {
        A3Dspace {
            coordinates: Vec::new(),
            circuits: Vec::new(),
            remaining_coordinates: 0,
            distances: Vec::new(),
        }
    }

    fn do_shortest_circuits(&mut self) {
        //compute straight line distances between all coordinates
        
        for i in 0..self.coordinates.len() {
            for j in i+1..self.coordinates.len() {
                let (x1, y1, z1) = self.coordinates[i];
                let (x2, y2, z2) = self.coordinates[j];
                let distance = (((x2 as isize - x1 as isize).pow(2) + (y2 as isize - y1 as isize).pow(2) + (z2 as isize - z1 as isize).pow(2)) as f64).sqrt();
                self.distances.push(((x1, y1, z1), (x2, y2, z2), distance));
            }
        }
        println!("Computed {} distances between coordinates", self.distances.len());
        //sort distances
        self.distances.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());
        println!("{:?}", self.distances);
        //print distances values
        self.do_shortest_circuits_step(0);
        
    }

    fn do_shortest_circuits_step(&mut self, mut step: usize) {
        println!("-----------Step {}-----------", step);
        
        let min_distance = self.distances.first().unwrap().2;
        println!("Minimum distance is {:.2}", min_distance);
        
        let circuit = (self.distances.first().unwrap().0, self.distances.first().unwrap().1);
        println!("Current circuit {:?}-{:?}", circuit.0, circuit.1);
        // add new circuit if one of its coordinates is not already in existing circuits. in not extend existing circuits
        let mut first = false;
        let mut second = false;

        let mut first_index: Option<usize> = None;
        let mut second_index: Option<usize> = None;
        
        for (i,existing_circuit) in self.circuits.iter_mut().enumerate() {
            // if one of the circuit coordinates is in existing circuit, extend it
            if existing_circuit.contains(&circuit.0) {
                first = true;
                first_index = Some(i);
                print!("first");
            }
            if existing_circuit.contains(&circuit.1) {
                second = true;
                second_index = Some(i);
                print!("second");
            }        
        }
       
        // now extend circuits if needed
        if first && !second{
            // extend first circuit
            if let Some(first_i) = first_index {
                let mut existing_circuit = &mut self.circuits[first_i];
                if !existing_circuit.contains(&circuit.1){
                    existing_circuit.push(circuit.1);
                    println!("1-extend");
                    self.remaining_coordinates -= 1;
                }
            }
            
        } else if second && !first{
            // extend second circuit
            if let Some(second_i) = second_index {
                let mut existing_circuit = &mut self.circuits[second_i];
                if !existing_circuit.contains(&circuit.0){
                    existing_circuit.push(circuit.0);
                    println!("2-extend");
                    self.remaining_coordinates -= 1;
                }
            }
            
        } else if first && second {
            if let (Some(mut first_i), Some(mut second_i)) = (first_index, second_index) {
                if first_i != second_i {
                    // merge circuits
                    if second_i < first_i {
                        let temp = first_i;
                        first_i = second_i;
                        second_i = temp;
                    }
                    print!("Merging circuits {:?} and {:?}", self.circuits[first_i], self.circuits[second_i]);
                    
                    let mut to_merge = self.circuits.remove(second_i);
                    self.circuits[first_i].append(&mut to_merge);
                    println!("into {:?}", self.circuits[first_i]);
                    
                }else {
                    println!("Same circuit");
                }
            }  
        }else { //if !first && !second {
            self.circuits.push(vec![circuit.0, circuit.1]);
            self.remaining_coordinates -= 2;
            println!("new circuit added");
        }
        println!("************");
        println!("Circuits formed: {:?} remaining {}", self.circuits, self.remaining_coordinates);
        
        // remove used circuit from distances
        self.distances.retain(|(c1, c2, _)| {
            !( (*c1 == circuit.0 && *c2 ==circuit.1) || (*c1 == circuit.1 && *c2 ==circuit.0) )
        });
        println!("Distances remaining after forming first circuit: {}", self.distances.len());
        println!("Number of circuits so far: {}", self.circuits.len());
        let mut sizes: Vec<usize> = self.circuits.iter().map(|circuit| circuit.len()).collect();
        sizes.sort_unstable_by(|a, b| b.cmp(a)); // sort descending

        let sum: usize = sizes.iter().take(3).sum();
        println!("Sizes of circuits: {:?} sum {}", sizes, sum);
        if step < MAX_IER -1{
            step += 1;
            self.do_shortest_circuits_step(step);
        }
    }

    fn get_three_largest_circuits_size(&self) -> usize {
        let mut sizes: Vec<usize> = self.circuits.iter().map(|circuit| circuit.len()).collect();
        sizes.sort_unstable_by(|a, b| b.cmp(a)); // sort descending
        println!("Sizes of circuits: {:?}", sizes);
        // get the product of the sizes of the three largest circuits
        let result: usize = sizes.iter().take(3).product();
        result
    }
}

pub fn read_document(file_path: &str) -> io::Result<A3Dspace> {
    let path = Path::new(file_path);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut space = A3Dspace::new();
    for line in reader.lines() {
        let line = line?;
        //println!("Read line: {}", line);
        // get all numbers int the line
        for num_str in line.split_whitespace() {
            let coordinates: Vec<&str> = num_str.split(',').collect();
            if coordinates.len() == 3 {
                let x = coordinates[0].trim().parse().unwrap();
                let y = coordinates[1].trim().parse().unwrap();
                let z = coordinates[2].trim().parse().unwrap();
                space.coordinates.push((x, y, z));
                space.remaining_coordinates += 1;
            }else{
                println!("Invalid coordinate format: {}", num_str);
            }
        } 

    }
    println!("Total junction boxes processed: {}", space.remaining_coordinates);
    Ok(space)
}


fn main() {

    let file_path = std::env::args().nth(1).expect("no pattern given");

    let mut a3Dspace = read_document(&file_path).expect("Failed to read document");
    println!("3D space: {:?}", a3Dspace.coordinates);
    //println!("Ingredients found: {:?}", ingredients.available_ids);
    a3Dspace.do_shortest_circuits();
    let result = a3Dspace.get_three_largest_circuits_size(); //get_fresh_available_ingredients();
    
    println!("Final answer is {}", result);

    if file_path=="test"{
        let answer = 40;
        assert!(result==answer); 
        return;
    }

}
