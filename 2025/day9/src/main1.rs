use std::fs::{File, OpenOptions};
use std::io::{self, BufRead};
use std::path::Path;
use rayon::prelude::*;


struct Tile {
    rectangles: Vec<((usize, usize), (usize, usize), usize)>, // corner.0, corner.1, surface
    number_of_columns: usize,
    number_of_lines: usize,
    coordinates: Vec<(usize, usize)>,
    red_coordinates: Vec<(usize, usize)>,
    number_of_coordinates: usize,
    min_coordinates_x: Vec<usize>,
    min_coordinates_y: Vec<usize>,
    max_coordinates_x: Vec<usize>,
    max_coordinates_y: Vec<usize>,
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
            min_coordinates_x: Vec::new(),
            min_coordinates_y: Vec::new(),
            max_coordinates_x: Vec::new(),
            max_coordinates_y: Vec::new(),
        }
    }

    fn build_tile(&mut self) {
        self.number_of_lines = self.coordinates.iter().max_by_key(|x| x.0).map(|x| x.0).unwrap() + 1;
        self.number_of_columns = self.coordinates.iter().max_by_key(|x| x.1).map(|x| x.1).unwrap() + 1;
        
        println!("Tile of size {}*{}", self.number_of_lines, self.number_of_columns);
        self.min_coordinates_y = vec![self.number_of_lines-1;  self.number_of_columns];
        self.min_coordinates_x = vec![self.number_of_columns-1;  self.number_of_lines];
        self.max_coordinates_y = vec![0;  self.number_of_columns];
        self.max_coordinates_x = vec![0;  self.number_of_lines];

        for i in 0..self.coordinates.len() {
            if self.coordinates[i].0 < self.min_coordinates_y[self.coordinates[i].1] {
                self.min_coordinates_y[self.coordinates[i].1]=self.coordinates[i].0;
            }
            if self.coordinates[i].1 < self.min_coordinates_x[self.coordinates[i].0] {
                self.min_coordinates_x[self.coordinates[i].0]=self.coordinates[i].1;
            }
            if self.coordinates[i].0 > self.max_coordinates_y[self.coordinates[i].1] {
                self.max_coordinates_y[self.coordinates[i].1]=self.coordinates[i].0;
            }
            if self.coordinates[i].1 > self.max_coordinates_x[self.coordinates[i].0] {
                self.max_coordinates_x[self.coordinates[i].0]=self.coordinates[i].1;
            }
        } 
        
        println!("minY {:?}", self.min_coordinates_y); 
        println!("maxY {:?}", self.max_coordinates_y); 
        println!("minX {:?}", self.min_coordinates_x);
        println!("maxX {:?}", self.max_coordinates_x);      

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

    /* fn add_green(&mut self){
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
        //self.print(true);

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
        self.print(true);
    }
 
     */
    
    fn find_largest_rectangle_withinbox(&mut self) -> usize{
        println!("tile coordinates: {}", self.coordinates.len());
        let result = 0;
        self.rectangles= Vec::new();
        //self.print(false);
        for i in 0..self.red_coordinates.len() {
            for j in (i+1)..self.red_coordinates.len() {
                println!("{:?}, {:?}", self.coordinates[i], self.coordinates[j]);
                
                let (x1, y1) = self.coordinates[i];
                let (x2, y2) = self.coordinates[j];
                let mut height = (x2 as isize - x1 as isize).abs() as usize; 
                let mut width =  (y2 as isize - y1 as isize).abs() as usize;
                if height==0 && width ==0 {
                    //self.rectangles.push(((x1, y1), (x2, y2), 0));
                }else if height*width == 0 {
                    self.rectangles.push(((x1, y1), (x2, y2), height+width));
                    println!("+1");
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
                    println!("corners {:?}", corners);
                    let mut to_add = vec![true;2];
                    for k in 0..corners.len() {
                        
                        if !self.coordinates.contains(&corners[k]){ // not red cell
                            to_add[k] = false;
                            //break;
                        }
                        let mut l = corners[k].0;
                        let mut p = corners[k].1;
                        if !to_add[k] {
                            
                            println!("{} <= {} <= {}", self.min_coordinates_y[corners[k].1], l, self.max_coordinates_y[corners[k].1]);
                            println!("{} <= {} <= {}", self.min_coordinates_x[corners[k].0], p, self.max_coordinates_x[corners[k].0]);
                            if self.min_coordinates_y[corners[k].1] <= l && l<= self.max_coordinates_y[corners[k].1] || 
                            self.min_coordinates_x[corners[k].0] <= p && p <= self.max_coordinates_x[corners[k].0]{ // green cell
                                to_add[k]=true;
                            }  
                        }
                        if !to_add[k] {
                            
                            /* let count1: usize = self.coordinates
                                .par_iter()
                                .filter(|(a, b)| *a>l && *b >p)
                                .count();
                            let count2: usize = self.coordinates
                                .par_iter()
                                .filter(|(a, b)| *a<l && *b <p)
                                .count();

                            println!("{}-{}", count1, count2); */
                            let n= self.coordinates.len();
                            let count = (0..n)
                                .into_par_iter() // parallel iterator
                                .map(|i| {
                                    let mut tmp = self.coordinates[i];
                                    let xi= tmp.0 as f64;
                                    let yi=tmp.1 as f64;

                                    tmp = self.coordinates[(i + 1) % n];
                                    let xj= tmp.0 as f64;
                                    let yj=tmp.1 as f64;
                                    
                                    if (yi > p as f64) != (yj > p as f64) && (l as f64 )< (xj - xi) * (p as f64 - yi) / (yj - yi + 1e-12) + xi {
                                        1
                                    } else {
                                        0
                                    }
                                })
                                .sum::<i32>();
                            println!("{}", count);
                            if count%2==1 {
                                to_add[k]=true;
                            }
                        }
                        
                    }
                    if to_add[0] && to_add[1] {
                        //print!("A {:?} B {:?} ", self.coordinates[i], self.coordinates[j]);
                        let area = (height+1)*(width+1);
                        //println!("corners {:?} {}", corners, area);
                        println!("+{}", area);
                        self.rectangles.push(((x1, y1), (x2, y2), area));
                    }
                    //break;
                }   
                //println!("rectange {:?}", self.rectangles);

            }
        }
        self.rectangles.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap());
        let max_distance = self.rectangles.first().unwrap().2;
        println!("{:?} : max distance is {:.2}", self.rectangles.first(), max_distance);
        
        println!("Rectangles size {}", self.rectangles.len());
        println!("rectange {:?}", self.rectangles);

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
    
    //tile.add_green();
    //return;
    let result = tile.find_largest_rectangle_withinbox(); //get_fresh_available_ingredients();
    
    println!("Final answer is {}", result);

    if file_path=="test"{
        let answer = 24; //50;
        assert!(result==answer); 
        return;
    }

}
