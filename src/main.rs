use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
struct MyData {
    x: u16,
    y: u16,
    samples: Vec<u32>,
}

fn read_file(file_name: &str) -> Result<Vec<u8>, std::io::Error> {
    let mut infile = File::open(file_name)?;
    let mut buffer = Vec::new();
    infile.read_to_end(&mut buffer)?;
    Ok(buffer)
}

fn parse_data(raw_data: &Vec<u8>) -> Vec<MyData> {
    let (num_vec_bytes, rest) = raw_data.split_at(std::mem::size_of::<u64>());
    let mut input = rest;
    let num_vec = u64::from_le_bytes(num_vec_bytes.try_into().unwrap());
    let mut ret_val: Vec<MyData> = Vec::with_capacity(num_vec.try_into().unwrap());

    for _ in 0..num_vec {
        let (x_bytes, rest) = input.split_at(std::mem::size_of::<u16>());
        input = rest;
        let xx = u16::from_le_bytes(x_bytes.try_into().unwrap());
        let (y_bytes, rest) = input.split_at(std::mem::size_of::<u16>());
        input = rest;
        let yy = u16::from_le_bytes(y_bytes.try_into().unwrap());

        let (samples_bytes, rest) = input.split_at(std::mem::size_of::<u64>());
        input = rest;
        let num_samples = u64::from_le_bytes(samples_bytes.try_into().unwrap());
        let mut samples: Vec<u32> = Vec::with_capacity(num_samples.try_into().unwrap());

        for _ in 0..num_samples {
            let (sample_bytes, rest) = input.split_at(std::mem::size_of::<u32>());
            input = rest;
            let sample = u32::from_le_bytes(sample_bytes.try_into().unwrap());
            samples.push(sample);
        }
        ret_val.push(MyData {
            x: xx,
            y: yy,
            samples,
        })
    }
    ret_val
}

fn main() {
    println!("Hello, binary world!");

    let buff = read_file("data.bin");

    let my_data = parse_data(&buff.unwrap());
    for data in my_data {
        println!(" {:04X?} {:04X?} {:04X?}", data.x, data.y, data.samples);
    }
}
