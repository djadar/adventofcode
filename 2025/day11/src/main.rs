use std::fs::{File, OpenOptions};
use std::io::{self, BufRead};
use std::path::Path;
use rayon::prelude::*;
use std::collections::HashMap;

type Mask = u8; // bit 0 = dac, bit 1 = fft

#[derive(Clone, Default, Debug)]
struct Server {
    device_list: HashMap<String, Vec<String>>, 
    number_of_devices: usize,
}

impl Server {
    fn new() -> Self {
        Server {
            device_list: HashMap::new(),
            number_of_devices: 0,
        }
    }

    fn find_number_of_paths(&self) -> usize {
        let position = "svr";
        let visited_dac_and_fft= vec![false, false];
        //self.find_path_to_out(position.to_string(), visited_dac_and_fft)         
        let mut memo = HashMap::new(); // memoization to precompute paths
        self.find_path_to_out(position, 0, &mut memo)
        
    }

    /* fn find_path_to_out1(&self, position:String, mut visited_dac_and_fft: Vec<bool>) -> usize{
        let mut result = 0; 
        if position !="out" {
            if let Some(attached_devices) = self.device_list.get(&position.to_string())
            {
                println!("attached_devices {:?}", attached_devices);
                for device in attached_devices {
                    //println!("device {}", device);
                    visited_dac_and_fft[0] = visited_dac_and_fft[0] || (device=="dac");
                    visited_dac_and_fft[1] = visited_dac_and_fft[1] || (device=="fft");
                    result += self.find_path_to_out(
                        device.to_string(), visited_dac_and_fft.clone());
                }
            }   
            
        }else {
            if visited_dac_and_fft[0] && visited_dac_and_fft[1] {
                result = 1;
            }
        }
        result
    }

    fn find_path_to_out2(
        &self,
        position: String,
        visited: Vec<bool>,
    ) -> usize {
        if position == "out" {
            // Check if both devices were visited
            return usize::from(visited[0] && visited[1]);
        }

        let Some(attached_devices) = self.device_list.get(&position) else {
            return 0;
        };

        attached_devices
            .par_iter()                 // ← parallel iteration
            .map(|device| {
                // update visit vector locally (no sharing!)
                let mut local_visited = visited.clone();
                local_visited[0] |= device == "dac";
                local_visited[1] |= device == "fft";

                // recursive parallel search
                self.find_path_to_out(device.clone(), local_visited)
            })
            .sum()
    }
 */
    

    fn find_path_to_out(&self, pos: &str, mask: Mask, memo: &mut HashMap<(String, Mask), usize>) -> usize {
        if let Some(&cached) = memo.get(&(pos.to_string(), mask)) {
            return cached;
        }

        let result = if pos == "out" {
            usize::from(mask == 0b11) // 1 if dac and fft found
        } else {
            self.device_list[pos]
                .iter()
                .map(|dev| {
                    let mut new_mask = mask;
                    if dev == "dac" { new_mask |= 0b01; }
                    if dev == "fft" { new_mask |= 0b10; }
                    self.find_path_to_out(dev, new_mask, memo)
                })
                .sum()
        };

        memo.insert((pos.to_string(), mask), result);
        result
    }

}

pub fn read_document(file_path: &str) -> io::Result<Server> {
    let path = Path::new(file_path);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut server = Server::new();
    for line in reader.lines() {
        let line = line?;

        let block: Vec<&str> = line.split(':').collect();
        if block.len() == 2 {
            let device = block[0].to_string();

            //attached devices
            let mut vec = Vec::new();
            for device in block[1].split_whitespace() {
                vec.push(device.to_string());
            }
            server.device_list.insert(device, vec);
            server.number_of_devices +=1;
        }else{
            println!("Wrong output");
        }
        
    }
    println!("Total processed: {}", server.number_of_devices);
    Ok(server)
}


fn main() {

    let file_path = std::env::args().nth(1).expect("no pattern given");

    let mut server = read_document(&file_path).expect("Failed to read document");
    println!("Server {:?}", server);
    
    let result = server.find_number_of_paths();
    
    println!("Final answer is {}", result);

    if file_path=="test"{
        let answer = 2;
        assert!(result==answer); 
        return;
    }

}
