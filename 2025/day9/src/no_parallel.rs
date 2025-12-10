use std::fs::{File, OpenOptions};
use std::io::{self, BufRead};
use std::path::Path;


struct Tile {
    rectangles: Vec<((usize, usize), (usize, usize), usize)>, // corner.0, corner.1, surface
    number_of_columns: usize,
    number_of_lines: usize,
    coordinates: Vec<(usize, usize)>,
    red_coordinates: Vec<(usize, usize)>,
    number_of_coordinates: usize,
}
impl Tile {
    fn new() -> Self {
        Tile {
            rectangles: Vec::new(),
            coordinates: Vec::new(),
            red_coordinates: Vec::new(),
            number_of_columns: 0,
            number_of_lines: 0,
            number_of_coordinates: 0,
        }
    }

    fn build_tile(&mut self) {
        self.number_of_lines = self.coordinates.iter().max_by_key(|x| x.0).map(|x| x.0).unwrap() + 1;
        self.number_of_columns = self.coordinates.iter().max_by_key(|x| x.1).map(|x| x.1).unwrap() + 1;
        
        println!("Tile of size {}*{}", self.number_of_lines, self.number_of_columns);
    }

    fn print(&self, flag: bool) {
        for j in 0..self.number_of_columns {
            for i in 0..self.number_of_lines {
                if self.coordinates.contains(&(i,j)) && self.red_coordinates.contains(&(i,j)){
                    print!("{:<5}", "#");
                        
                }else if self.coordinates.contains(&(i,j)) {
                    if flag {
                        print!("({:<2},{:<1})", i, j);
                    } else {
                        print!("{:<5}", "X");
                    }
                        
                } else {
                    print!("{:<5}", ".");
                }
            }
            println!("");
        }
    }
    fn add_green(&mut self){
        //println!("tile coordinates: {:?}", self.coordinates);
        //self.print(true);
        let mut added =0;
        for i in 0..self.coordinates.len() {
            //rintln!("step {}/{}", i, self.coordinates.len());
            for j in i+1..self.coordinates.len() {
                //println!("A {:?} B {:?}", self.coordinates[i], self.coordinates[j]);
                let (mut x1, mut y1) = self.coordinates[i];
                let (mut x2, mut y2) = self.coordinates[j];
                let height = (x2 as isize - x1 as isize).abs() as usize; 
                let width = (y2 as isize - y1 as isize).abs() as usize;
                if width!=0 && height==0 {
                    //println!("A {:?} B {:?}", self.coordinates[i], self.coordinates[j]);
                
                    if y1 > y2{
                        let temp = y2;
                        y2 = y1;
                        y1 = temp;
                    }
                    for a in y1+1..y2 {
                        if self.coordinates.contains(&(x1,a)) {
                            break;
                        }
                        self.coordinates.push((x1,a));
                        added +=1;
                    }
                    
                }
                
                if height!=0 && width==0{
                    
                    if x1 > x2{
                        let temp = x2;
                        x2 = x1;
                        x1 = temp;
                    }
                    for b in x1+1..x2 {
                        if self.coordinates.contains(&(b,y1)) {
                            break;
                        }
                        self.coordinates.push((b,y1));
                        added +=1;
                    }
                    
                }
            }
            
        } 
        //println!("added {}", added);
        //self.print(false);

        for j in 0..self.number_of_columns {
            let mut found_red = false;
            let mut i = 0;
            while  i < self.number_of_lines {
                //print!("({},{})", i, j);
                if self.coordinates.contains(&(i,j)) {
                    found_red = true;
                }
                if found_red && !self.coordinates.contains(&(i,j)) {
                    self.coordinates.push((i,j));
                    added +=1;
                }
                i+=1;
            }
            
            
        }
        println!("added {}", added);
        self.print(false);
    }

