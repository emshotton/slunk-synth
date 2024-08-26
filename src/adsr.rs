use defmt::info;
pub const MAX_LEVEL: u32 = 4095;
const DEFAULT_ATTACK_MS: u32 = 100;
const DEFAULT_DECAY_MS: u32 = 50;
const DEFAULT_RELEASE_MS: u32 = 500;

#[derive(Debug, PartialEq)]
pub struct Adsr {
    attack_ms: u32,
    decay_ms: u32,
    sustain_level: u32,
    aftertouch: u32,
    release_ms: u32,
    state: AdsrState,
    time_us: u32,
    triggered: bool,
    velocity: u32,
}

#[derive(Debug, PartialEq)]
pub enum AdsrState {
    Attack,
    Decay,
    Sustain,
    Release,
    Done,
}

impl Adsr {
    pub fn new() -> Self {
        Self {
            attack_ms: DEFAULT_ATTACK_MS,
            decay_ms: DEFAULT_DECAY_MS,
            sustain_level: MAX_LEVEL / 3,
            release_ms: DEFAULT_RELEASE_MS,
            aftertouch: 0,
            state: AdsrState::Done,
            time_us: 0,
            triggered: false,
            velocity: 127,
        }
    }

    pub fn set_attack(&mut self, attack_ms: u32) {
        self.attack_ms = attack_ms;
    }

    pub fn set_decay(&mut self, decay_ms: u32) {
        self.decay_ms = decay_ms;
    }

    pub fn set_sustain(&mut self, sustain_level: u32) {
        self.sustain_level = sustain_level;
    }

    pub fn set_aftertouch(&mut self, aftertouch: u32) {
        self.aftertouch = aftertouch;
    }

    pub fn set_release(&mut self, release_ms: u32) {
        self.release_ms = release_ms;
    }

    pub fn trigger(&mut self, velocity: u8) {
        // When triggered reset the time and set the state to attack
        self.velocity = velocity as u32;
        self.triggered = true;
        self.time_us = 0;
        self.state = AdsrState::Attack;
    }

    pub fn release(&mut self) {
        self.triggered = false;
    }

    pub fn update(&mut self, dt_us: u32) -> u16 {
        self.time_us += dt_us;
        let time_ms = self.time_us / 1000;

        match self.state {
            AdsrState::Done => {
                // The trigger() function will reset the state to attack
                self.time_us = 0;
                return 0;
            }
            AdsrState::Attack => {
                if time_ms >= self.attack_ms {
                    self.state = AdsrState::Decay;
                    return MAX_LEVEL as u16;
                }
                let level = (((MAX_LEVEL * time_ms / u32::max(self.attack_ms, 1)) * self.velocity)
                    / 127) as u16;
                return level;
            }
            AdsrState::Decay => {
                if time_ms >= self.attack_ms + self.decay_ms {
                    self.state = AdsrState::Sustain;
                    return self.sustain_level as u16;
                }

                return (((MAX_LEVEL
                    - (time_ms - u32::min(self.attack_ms, time_ms))
                        * (MAX_LEVEL - u32::min(self.sustain_level, MAX_LEVEL))
                        / u32::max(self.decay_ms, 1))
                    * self.velocity)
                    / 127) as u16;
            }
            AdsrState::Sustain => {
                // check to see if the note has been released
                if !self.triggered {
                    self.state = AdsrState::Release;
                    self.time_us = 0;
                }

                let sustain = u16::min(
                    (((self.sustain_level * self.velocity) / 127) + self.aftertouch) as u16,
                    MAX_LEVEL as u16,
                );
                return sustain;
            }
            AdsrState::Release => {
                if time_ms > self.release_ms {
                    self.state = AdsrState::Done;
                    self.time_us = 0;
                    return 0;
                }

                let sustain_diff = time_ms * self.sustain_level / u32::max(self.release_ms, 1);
                let value = ((self.sustain_level - sustain_diff) * self.velocity) / 127;
                return value as u16;
            }
        }
    }
}
