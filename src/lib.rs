extern crate rand;
use rand::Rng;
use rand::distributions::normal::Normal;
use rand::distributions::{IndependentSample, Range};
use std::error::Error;
use std::io::prelude::*;
use std::fs::File;

pub fn read_file(filename: &str) -> Result<String, Box<Error>> {
    let mut f = File::open(filename)?;
    let mut string = String::new();
    f.read_to_string(&mut string)?;
    Ok(string)
}

pub fn gen_data(input_list: &str, size_str: &str) -> Result<(), Box<Error>> {
    
    let name_file = read_file(input_list)?;
    let names: Vec<&str>  = name_file.lines().collect();

    let size: u32 = size_str.parse().unwrap();

    let (max_x, max_y): (u32, u32) = (10000, 10000);

    let dots_per_name = size / names.len() as u32;
    
    let mut rng = rand::thread_rng();
    let mean_range = Range::new(0f64, 1.);
    let dev_range = Range::new(15f64, 3000.);
        
    println!("name,x,y"); 
    for name in names {
    
        let x_mean = mean_range.ind_sample(&mut rng) * max_x as f64;
        let y_mean = mean_range.ind_sample(&mut rng) * max_y as f64;

        let mut x_dev = dev_range.ind_sample(&mut rng);
        let mut y_dev = dev_range.ind_sample(&mut rng);

        let diff = (x_dev - y_dev ).abs();
        if diff > 1000. {
            if x_dev > y_dev {
                y_dev += diff * 0.7;
            }
            else {
                x_dev += diff * 0.7;
            }
        }

        let normal_x = Normal::new(x_mean, x_dev);
        let normal_y = Normal::new(y_mean, y_dev);

        let mut v = Vec::new();
        for _i in 0..dots_per_name {

            let x = normal_x.ind_sample(&mut rand::thread_rng());
            let y = normal_y.ind_sample(&mut rand::thread_rng());

            v.push(format!("{},{:.3},{:.3}", name, x, y)); 
        }
        rand::thread_rng().shuffle(&mut v);
        for line in v {
            println!("{}", line);
        }
    }

    Ok(())
}
