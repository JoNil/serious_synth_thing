//initial commit
fn Buffer() {
    
}

pub struct Synth {
    phase: i32,
    sample_rate: u32,
}

impl Synth {

    pub fn new(sample_rate: u32) -> Synth {
        Synth {
            phase: 0,
            sample_rate: sample_rate,
        }
    } 

    pub fn generate(&mut self, buffer: &mut [f32]) {
        for sample in buffer.iter_mut() {
            *sample = (self.phase as f32 * 440.0 * 2.0 * 3.141592 / self.sample_rate as f32).sin());
            self.phase += 1;
        }
    }
}
