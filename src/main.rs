use rand::prelude::*;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let mut file_index: u32 = 001;
    for _ in 0..1 {
        let path_string = format!("../output/matrix{}.txt", file_index);
        let path = Path::new(&path_string);
        let display = path.display();
        let mut file = match File::create(&path) {
            Err(why) => panic!("couldn't create {}: {}", display, why),
            Ok(file) => file,
        };

        let size: u32 = thread_rng().gen_range(4, 10);
        let concentration: f64 = thread_rng().gen_range(1.0, 4.0);
        // let first_row = format!("{}\n", size.clone().to_string());
        // match file.write_all(first_row.as_bytes()) {
        //     Err(why) => panic!("couldn't write to {}: {}", display, why),
        //     Ok(_) => println!("successfully wrote to {}", display),
        // }
        for _ in 0..size {
            for i in 0..size {
                if rand::thread_rng().gen_bool(1.0 / concentration) {
                    let random_i32 = rand::thread_rng().gen_range(1, 10);
                    let mut num = String::new();
                    if i < size - 1 {
                        num = format!("{} ", random_i32.to_string());
                    } else {
                        num = format!("{}", random_i32.to_string());
                    }

                    match file.write_all(num.as_bytes()) {
                        Err(why) => panic!("couldn't write to {}: {}", display, why),
                        Ok(_) => println!("successfully wrote to {}", display),
                    }
                } else {
                    let num: String;
                    if i < size - 1 {
                        num = format!("{} ", 0.to_string());
                    } else {
                        num = format!("{}", 0.to_string());
                    }
                    match file.write_all(num.as_bytes()) {
                        Err(why) => panic!("couldn't write to {}: {}", display, why),
                        Ok(_) => println!("successfully wrote to {}", display),
                    }
                }
            }
            match file.write_all("\n".as_bytes()) {
                Err(why) => panic!("couldn't write to {}: {}", display, why),
                Ok(_) => println!("successfully wrote to {}", display),
            }
        }
        file_index += 1;
    }
}
