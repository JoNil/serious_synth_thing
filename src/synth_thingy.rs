use tones::*;

pub struct Synth {
    phase: i32,
    sample_rate: i32,
    tones: Vec<f32>,
    tone_index: i32,
    counter: i32,
    bpm: i32,
}

impl Synth {

    pub fn new(sample_rate: i32) -> Synth {
        Synth {
            phase: 0,
            sample_rate: sample_rate,
            tones: vec![C5, F5, G5, A5, D5, F5, G5, B5, C5, C5, F5, G5, A5],
            tone_index: 0,
            counter: 0,
            bpm: 120,
        }
    }

    pub fn generate(&mut self, buffer: &mut [f32]) {
        for sample in buffer.iter_mut() {

            self.counter += 1;

            if self.counter > self.sample_rate / (self.bpm / 60 * 4) {
                self.counter = 0;
                self.tone_index += 1;
            }

            if self.tone_index >= self.tones.len() as i32 {
                self.tone_index = 0;
            }

            *sample = (self.phase as f32 * self.tones[self.tone_index as usize] * 2.0 * 3.141592 / self.sample_rate as f32).sin();
            self.phase += 1;
        }
    }
}
