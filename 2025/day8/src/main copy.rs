use std::fs::{File, OpenOptions};
use std::io::{self, BufRead};
use std::path::Path;

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
        //print distances values
        self.do_shortest_circuits_step(0);
        
    }

    fn do_shortest_circuits_step(&mut self, mut step: usize) {
        println!("-----------Step {}-----------", step);
        
        let min_distance = self.distances.first().unwrap().2;
        println!("Minimum distance is {:.2}", min_distance);
        
        let circuit = (self.distances.first().unwrap().0, self.distances.first().unwrap().1);
        println!("Current circuit between {:?} and {:?}", circuit.0, circuit.1);
        // add new circuit if one of its coordinates is not already in existing circuits. in not extend existing circuits
        let mut first = false;
        let mut second = false;

        let mut to_merge_index: Option<usize> = None;
        
        for (i,existing_circuit) in self.circuits.iter_mut().enumerate() {
            let mut to_merge_index = None; 
            if existing_circuit.contains(&circuit.0) && !existing_circuit.contains(&circuit.1) {
                existing_circuit.push(circuit.1);
                self.remaining_coordinates -= 1;
                first = true;
                println!("first");
                to_merge_index = Some(i);
                continue;
            } else if existing_circuit.contains(&circuit.1) && !existing_circuit.contains(&circuit.0) {
                existing_circuit.push(circuit.0);
                self.remaining_coordinates -= 1;
                second = true;
                println!("second");
                
                continue;
            }else if existing_circuit.contains(&circuit.0) && existing_circuit.contains(&circuit.1) {
                first = true;
                println!("both already in circuit");
                break;
            }
        }
        // Merge circuits if both extended
        if first && second {
            println!("both extended existing circuits");
            if let Some(idx) = to_merge_index {
                // Find the other circuit containing circuit.1
                for (i, other_circuit) in self.circuits.iter().enumerate() {
                    if i != idx && other_circuit.contains(&circuit.1) {
                        let mut to_merge = self.circuits.remove(i);
                        self.circuits[idx].append(&mut to_merge);
                        println!("merged circuits");
                        break;
                    }
                }
            }
        }
        if !first && !second {
            self.circuits.push(vec![circuit.0, circuit.1]);
            self.remaining_coordinates -= 2;
            println!("new circuit added");
        }
        println!("Circuits formed: {:?} remaining {}", self.circuits, self.remaining_coordinates);
        
        // remove used circuit from distances
        self.distances.retain(|(c1, c2, _)| {
            !( (*c1 == circuit.0 && *c2 ==circuit.1) || (*c1 == circuit.1 && *c2 ==circuit.0) )
        });
        println!("Distances remaining after forming first circuit: {}", self.distances.len());
        println!("Number of circuits so far: {}", self.circuits.len());
        if self.remaining_coordinates > 0 {
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
