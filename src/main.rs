extern crate cpal;
extern crate futures;

use futures::stream::Stream;
use futures::task;
use futures::task::Executor;
use futures::task::Run;

use std::sync::Arc;

mod synth_thingy;
mod tones;

struct MyExecutor;

impl Executor for MyExecutor {
    fn execute(&self, r: Run) {
        r.run();
    }
}

fn output_sound_thingy(buffer: cpal::UnknownTypeBuffer, format: &cpal::Format, output_buffer: &[f32]) {
    match buffer {
        cpal::UnknownTypeBuffer::U16(mut buffer) => {
            for (sample, value) in buffer.chunks_mut(format.channels.len()).zip(output_buffer.iter()) {
                let value = ((value * 0.5 + 0.5) * std::u16::MAX as f32) as u16;
                for out in sample.iter_mut() { *out = value; }
            }
        },

        cpal::UnknownTypeBuffer::I16(mut buffer) => {
            for (sample, value) in buffer.chunks_mut(format.channels.len()).zip(output_buffer.iter()) {
                let value = (value * std::i16::MAX as f32) as i16;
                for out in sample.iter_mut() { *out = value; }
            }
        },

        cpal::UnknownTypeBuffer::F32(mut buffer) => {
            for (sample, value) in buffer.chunks_mut(format.channels.len()).zip(output_buffer.iter()) {
                for out in sample.iter_mut() { *out = *value; }
            }
        },
    };
}

fn main() {
    let endpoint = cpal::get_default_endpoint().expect("Failed to get default endpoint");
    let format = endpoint.get_supported_formats_list().unwrap().next().expect("Failed to get endpoint format");

    let event_loop = cpal::EventLoop::new();
    let executor = Arc::new(MyExecutor);

    let (mut voice, stream) = cpal::Voice::new(&endpoint, &format, &event_loop).expect("Failed to create a voice");

    let mut synth = synth_thingy::Synth::new(format.samples_rate.0 as i32);

    voice.play();
    task::spawn(stream.for_each(move |buffer| -> Result<_, ()> {

        let mut output_buffer = vec![0.0; buffer.len() / format.channels.len()];

        synth.generate(&mut output_buffer);

        output_sound_thingy(buffer, &format, output_buffer.as_slice());

        Ok(())
    })).execute(executor);

    event_loop.run();
}
