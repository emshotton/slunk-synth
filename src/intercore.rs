use defmt::Format;

pub enum Waveform {
    Sine,
    Sawtooth,
    Square,
    Triangle,
}

impl Default for Waveform {
    fn default() -> Self {
        Self::Sine
    }
}

impl Format for Waveform {
    fn format(&self, f: defmt::Formatter) {
        match self {
            Self::Sine => defmt::write!(f, "Sine"),
            Self::Sawtooth => defmt::write!(f, "Sawtooth"),
            Self::Square => defmt::write!(f, "Square"),
            Self::Triangle => defmt::write!(f, "Triangle"),
        }
    }
}

impl Waveform {
    pub fn to_u8(&self) -> u8 {
        match self {
            Self::Sine => 0,
            Self::Sawtooth => 64,
            Self::Square => 128,
            Self::Triangle => 192,
        }
    }

    pub fn from_u8(byte: u8) -> Option<Self> {
        match byte {
            0..64 => Some(Self::Sine),
            64..128 => Some(Self::Sawtooth),
            128..192 => Some(Self::Square),
            192..=u8::MAX => Some(Self::Triangle),
            _ => None,
        }
    }
}

pub enum IntercoreMessage {
    NoteOn { note: u8, velocity: u8 },
    NoteOff { note: u8 },
    AttackControl { attack_ms: u16 },
    DecayControl { decay_ms: u16 },
    SustainControl { sustain_level: u16 },
    ReleaseControl { release_ms: u16 },
    WaveformControl { waveform: Waveform },
    PortamentoControl { portamento_time_ms: u16 },
}

impl IntercoreMessage {
    pub fn from_u32(bytes: u32) -> Option<Self> {
        let bytes = bytes.to_ne_bytes();
        match bytes[0] {
            0x90 => Some(Self::NoteOn {
                note: bytes[1],
                velocity: bytes[2],
            }),
            0x80 => Some(Self::NoteOff { note: bytes[1] }),
            0x01 => Some(Self::AttackControl {
                attack_ms: u16::from_ne_bytes([bytes[1], bytes[2]]),
            }),
            0x02 => Some(Self::DecayControl {
                decay_ms: u16::from_ne_bytes([bytes[1], bytes[2]]),
            }),
            0x03 => Some(Self::SustainControl {
                sustain_level: u16::from_ne_bytes([bytes[1], bytes[2]]),
            }),
            0x04 => Some(Self::ReleaseControl {
                release_ms: u16::from_ne_bytes([bytes[1], bytes[2]]),
            }),
            0x05 => Some(Self::WaveformControl {
                waveform: Waveform::from_u8(bytes[1])?,
            }),
            0x06 => Some(Self::PortamentoControl {
                portamento_time_ms: u16::from_ne_bytes([bytes[1], bytes[2]]),
            }),
            _ => None,
        }
    }

    pub fn to_u32(&self) -> u32 {
        match self {
            Self::NoteOn { note, velocity } => {
                let mut bytes = [0u8; 4];
                bytes[0] = 0x90;
                bytes[1] = *note;
                bytes[2] = *velocity;
                bytes[3] = 0x00;
                u32::from_ne_bytes(bytes)
            }
            Self::NoteOff { note } => {
                let mut bytes = [0u8; 4];
                bytes[0] = 0x80;
                bytes[1] = *note;
                bytes[2] = 0x00;
                bytes[3] = 0x00;
                u32::from_ne_bytes(bytes)
            }
            Self::AttackControl { attack_ms } => {
                let mut bytes = [0u8; 4];
                // 0x01 is the message type for AttackControl
                bytes[0] = 0x01;
                // Split attack_ms into 2 bytes
                let attack_bytes = attack_ms.to_ne_bytes();
                bytes[1] = attack_bytes[0];
                bytes[2] = attack_bytes[1];
                // Padding
                bytes[3] = 0x00;
                u32::from_ne_bytes(bytes)
            }
            Self::DecayControl { decay_ms } => {
                let mut bytes = [0u8; 4];
                bytes[0] = 0x02;
                // Split decay_ms into 2 bytes
                let decay_bytes = decay_ms.to_ne_bytes();
                bytes[1] = decay_bytes[0];
                bytes[2] = decay_bytes[1];
                // Padding
                bytes[3] = 0x00;
                u32::from_ne_bytes(bytes)
            }
            Self::SustainControl { sustain_level } => {
                let mut bytes = [0u8; 4];
                bytes[0] = 0x03;
                // Split sustain_level into 2 bytes
                let sustain_bytes = sustain_level.to_ne_bytes();
                bytes[1] = sustain_bytes[0];
                bytes[2] = sustain_bytes[1];
                // Padding
                bytes[3] = 0x00;
                u32::from_ne_bytes(bytes)
            }
            Self::ReleaseControl { release_ms } => {
                let mut bytes = [0u8; 4];
                bytes[0] = 0x04;
                // Split release_ms into 2 bytes
                let release_bytes = release_ms.to_ne_bytes();
                bytes[1] = release_bytes[0];
                bytes[2] = release_bytes[1];
                // Padding
                bytes[3] = 0x00;
                u32::from_ne_bytes(bytes)
            }
            Self::WaveformControl { waveform } => {
                let mut bytes = [0u8; 4];
                bytes[0] = 0x05;
                bytes[1] = waveform.to_u8();
                bytes[2] = 0x00;
                bytes[3] = 0x00;
                u32::from_ne_bytes(bytes)
            }
            Self::PortamentoControl { portamento_time_ms } => {
                let mut bytes = [0u8; 4];
                bytes[0] = 0x06;
                // Split portamento_time_ms into 2 bytes
                let portamento_bytes = portamento_time_ms.to_ne_bytes();
                bytes[1] = portamento_bytes[0];
                bytes[2] = portamento_bytes[1];
                // Padding
                bytes[3] = 0x00;
                u32::from_ne_bytes(bytes)
            }
        }
    }
}
