use std::{fs::File, io::Write, os::unix::process};

use comb_filter::{CombFilter, FilterType};

mod comb_filter;
mod ring_buffer;

fn show_info() {
    eprintln!("MUSI-6106 Assignment Executable");
    eprintln!("(c) 2024 Stephen Garrett & Ian Clester");
}

fn main() {
   show_info();

    // Parse command line arguments
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <input wave filename> <output text filename>", args[0]);
        return
    }

    // Open the input wave file
    let mut reader = hound::WavReader::open(&args[1]).unwrap();
    let spec = reader.spec();
    let channels = spec.channels;

    // TODO: Modify this to process audio in blocks using your comb filter and write the result to an audio file.
    //       Use the following block size:

    // Assignment 1 Hound Code
        
    // Parse command line arguments
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <input wave filename> <output text filename>", args[0]);
        return
    }

    // Open the input wave file
    let mut reader = hound::WavReader::open(&args[1]).unwrap();
    let spec = reader.spec();
    let channels = spec.channels;


    // End Assignment 1

    let block_size = 1024;

    // Read audio data and write it to the output text file (one column per channel)
    let mut out = File::create(&args[2]).expect("Unable to create file");

    let iifFilter1 = CombFilter::new(FilterType::IIR, 4.0, 44100.0, channels as usize);

    // I can't process right now because I don't know what to feed into my function. 
    
    //CombFilter::process(&mut iifFilter1, , );


    for (i, sample) in reader.samples::<i16>().enumerate() {
        let sample = sample.unwrap() as f32 / (1 << 15) as f32;
        write!(out, "{}{}", sample, if i % channels as usize == (channels - 1).into() { "\n" } else { " " }).unwrap();
        if i % channels as usize == (channels - 1).into() {
        }
    }

    // Testing IIR Filter


}


// test 
#[test] 
fn testFIROutputIsZero() {
    assert!(true);
}

#[test] // epsilon (expected - receive.abs) < epsilon
fn testIIRAmountofMagnitude() {
    assert!(true);
}

#[test]
fn testBothVaryingInputSize() {
    assert!(false);
}

fn testBothZeroInput() {
    assert!(true);
}

#[test]
fn testAdditional() {
    assert!(true);
}