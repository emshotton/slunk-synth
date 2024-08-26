pub const WAVETABLE_SIZE: usize = 128;
pub static SINE_WAVETABLE: [u8; WAVETABLE_SIZE] = [
    127, 133, 139, 146, 152, 158, 164, 170, 176, 182, 187, 193, 198, 203, 208, 213, 217, 221, 226,
    229, 233, 236, 239, 242, 245, 247, 249, 251, 252, 253, 254, 254, 255, 254, 254, 253, 252, 251,
    249, 247, 245, 242, 239, 236, 233, 229, 226, 221, 217, 213, 208, 203, 198, 193, 187, 182, 176,
    170, 164, 158, 152, 146, 139, 133, 127, 121, 115, 108, 102, 96, 90, 84, 78, 72, 67, 61, 56, 51,
    46, 41, 37, 33, 28, 25, 21, 18, 15, 12, 9, 7, 5, 3, 2, 1, 0, 0, 0, 0, 0, 1, 2, 3, 5, 7, 9, 12,
    15, 18, 21, 25, 28, 33, 37, 41, 46, 51, 56, 61, 67, 72, 78, 84, 90, 96, 102, 108, 115, 121,
];
pub static SQUARE_WAVETABLE: [u8; WAVETABLE_SIZE] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255,
];
pub static TRIANGLE_WAVETABLE: [u8; WAVETABLE_SIZE] = [
    0, 3, 7, 11, 15, 19, 23, 27, 31, 35, 39, 43, 47, 51, 55, 59, 63, 67, 71, 75, 79, 83, 87, 91,
    95, 99, 103, 107, 111, 115, 119, 123, 127, 131, 135, 139, 143, 147, 151, 155, 159, 163, 167,
    171, 175, 179, 183, 187, 191, 195, 199, 203, 207, 211, 215, 219, 223, 227, 231, 235, 239, 243,
    247, 251, 255, 251, 247, 243, 239, 235, 231, 227, 223, 219, 215, 211, 207, 203, 199, 195, 191,
    187, 183, 179, 175, 171, 167, 163, 159, 155, 151, 147, 143, 139, 135, 131, 127, 123, 119, 115,
    111, 107, 103, 99, 95, 91, 87, 83, 79, 75, 71, 67, 63, 59, 55, 51, 47, 43, 39, 35, 31, 27, 23,
    19, 15, 11, 7, 3,
];
pub static SAWTOOTH_WAVETABLE: [u8; WAVETABLE_SIZE] = [
    0, 1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25, 27, 29, 31, 33, 35, 37, 39, 41, 43, 45, 47,
    49, 51, 53, 55, 57, 59, 61, 63, 65, 67, 69, 71, 73, 75, 77, 79, 81, 83, 85, 87, 89, 91, 93, 95,
    97, 99, 101, 103, 105, 107, 109, 111, 113, 115, 117, 119, 121, 123, 125, 127, 129, 131, 133,
    135, 137, 139, 141, 143, 145, 147, 149, 151, 153, 155, 157, 159, 161, 163, 165, 167, 169, 171,
    173, 175, 177, 179, 181, 183, 185, 187, 189, 191, 193, 195, 197, 199, 201, 203, 205, 207, 209,
    211, 213, 215, 217, 219, 221, 223, 225, 227, 229, 231, 233, 235, 237, 239, 241, 243, 245, 247,
    249, 251, 253,
];
pub static MIDI_NOTE_TO_SAMPLE_INTERVAL_NS: [u32; 128] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 478011, 451264, 425894, 401768, 379363, 357910, 337952,
    318878, 301023, 284091, 268168, 253100, 238949, 225479, 212857, 200924, 189645, 178987, 168919,
    159439, 150512, 142045, 134084, 126550, 119446, 112740, 106417, 100452, 94805, 89485, 84459,
    79719, 75245, 71023, 67038, 63275, 59726, 56373, 53208, 50223, 47405, 44743, 42230, 39860,
    37624, 35511, 33519, 31638, 29861, 28186, 26604, 25110, 23701, 22371, 21116, 19930, 18812,
    17756, 16759, 15819, 14931, 14093, 13302, 12555, 11850, 11185, 10558, 9965, 9406, 8878, 8380,
    7909, 7465, 7046, 6651, 6278, 5925, 5593, 5279, 4983, 4703, 4439, 4190, 3955, 3733, 3523, 3325,
    3139, 2963, 2796, 2639, 2491, 2351, 2219, 2095, 1977, 1866, 1762, 1663, 1569, 1481, 1398, 1320,
    1246, 1176, 1110, 1047, 989, 0, 0, 0, 0, 0, 0, 0, 0,
];
pub struct WavetablePlayer {
    wavetable: &'static [u8; WAVETABLE_SIZE],
    note: u8,
    sample_interval_ns: u32,
    note_counter_ns: u32,
    wavetable_index: u32,
    portamento_time_ms: u32,
    portamento_target_sample_interval_ns: u32,
    portamento_prev_sample_interval_ns: u32,
    protamento_counter_ns: u32,
}

