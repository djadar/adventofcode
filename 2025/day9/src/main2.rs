use rayon::prelude::*;
use rayon::ThreadPoolBuilder;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
struct Tile {
    rectangles: Vec<((usize, usize), (usize, usize), usize)>,
    coordinates: Vec<(usize, usize)>,
    coordinate_set: HashSet<(usize, usize)>,
    red_coordinates: Vec<(usize, usize)>,
    red_set: HashSet<(usize, usize)>,
    number_of_columns: usize,
    number_of_lines: usize,
    number_of_coordinates: usize,
}

impl Tile {
    fn new() -> Self {
        Tile {
            rectangles: Vec::new(),
            coordinates: Vec::new(),
            coordinate_set: HashSet::new(),
            red_coordinates: Vec::new(),
            red_set: HashSet::new(),
            number_of_columns: 0,
            number_of_lines: 0,
            number_of_coordinates: 0,
        }
    }

    fn sync_sets(&mut self) {
        self.coordinate_set.clear();
        self.coordinate_set.extend(self.coordinates.iter().copied());
        self.red_set.clear();
        self.red_set.extend(self.red_coordinates.iter().copied());
    }

    fn build_tile(&mut self) {
        let (max_x, max_y) = self
            .coordinates
            .iter()
            .fold((0usize, 0usize), |(mx, my), &(x, y)| (mx.max(x), my.max(y)));
        self.number_of_lines = max_x + 1;
        self.number_of_columns = max_y + 1;
        println!("Step 1: Tile built with size {}x{}", self.number_of_lines, self.number_of_columns);
    }

        fn add_green(&mut self) -> usize {
        println!("Step 2: Adding green points in one pass...");

        let mut new_points: Vec<(usize, usize)> = Vec::new();

        // Work on red points only
        let reds = self.red_coordinates.clone();

        for i in 0..reds.len() {
            let (x1, y1) = reds[i];
            for j in (i + 1)..reds.len() {
                let (x2, y2) = reds[j];
                let height = x2.abs_diff(x1);
                let width = y2.abs_diff(y1);

                if width != 0 && height == 0 {
                    // horizontal line
                    let (min_y, max_y) = if y1 <= y2 { (y1, y2) } else { (y2, y1) };
                    for a in (min_y + 1)..max_y {
                        let p = (x1, a);
                        if !self.coordinate_set.contains(&p) {
                            new_points.push(p);
                        }
                    }
                } else if height != 0 && width == 0 {
                    // vertical line
                    let (min_x, max_x) = if x1 <= x2 { (x1, x2) } else { (x2, x1) };
                    for b in (min_x + 1)..max_x {
                        let p = (b, y1);
                        if !self.coordinate_set.contains(&p) {
                            new_points.push(p);
                        }
                    }
                }
            }
        }

        // Deduplicate and insert all at once
        new_points.sort_unstable();
        new_points.dedup();

        for p in &new_points {
            self.coordinates.push(*p);
            self.coordinate_set.insert(*p);
        }

        let added = new_points.len();
        println!("Step 2 complete: {} green points added in one pass.", added);
        added
    }

    fn add_green2(&mut self) -> usize {
        println!("Step 2: Filling green points...");
        let mut total_added = 0usize;

        loop {
            let mut new_points: Vec<(usize, usize)> = Vec::new();
            let coords = self.coordinates.clone();

            for i in 0..coords.len() {
                let (x1, y1) = coords[i];
                for j in (i + 1)..coords.len() {
                    let (x2, y2) = coords[j];
                    let height = x2.abs_diff(x1);
                    let width = y2.abs_diff(y1);

                    if width != 0 && height == 0 {
                        let (min_y, max_y) = if y1 <= y2 { (y1, y2) } else { (y2, y1) };
                        let mut blocked = false;
                        for a in (min_y + 1)..max_y {
                            if self.coordinate_set.contains(&(x1, a)) {
                                blocked = true;
                                break;
                            }
                        }
                        if !blocked {
                            for a in (min_y + 1)..max_y {
                                let p = (x1, a);
                                if !self.coordinate_set.contains(&p) {
                                    new_points.push(p);
                                }
                            }
                        }
                    } else if height != 0 && width == 0 {
                        let (min_x, max_x) = if x1 <= x2 { (x1, x2) } else { (x2, x1) };
                        let mut blocked = false;
                        for b in (min_x + 1)..max_x {
                            if self.coordinate_set.contains(&(b, y1)) {
                                blocked = true;
                                break;
                            }
                        }
                        if !blocked {
                            for b in (min_x + 1)..max_x {
                                let p = (b, y1);
                                if !self.coordinate_set.contains(&p) {
                                    new_points.push(p);
                                }
                            }
                        }
                    }
                }
            }

            if new_points.is_empty() {
                break;
            }
            new_points.sort_unstable();
            new_points.dedup();

            for p in &new_points {
                self.coordinates.push(*p);
                self.coordinate_set.insert(*p);
            }
            total_added += new_points.len();
            println!("   Added {} new green points (total so far: {})", new_points.len(), total_added);
        }

        println!("Step 2 complete: {} green points added.", total_added);
        total_added
    }

