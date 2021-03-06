use fon::{mono::Mono64, Audio, Sink};
use twang::{Fc, Signal, Synth};

mod wav;

// Target sample rate set to 48 KHz
const S_RATE: u32 = 48_000;

fn sine(_: &mut (), fc: Fc) -> Signal {
    fc.freq(440.0).sine()
}

fn main() {
    // Initialize audio with five seconds of silence.
    let mut audio = Audio::<Mono64>::with_silence(S_RATE, S_RATE as usize * 5);
    // Create the synthesizer.
    let mut synth = Synth::new((), sine);

    audio.sink(..).stream(&mut synth);

    // Write synthesized audio to WAV file.
    wav::write(audio, "sine.wav").expect("Failed to write WAV file");
}
