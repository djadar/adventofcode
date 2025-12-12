use std::fs::{File, OpenOptions};
use std::io::{self, BufRead};
use std::path::Path;
use rayon::prelude::*;
use std::collections::HashMap;
use std::fs;

#[derive(Clone, Default, Debug)]
struct Shape{
    index: usize,
    points: Vec<(usize, usize)>,
}

impl Shape {
    fn new() -> Self {
        Shape {
            index: 0,
            points: Vec::new(),
        }
    }
}


#[derive(Clone, Default, Debug)]
struct Region{
    presents: Vec<usize>,
    width: usize,
    length: usize,
}

impl Region {
    fn new() -> Self {
        Region {
            presents: Vec::new(),
            width: 0,
            length: 0,
        }
    }
}


#[derive(Clone, Default, Debug)]
struct Summary {
    shapes: Vec<Shape>,
    number_of_shapes: usize,
    regions: Vec<Region>,
    number_of_regions: usize,
}

impl Summary {
    fn new() -> Self {
        Summary {
            shapes: Vec::new(),
            number_of_shapes: 0,
            regions: Vec::new(),
            number_of_regions: 0,
        }
    }

    fn number_of_selected_regions(&self) -> usize {
        0
    }
}


pub fn read_document(path: &str) -> Summary {
    let content = fs::read_to_string(path)
        .expect("Cannot read file");

    let mut shapes: Vec<Shape> = Vec::new();
    let mut regions: Vec<Region> = Vec::new();

    let mut lines = content.lines().peekable();

    // ------------------------------
    // 1. Parse Shapes
    // ------------------------------
    while let Some(line) = lines.peek() {
        let trimmed = line.trim();

        // Stop when reaching region definitions (e.g. "4x4:")
        if trimmed.contains('x') && trimmed.contains(':') {
            break;
        }

        if trimmed.ends_with(':') {
            // Example: "0:"
            let shape_index: usize = trimmed[..trimmed.len() - 1].parse().unwrap();
            lines.next(); // consume this line

            let mut points = Vec::new();
            let mut row = 0;

            // Read consecutive ### lines
            while let Some(shape_line) = lines.peek() {
                let shape_line = shape_line.trim();
                if shape_line.is_empty() || shape_line.ends_with(':') {
                    break;
                }

                // record '#' positions
                for (col, ch) in shape_line.chars().enumerate() {
                    if ch == '#' {
                        points.push((row, col));
                    }
                }

                row += 1;
                lines.next();
            }

            shapes.push(Shape { index: shape_index, points });
        } else {
            lines.next();
        }
    }

    // ------------------------------
    // 2. Parse Regions
    // ------------------------------
    while let Some(line) = lines.next() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        // Example: "4x4: 0 0 0 0 2 0"
        if let Some((dim, values)) = trimmed.split_once(':') {
            let dim = dim.trim();
            let values = values.trim();

            let (width_str, length_str) = dim.split_once('x').unwrap();
            let width: usize = width_str.parse().unwrap();
            let length: usize = length_str.parse().unwrap();

            let presents: Vec<usize> =
                values.split_whitespace().map(|v| v.parse().unwrap()).collect();

            regions.push(Region { presents, width, length });
        }
    }

    Summary {
        number_of_shapes: shapes.len(),
        shapes,
        number_of_regions: regions.len(),
        regions,
    }
}



fn main() {

    let file_path = std::env::args().nth(1).expect("no pattern given");
    
    let mut summary = read_document(&file_path);
    println!("Summary loaded successfully {:?}", summary);

    let result = summary.number_of_selected_regions();
    println!("Final result: {}", result);

    if file_path=="test"{
        assert!(result==2); 
        return;
    }
}
