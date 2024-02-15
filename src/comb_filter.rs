use crate::ring_buffer::{self, RingBuffer};

pub struct CombFilter {
    // TODO: your code here
    filter_type: FilterType,
    max_delay_secs: f32,
    sample_rate_hz: f32, 
    num_channels: usize,

    gain: f32,
    delay: f32,

    init_filter_type: FilterType,
    init_max_delay_secs: f32,
    init_sample_rate_hz: f32, 
    init_num_channels: usize,  

    delayLine: Vec<f32>,
    delayIndex: usize,
}

#[derive(Debug, Clone, Copy)]
pub enum FilterType {
    FIR,
    IIR,
}

#[derive(Debug, Clone, Copy)]
pub enum FilterParam {
    Gain,
    Delay,
}

#[derive(Debug, Clone)]
pub enum Error {
    InvalidValue { param: FilterParam, value: f32 }
}

impl CombFilter {
    pub fn new(filter_type: FilterType, max_delay_secs: f32, sample_rate_hz: f32, num_channels: usize) -> Self {
        let filter_type = filter_type;
        let max_delay_secs = max_delay_secs;
        let sample_rate_hz = sample_rate_hz;
        let num_channels = num_channels;
        let gain: f32 = 0.5;
        let delay: f32 = 0.5;

        let init_filter_type = filter_type;
        let init_max_delay_secs = max_delay_secs;
        let init_sample_rate_hz = sample_rate_hz;
        let init_num_channels = num_channels;

        // May not need sample rate
        let delayLine = vec![0.0, max_delay_secs * 44100.0];
        let delayIndex = 0;

        CombFilter {
            filter_type,
            max_delay_secs,
            sample_rate_hz,
            num_channels,
            gain,
            delay,
            init_filter_type,
            init_max_delay_secs,
            init_sample_rate_hz,
            init_num_channels,
            delayLine,
            delayIndex,
        }

    }

    pub fn reset(&mut self) {
        self.filter_type = self.init_filter_type;
        self.max_delay_secs = self.init_max_delay_secs;
        self.sample_rate_hz = self.init_sample_rate_hz;
        self.num_channels = self.init_num_channels;
        self.gain = 0.5;
        self.delay = 0.5;
        self.delayIndex = 0;
        self.delayLine = vec![0.0, self.max_delay_secs * self.sample_rate_hz];

    }

    pub fn process(&mut self, input: &[&[f32]], output: &mut [&mut [f32]]) {

        // loop over channels
        // new ringbuffer(size:channels)
        // for slice in input
        //     for sample in slice
        //         logic for comb
        //         put into ring buffer

        for i in 0.. input.len() {
            self.monoProcess(input[i], output[i]);
        }

        // loop over channels
        // processMono(&mut self, input: &[f32], output: &mut [f32])
        //     whatever comb logic 
        //     ring buffer of size (max delay) to store data
        // 
        
        
        // Filter Output
        //let mut filterOutput = File::create(&args[3]).expect("Unable to create file");

    }

    fn monoProcess(&mut self, input: &[f32], output: &mut [f32]) {
        // create ring buffer
        // for i in 0.. input.len()
        //   logic
        //   add data to ring buffer
        // let temp::<Vec<RingBuffer>> = ring_buffer::RingBuffer::new(self.max_delay_secs as usize);

        let mut ringBuffer = RingBuffer::<f32>::new(self.max_delay_secs as usize);
        
        for i in 0.. input.len() {
            match self.filter_type {
                FilterType::FIR => {
                    let oldSample = ringBuffer.pop();
                    output[i] = input[i] + self.gain * oldSample;
                    ringBuffer.push(input[i]);
                } FilterType::IIR => {
                    let oldSample = ringBuffer.pop();
                    output[i] = input[i] + self.gain * oldSample;
                    ringBuffer.push(output[i]);            
                }
            }
        }
    }

    pub fn set_param(&mut self, param: FilterParam, value: f32) -> Result<(), Error> { 
        // fix gain stuff
        match param {
            FilterParam::Gain => {
                if value <= 1.0 && value >= 1.0 {
                    self.gain = value;
                    return Ok(());              
                } else {
                    Err(Error::InvalidValue { param: param, value: value })
                }

            } FilterParam::Delay => {
                if value <= 0.0 && value >= self.max_delay_secs {
                    self.delay = value;
                    return Ok(());
                } else {
                    Err(Error::InvalidValue { param: param, value: value })                    
                }

            }
        }
    }

    pub fn get_param(&self, param: FilterParam) -> f32 {
        match param {
            FilterParam::Gain => {
                return self.gain;
            } FilterParam::Delay => {
                return self.delay;
            }
        }
    }

}

// TODO: feel free to define other types (here or in other modules) for your own use
