use crate::adsr::Adsr;
use crate::wavetables::{
    WavetablePlayer, SAWTOOTH_WAVETABLE, SINE_WAVETABLE, SQUARE_WAVETABLE, WAVETABLE_SIZE,
};

pub trait Synth {
    fn new() -> Self;
    fn update(&mut self, elapsed_time_us: u32) -> u8;
    fn note_on(&mut self, note: u8, velocity: u8);
    fn note_off(&mut self, note: u8);
    fn attack_control(&mut self, attack_ms: u16);
    fn decay_control(&mut self, decay_ms: u16);
    fn sustain_control(&mut self, sustain_level: u16);
    fn release_control(&mut self, release_ms: u16);
    fn set_wavetable(&mut self, wavetable: &'static [u8; WAVETABLE_SIZE]);
    fn portamento_control(&mut self, portamento_time_ms: u16);
    fn channel_aftertouch(&mut self, aftertouch: u8);
}

struct MonoSynth {
    oscilator: WavetablePlayer,
    adsr: Adsr,
}

impl Synth for MonoSynth {
    fn new() -> Self {
        let wavetable_player = WavetablePlayer::new(&SAWTOOTH_WAVETABLE, 69);
        let adsr = Adsr::new();
        Self {
            oscilator: wavetable_player,
            adsr: adsr,
        }
    }

    fn update(&mut self, elapsed_time_us: u32) -> u8 {
        let level = self.adsr.update(elapsed_time_us);
        let sample = (self.oscilator.next_sample(elapsed_time_us) as u32 * level as u32)
            / crate::adsr::MAX_LEVEL as u32;
        sample as u8
    }

    fn note_on(&mut self, note: u8, velocity: u8) {
        self.oscilator.set_midi_note(note);
        self.adsr.trigger(velocity);
    }

    fn note_off(&mut self, _note: u8) {
        self.adsr.release();
    }

    fn attack_control(&mut self, attack_ms: u16) {
        self.adsr.set_attack(attack_ms as u32);
    }

    fn decay_control(&mut self, decay_ms: u16) {
        self.adsr.set_decay(decay_ms as u32);
    }

    fn sustain_control(&mut self, sustain_level: u16) {
        self.adsr
            .set_sustain(core::cmp::min(sustain_level as u32, crate::adsr::MAX_LEVEL));
    }

    fn release_control(&mut self, release_ms: u16) {
        self.adsr.set_release(release_ms as u32);
    }

    fn set_wavetable(&mut self, wavetable: &'static [u8; WAVETABLE_SIZE]) {
        self.oscilator.set_wavetable(wavetable);
    }

    fn portamento_control(&mut self, portamento_time_ms: u16) {
        self.oscilator.set_portamento(portamento_time_ms as u32);
    }

    fn channel_aftertouch(&mut self, aftertouch: u8) {
        self.adsr.set_aftertouch(aftertouch as u32 + 127);
    }
}

pub struct PolySynth {
    voices: [MonoSynth; 5],
    active_voices: [bool; 5],
}

impl Synth for PolySynth {
    fn new() -> Self {
        let voices = [
            MonoSynth::new(),
            MonoSynth::new(),
            MonoSynth::new(),
            MonoSynth::new(),
            MonoSynth::new(),
        ];
        Self {
            voices,
            active_voices: [false, false, false, false, false],
        }
    }

    fn update(&mut self, elapsed_time_us: u32) -> u8 {
        let mut sample = 0;
        for voice in self.voices.iter_mut() {
            sample += voice.update(elapsed_time_us) as u32;
        }
        sample as u8
    }

    fn note_on(&mut self, note: u8, velocity: u8) {
        let mut voice_index = 0;
        for (i, active) in self.active_voices.iter().enumerate() {
            if !*active {
                voice_index = i;
                break;
            }
        }
        self.voices[voice_index].note_on(note, velocity);
        self.active_voices[voice_index] = true;
    }

    fn note_off(&mut self, note: u8) {
        for (voice, active) in self.voices.iter_mut().zip(self.active_voices.iter_mut()) {
            if *active && voice.oscilator.get_midi_note() == note {
                voice.note_off(note);
                *active = false;
            }
        }
    }

    fn attack_control(&mut self, attack_ms: u16) {
        for voice in self.voices.iter_mut() {
            voice.attack_control(attack_ms);
        }
    }

    fn decay_control(&mut self, decay_ms: u16) {
        for voice in self.voices.iter_mut() {
            voice.decay_control(decay_ms);
        }
    }

    fn sustain_control(&mut self, sustain_level: u16) {
        for voice in self.voices.iter_mut() {
            voice.sustain_control(sustain_level);
        }
    }

    fn release_control(&mut self, release_ms: u16) {
        for voice in self.voices.iter_mut() {
            voice.release_control(release_ms);
        }
    }

    fn set_wavetable(&mut self, wavetable: &'static [u8; WAVETABLE_SIZE]) {
        for voice in self.voices.iter_mut() {
            voice.set_wavetable(wavetable);
        }
    }

    fn portamento_control(&mut self, portamento_time_ms: u16) {
        for voice in self.voices.iter_mut() {
            voice.oscilator.set_portamento(portamento_time_ms as u32);
        }
    }

    fn channel_aftertouch(&mut self, aftertouch: u8) {
        for voice in self.voices.iter_mut() {
            voice.adsr.set_aftertouch(aftertouch as u32);
        }
    }
}