    fn find_largest_rectangle_withinbox(&mut self) -> usize{
        println!("tile coordinates: {}", self.coordinates.len());
        let result = 0;
        self.rectangles= Vec::new();
        for i in 0..self.red_coordinates.len() {
            for j in i+1..self.red_coordinates.len() {
                //println!("A {:?} B {:?}", self.coordinates[i], self.coordinates[j]);
                
                let (x1, y1) = self.coordinates[i];
                let (x2, y2) = self.coordinates[j];
                let mut height = (x2 as isize - x1 as isize).abs() as usize; 
                let mut width =  (y2 as isize - y1 as isize).abs() as usize;
                if height==0 && width ==0 {
                    //self.rectangles.push(((x1, y1), (x2, y2), 0));
                }else if height*width == 0 {
                    self.rectangles.push(((x1, y1), (x2, y2), height+width));
                }else{ // not !=0
                    //self.rectangles.push(((x1, y1), (x2, y2), height*width)
                    //println!("height {} width {}", height, width);
                    let mut min_x = x1.min(x2);
                    let mut max_x = x1.max(x2);
                    let mut min_y = y1.min(y2);
                    let mut max_y = y1.max(y2);
                    //area =  (max_x - min_x)* (max_y - min_y)
                    let mut corners = vec![(min_x, min_y), 
                    (min_x, max_y), (max_x, min_y), (max_x, max_y)];
                    //println!("corners {:?}", corners);
                    let mut to_remove= Vec::new();
                    let mut k=0;
                    while k< corners.len(){
                        if corners[k]==(x1, y1) || corners[k]==(x2, y2) {
                            to_remove.push(k);
                        }
                        k+=1;  
                    }
                    corners.remove(to_remove[1]);
                    corners.remove(to_remove[0]);
                    //println!("corners {:?}", corners);
                    let mut add = true;
                    for i in 0..corners.len() {
                        if !self.coordinates.contains(&corners[i]){
                            add = false;
                        }
                    }
                    if add {
                        //print!("A {:?} B {:?} ", self.coordinates[i], self.coordinates[j]);
                        let area = (height+1)*(width+1);
                        //println!("corners {:?} {}", corners, area);
                    
                        self.rectangles.push(((x1, y1), (x2, y2), area));
                    }
                    
                    

                }   
            }
        }
        self.rectangles.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap());
        let max_distance = self.rectangles.first().unwrap().2;
        println!("{:?} : max distance is {:.2}", self.rectangles.first(), max_distance);
        
        println!("Rectangles size {}", self.rectangles.len());


        max_distance
        
    }

    fn find_largest_rectangle(&mut self) -> usize{
        let result = 0;

        for i in 0..self.coordinates.len() {
            for j in i+1..self.coordinates.len() {
                let (x1, y1) = self.coordinates[i];
                let (x2, y2) = self.coordinates[j];
                let height = (x2 as isize - x1 as isize).abs() as usize +1; 
                let width =  (y2 as isize - y1 as isize).abs() as usize +1;
                if height==0 && width ==0 {
                    self.rectangles.push(((x1, y1), (x2, y2), 0));
                }else if height*width == 0 {
                    self.rectangles.push(((x1, y1), (x2, y2), height+width));
                }else{
                    self.rectangles.push(((x1, y1), (x2, y2), height*width));
                }   
            }
        }
        self.rectangles.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap());
        let max_distance = self.rectangles.first().unwrap().2;
        println!("max distance is {:.2}", max_distance);
        
        //println!("Rectangles are {:?}", self.rectangles);
        max_distance
        
    }

}

pub fn read_document(file_path: &str) -> io::Result<Tile> {
    let path = Path::new(file_path);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut space = Tile::new();
    for line in reader.lines() {
        let line = line?;
        //println!("Read line: {}", line);
        // get all numbers int the line
        for num_str in line.split_whitespace() {
            let coordinates: Vec<&str> = num_str.split(',').collect();
            if coordinates.len() == 2 {
                let x = coordinates[0].trim().parse().unwrap();
                let y = coordinates[1].trim().parse().unwrap();

                space.coordinates.push((x, y));
                space.number_of_coordinates += 1;
                
            }else{
                println!("Invalid coordinate format: {}", num_str);
            }
        } 

    }
    space.red_coordinates = space.coordinates.clone();
    println!("Total processed: {}", space.number_of_coordinates);
    Ok(space)
}


fn main() {

    let file_path = std::env::args().nth(1).expect("no pattern given");

    let mut tile = read_document(&file_path).expect("Failed to read document");
    //println!("tile coordinates: {:?}", tile.coordinates);
    //println!("Ingredients found: {:?}", ingredients.available_ids);
    
    tile.build_tile();
    let result = tile.find_largest_rectangle(); //get_fresh_available_ingredients();
    
    tile.add_green();
    //return;
    let result = tile.find_largest_rectangle_withinbox(); //get_fresh_available_ingredients();
    
    println!("Final answer is {}", result);

    if file_path=="test"{
        let answer = 24; //50;
        assert!(result==answer); 
        return;
    }

}
