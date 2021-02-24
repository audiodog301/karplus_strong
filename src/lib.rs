
#![allow(incomplete_features)]
#![feature(generic_associated_types)]

use serde::{Serialize, Deserialize};

use baseplug::{
    ProcessContext,
    Plugin,
};

struct Delay {
    buffer: [f32; 2000],
    time: u32
}

impl Delay {
    pub fn new(time: u32) -> Self {
        Self {
            buffer: [0.0; 2000],
            time: time
        }
    }    
    
    fn process(&mut self, input: f32) -> f32 {
        for counter in 0..self.buffer.len() {
            self.buffer[counter + 1] = self.buffer[counter];    
        }

        self.buffer[0] = input;

        self.buffer[self.time as usize]
    }
        
}

baseplug::model! {
    #[derive(Debug, Serialize, Deserialize)]
    struct FilterModel {
        #[model(min = 0.0, max = 2000.0)]
        #[parameter(name = "amount")]
        amount: f32
    }
}

impl Default for FilterModel {
    fn default() -> Self {
        Self {
            amount: 0.0
        }
    }
}

struct Filter {
    delay_left: Delay,
    delay_right: Delay
}

impl Plugin for Filter {
    const NAME: &'static str = "Baby\'s First Delay Plugin";
    const PRODUCT: &'static str = "Baby\'s First Delay Plugin";
    const VENDOR: &'static str = "audiodog301";

    const INPUT_CHANNELS: usize = 2;
    const OUTPUT_CHANNELS: usize = 2;

    type Model = FilterModel;

    #[inline]
    fn new(_sample_rate: f32, _model: &FilterModel) -> Self {
        Self {
            delay_left: Delay::new(0),
            delay_right: Delay::new(0)
        }
    }

    #[inline]
    fn process(&mut self, model: &FilterModelProcess, ctx: &mut ProcessContext<Self>) {
        let input = &ctx.inputs[0].buffers;
        let output = &mut ctx.outputs[0].buffers;
        
        for i in 0..ctx.nframes { 
            output[0][i] = self.delay_left.process(input[0][i]);
            output[1][i] = self.delay_right.process(input[1][i]);
        }
    }            
}

baseplug::vst2!(Filter, b"hehe");