impl WavetablePlayer {
    pub fn new(wavetable: &'static [u8; WAVETABLE_SIZE], midi_note: u8) -> Self {
        let sample_interval_ns = MIDI_NOTE_TO_SAMPLE_INTERVAL_NS[midi_note as usize];
        Self {
            wavetable,
            note: midi_note,
            sample_interval_ns,
            note_counter_ns: 0,
            wavetable_index: 0,
            portamento_time_ms: 0,
            portamento_target_sample_interval_ns: sample_interval_ns,
            portamento_prev_sample_interval_ns: sample_interval_ns,
            protamento_counter_ns: 0,
        }
    }

    pub fn set_wavetable(&mut self, wavetable: &'static [u8; WAVETABLE_SIZE]) {
        self.wavetable = wavetable;
    }

    pub fn set_portamento(&mut self, glide_time_ms: u32) {
        self.portamento_time_ms = glide_time_ms;
    }

    pub fn set_midi_note(&mut self, midi_note: u8) {
        self.portamento_target_sample_interval_ns =
            MIDI_NOTE_TO_SAMPLE_INTERVAL_NS[midi_note as usize];
        self.portamento_prev_sample_interval_ns = self.sample_interval_ns;
        self.protamento_counter_ns = 0;

        self.note = midi_note;
    }

    pub fn get_midi_note(&self) -> u8 {
        self.note
    }

    pub fn next_sample(&mut self, elapsed_time_us: u32) -> u8 {
        if !(self.sample_interval_ns == self.portamento_target_sample_interval_ns) {
            self.protamento_counter_ns += elapsed_time_us;

            let progress_scaled_1000 =
                (self.protamento_counter_ns / (u32::max(self.portamento_time_ms, 1))) as i32;

            let diff: i32 = self.portamento_target_sample_interval_ns as i32
                - self.portamento_prev_sample_interval_ns as i32;

            let sample_interval_ns = self.portamento_prev_sample_interval_ns as i32
                + (diff * progress_scaled_1000 / 1_000);

            self.sample_interval_ns = sample_interval_ns as u32;

            if progress_scaled_1000 >= 1_000 {
                self.protamento_counter_ns = 0;
                self.portamento_prev_sample_interval_ns = self.portamento_target_sample_interval_ns;
                self.sample_interval_ns = self.portamento_target_sample_interval_ns;
            }
        }

        self.note_counter_ns += elapsed_time_us * 1_000;

        if self.note_counter_ns >= self.sample_interval_ns {
            let wavetable_inc = self.note_counter_ns / self.sample_interval_ns;
            self.wavetable_index += wavetable_inc;
            self.note_counter_ns = self.note_counter_ns - self.sample_interval_ns * wavetable_inc;
        }

        if self.wavetable_index >= self.wavetable.len() as u32 {
            let diff = self.wavetable_index - self.wavetable.len() as u32;
            self.wavetable_index = diff;
        }

        let sample = self.wavetable[self.wavetable_index as usize];

        sample as u8
    }
}
