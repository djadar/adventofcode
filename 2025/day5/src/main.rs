use std::fs::{File, OpenOptions};
use std::io::{self, BufRead};
use std::path::Path;

struct IngredientID {
    count_ranges: usize,
    fresh_ranges: Vec<(usize, usize)>,
    available_ids: Vec<usize>,
    count_available_ids: usize,
}
impl IngredientID {
    fn new() -> Self {
        IngredientID {
            count_ranges: 0,
            fresh_ranges: Vec::new(),
            available_ids: Vec::new(),
            count_available_ids: 0,
        }
    }

    fn get_fresh_available_ingredients(&mut self) -> usize{
        let mut count_fresh_ids=0;
        for id in &self.available_ids {
            let mut k=0;
            let mut fresh = false;
            while !fresh && k < self.fresh_ranges.len() {
                let range = self.fresh_ranges[k];
                if *id >= range.0 && *id <= range.1 {
                    fresh=true;
                    count_fresh_ids +=1;
                    println!("fesh id {}", *id);
                }

                k+=1;
            }
        }

        count_fresh_ids
    }

    fn get_fresh_ingredients_from_range(&mut self) -> usize {
    let mut count_fresh_ids = 0;
    
    // Sort ranges (a,b) by start position a
    self.fresh_ranges.sort_by_key(|r| r.0);
    
    let mut merged_ranges: Vec<(usize, usize)> = Vec::new();
    
    for &(start, end) in &self.fresh_ranges {
        if let Some(last) = merged_ranges.last_mut() {
            // Check if current range overlaps or is adjacent to the last merged range
            if start <= last.1 + 1 {
                // Merge
                last.1 = last.1.max(end);
            } else {
                // No overlap
                merged_ranges.push((start, end));
            }
        } else {
            // First range
            merged_ranges.push((start, end));
        }
    }
    
    // Count all IDs in merged ranges
    for (start, end) in merged_ranges {
        count_fresh_ids += end - start + 1;
    }
    
    println!("Total fresh IDs from ranges: {}", count_fresh_ids);
    count_fresh_ids
}
     
    /* fn get_fresh_ingredients_from_range2(&mut self, step:usize) -> usize{

        println!("STEP {}", step);

        let mut compact_ranges= Vec::new();
        compact_ranges.push(self.fresh_ranges[0]);
        println!("First component {:?}", self.fresh_ranges[0]);
        // get if two range overlaps
        for i in 0..self.fresh_ranges.len() {
            let mut component = false; 
            let mut k=0;
            let range = self.fresh_ranges[i];
            while !component && k < compact_ranges.len() {
                let compact_range = compact_ranges[k];
                let mut first = compact_range;
                let mut second = range;
                if range.0 == compact_range.0 {
                    component = true;
                    break;
                }else if range.0 < compact_range.0{
                    first = range;
                    second = compact_range;
                }
                if first.1 >= second.0 {
                    component = true;
                }
                    
                if component {
                    let new_component = (
                        (first.0).min(second.0),
                        (first.1).max(second.1)
                    );
                    compact_ranges[k] = new_component;
                    println!("(merge) List of component {:?}", compact_ranges);
                }
                k+=1;
            }
            if !component{
                compact_ranges.push(range);
                //println!("new component {:?}", range);
                println!("(Push) List of component {:?}", compact_ranges);
            }
        }

        let mut count_fresh_ids=0;
        if self.fresh_ranges.len() != compact_ranges.len(){
            self.fresh_ranges = compact_ranges;
            return self.get_fresh_ingredients_from_range2(step+1);
        }else{
            println!("Total steps {} and {} ranges", step, compact_ranges.len());
            for range in compact_ranges {
                count_fresh_ids += range.1 - range.0 +1;
            }
            return count_fresh_ids;
        }
        
    }

    fn get_fresh_ingredients_from_range1(&mut self, step:usize) -> usize{

        println!("STEP {}", step);

        let mut compact_ranges= Vec::new();
        compact_ranges.push(self.fresh_ranges[0]);
        println!("First component {:?}", self.fresh_ranges[0]);
        // get if two range overlaps
        for i in 0..self.fresh_ranges.len() {
            let mut component = false; 
            let mut k=0;
            let range = self.fresh_ranges[i];
            while !component && k < compact_ranges.len() {
                let compact_range = compact_ranges[k];
                let mut valmin = 0;
                let mut valmax = 0;
                if range.0 >= compact_range.0 && range.0 <= compact_range.1 {
                    component=true;
                    valmin = range.0;
                    println!("0-merge {:?} with {:?}", range, compact_range);
                }else if range.1 >= compact_range.0 && range.1 <= compact_range.1 {
                    component=true;
                    println!("1-merge {:?} with {:?}", range, compact_range);
                }
                if component {
                    let new_component = (
                        (compact_range.0).min(range.0),
                        (compact_range.1).max(range.1)
                    );
                    compact_ranges[k] = new_component;
                    println!("(merge) List of component {:?}", compact_ranges);
                }
                k+=1;
            }
            if !component{
                compact_ranges.push(range);
                //println!("new component {:?}", range);
                println!("(Push) List of component {:?}", compact_ranges);
            }
        }

        let mut count_fresh_ids=0;
        if self.fresh_ranges.len() != compact_ranges.len(){
            self.fresh_ranges = compact_ranges;
            return self.get_fresh_ingredients_from_range1(step+1);
        }else{
            println!("Total steps {}", step);
            for range in compact_ranges {
                count_fresh_ids += range.1 - range.0 +1;
            }
            return count_fresh_ids;
        }
        
    }

} */

}

pub fn read_document(file_path: &str) -> io::Result<IngredientID> {
    let path = Path::new(file_path);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut ingredients = IngredientID::new();
    for line in reader.lines() {
        let line = line?;
        //println!("Read line: {}", line);
        // get all numbers int the line
        for num_str in line.split_whitespace() {
            let num_range: Vec<&str> = num_str.split('-').collect();
            if num_range.len() == 2 {
                let start = num_range[0].trim().parse().unwrap();
                let end = num_range[1].trim().parse().unwrap();
                ingredients.fresh_ranges.push((start, end));
                ingredients.count_ranges += 1;
            }else{
                let id: usize = num_str.trim().parse().unwrap();
                ingredients.available_ids.push(id);
                ingredients.count_available_ids += 1;
            }
        } 

    }
    println!("Total ranges processed: {}", ingredients.count_ranges);
    println!("Total ingredients found: {}", ingredients.count_available_ids);
    Ok(ingredients)
}


fn main() {

    let file_path = std::env::args().nth(1).expect("no pattern given");

    let mut ingredients = read_document(&file_path).expect("Failed to read document");
    println!("Ranges processed: {:?}", ingredients.fresh_ranges);
    //println!("Ingredients found: {:?}", ingredients.available_ids);
    
    let mut result = ingredients.get_fresh_ingredients_from_range(); //get_fresh_available_ingredients();
    
    println!("Final answer is {}", result);

}
