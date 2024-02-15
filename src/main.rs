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
    let mut reader: hound::WavReader<std::io::BufReader<File>> = hound::WavReader::open(&args[1]).unwrap();
    let spec = reader.spec();
    let channels = spec.channels;


    // End Assignment 1

    let block_size = 1024;

    // Read audio data and write it to the output text file (one column per channel)
    let mut out = File::create(&args[2]).expect("Unable to create file");

    let iifFilter1 = CombFilter::new(FilterType::IIR, 4.0, 44100.0, channels as usize);

    // I can't process right now because I don't know what to feed into my function. 
    
    for (i, sample) in reader.samples::<i16>().enumerate() {
        let sample = sample.unwrap() as f32 / (1 << 15) as f32;
        write!(out, "{}{}", sample, if i % channels as usize == (channels - 1).into() { "\n" } else { " " }).unwrap();
        if i % channels as usize == (channels - 1).into() {
        }
    }

    // let reader: hound::WavReader<std::io::BufReader<File>> = hound::WavReader::open(&args[1]).unwrap();

    // // Read all samples into a Vec<f32>
    // let samples: Vec<f32> = reader.unwrap();

    // // Determine the number of channels
    // let num_channels = reader.spec().channels as usize;

    // // Create slices for each channel
    // let slices: &Vec<&[f32]> = samples.chunks(num_channels).collect();

    // CombFilter::process(&mut iifFilter1, slices, &mut [&mut []]);
    
}


// test 
#[test] 
fn testFIROutputIsZero() {
    let mut FIRZeroInput: &[&[f32]] = &[&[1.0]];
    let mut FIRZeroOutput: &mut [&mut [f32]] = &mut [&mut []];

    let mut FIRZeroFilter = CombFilter::new(FilterType::FIR, 1.0, 1.0, 1);
    CombFilter::process(&mut FIRZeroFilter, FIRZeroInput, FIRZeroOutput);

    assert!(FIRZeroOutput == FIRZeroInput);
}

#[test] // epsilon (expected - receive.abs) < epsilon

// Checks to see if IIR is equal to input and if the gain is 1. 
fn testIIRAmountofMagnitude() {
    let mut IIRAmountofMagnitude: &[&[f32]] = &[&[1.0]];
    let mut IIRAmountOutput: &mut [&mut [f32]] = &mut [&mut []];
    let mut IIRAmountFilter = CombFilter::new(FilterType::FIR, 1.0, 1.0, 1);

    CombFilter::set_param(&mut IIRAmountFilter, comb_filter::FilterParam::Gain , 1.0).expect("Unable to set Param");
    CombFilter::process(&mut IIRAmountFilter, IIRAmountofMagnitude, IIRAmountOutput);

    assert!(IIRAmountofMagnitude == IIRAmountOutput && CombFilter::get_param(&mut IIRAmountFilter, comb_filter::FilterParam::Gain) == 1.0);
}

#[test]
fn testBothVaryingInputSize() {       
    let mut varyingInput: &[&[f32]] = &[&[1.0, 2.0], &[1.0, 2.0, 3.0, 4.0]];
    let mut varyingOutput: &mut [&mut [f32]] = &mut [&mut []];
    let mut varyingFilter = CombFilter::new(FilterType::FIR, 1.0, 1.0, 1);

    CombFilter::set_param(&mut varyingFilter, comb_filter::FilterParam::Gain , 1.0).expect("Unable to set Param");
    CombFilter::process(&mut varyingFilter, varyingInput, varyingOutput); 

    assert!(varyingOutput == varyingInput);

}

#[test]
fn testIIRZeroInput() {
    let mut IIRZeroInput: &[&[f32]] = &[&[]];
    let mut IIRZeroOutput: &mut [&mut [f32]] = &mut [&mut []];
    let mut IIRZeroFilter = CombFilter::new(FilterType::IIR, 0.0, 44100.0, 0);
    CombFilter::process(&mut IIRZeroFilter, IIRZeroInput, IIRZeroOutput);

    assert!(IIRZeroOutput == &[&[]]);
}

#[test]
fn testFIRZeroInput() {
    let mut FIRZeroInput: &[&[f32]] = &[&[]];
    let mut FIRZeroOutput: &mut [&mut [f32]] = &mut [&mut []];
    let mut FIRZeroFilter = CombFilter::new(FilterType::FIR, 0.0, 44100.0, 0);
    CombFilter::process(&mut FIRZeroFilter, FIRZeroInput, FIRZeroOutput);
    assert!(FIRZeroOutput == &[&[]]);
}

#[test]
fn testGainParams() {
    let mut setParamFilter = CombFilter::new(FilterType::FIR, 3.0, 44100.0, 0);
    CombFilter::set_param(&mut setParamFilter, comb_filter::FilterParam::Gain , 0.2).expect("Unable to set Param");
    assert!(setParamFilter.get_param(comb_filter::FilterParam::Gain) == 0.2);
}

#[test]
fn testDelayParams() {
    let mut setDelayFilter = CombFilter::new(FilterType::FIR, 3.0, 44100.0, 0);
    CombFilter::set_param(&mut setDelayFilter, comb_filter::FilterParam::Delay, 0.1).expect("Unable to set Param");
    assert!(setDelayFilter.get_param(comb_filter::FilterParam::Delay) == 0.1);
}
