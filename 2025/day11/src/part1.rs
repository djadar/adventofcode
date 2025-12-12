use std::fs::{File, OpenOptions};
use std::io::{self, BufRead};
use std::path::Path;
use rayon::prelude::*;
use std::collections::HashMap;


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
        let position = "you";
        self.find_path_to_out(position.to_string())         
        
    }

    fn find_path_to_out(&self, position:String) -> usize{
        let mut result = 0; 
        if position !="out" {
            if let Some(attached_devices) = self.device_list.get(&position.to_string())
            {
                println!("attached_devices {:?}", attached_devices);
                for device in attached_devices {
                    println!("device {}", device);
                    result += self.find_path_to_out(device.to_string());
                }
            }   
            
        }else {
            result = 1;
        }
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
        let answer = 5;
        assert!(result==answer); 
        return;
    }

}