    fn find_largest_rectangle_withinbox_parallel(&mut self) -> usize {
        println!("Step 3: Searching largest rectangle...");
        self.rectangles.clear();

        let red = self.red_coordinates.clone();
        let coord_set = self.coordinate_set.clone();

        let rectangles: Vec<((usize, usize), (usize, usize), usize)> = (0..red.len())
            .into_par_iter()
            .flat_map(|i| {
                let (x1, y1) = red[i];
                let mut local = Vec::new();
                for j in (i + 1)..red.len() {
                    let (x2, y2) = red[j];
                    let height = x2.abs_diff(x1);
                    let width = y2.abs_diff(y1);

                    if height == 0 && width == 0 {
                        continue;
                    } else if height * width == 0 {
                        local.push(((x1, y1), (x2, y2), height + width));
                    } else {
                        let min_x = x1.min(x2);
                        let max_x = x1.max(x2);
                        let min_y = y1.min(y2);
                        let max_y = y1.max(y2);

                        let corners = [
                            (min_x, min_y),
                            (min_x, max_y),
                            (max_x, min_y),
                            (max_x, max_y),
                        ];

                        if corners.iter().all(|c| coord_set.contains(c)) {
                            let area = (height + 1) * (width + 1);
                            local.push(((x1, y1), (x2, y2), area));
                        }
                    }
                }
                local
            })
            .collect();

        self.rectangles = rectangles;
        self.rectangles.sort_by(|a, b| b.2.cmp(&a.2));

        let max_area = self.rectangles.first().map(|r| r.2).unwrap_or(0);
        println!("Step 3 complete: Largest rectangle area = {}", max_area);
        max_area
    }
}

pub fn read_document(file_path: &str) -> io::Result<Tile> {
    let path = Path::new(file_path);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut tile = Tile::new();
    let mut parsed = 0usize;

    println!("Step 0: Reading input file...");
    for (line_idx, line) in reader.lines().enumerate() {
        let line = line?;
        for num_str in line.split_whitespace() {
            let parts: Vec<&str> = num_str.split(',').collect();
            if parts.len() == 2 {
                if let (Ok(x), Ok(y)) = (parts[0].trim().parse::<usize>(), parts[1].trim().parse::<usize>()) {
                    tile.coordinates.push((x, y));
                    parsed += 1;
                } else {
                    println!("Line {}: Invalid coordinate '{}'", line_idx + 1, num_str);
                }
            }
        }
    }

    tile.number_of_coordinates = parsed;
    tile.red_coordinates = tile.coordinates.clone();
    tile.sync_sets();

    println!("Step 0 complete: {} coordinates loaded.", tile.number_of_coordinates);
    Ok(tile)
}

fn main() {
    ThreadPoolBuilder::new()
        .num_threads(num_cpus::get())
        .build_global()
        .unwrap();

    println!("Using {} threads for Rayon", num_cpus::get());

    let file_path = std::env::args().nth(1).expect("no pattern given");
    let mut tile = read_document(&file_path).expect("Failed to read document");

    tile.build_tile();
    tile.add_green();
    let result = tile.find_largest_rectangle_withinbox_parallel();

    println!("Final Answer: {}", result);
}
