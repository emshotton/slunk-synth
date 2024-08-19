use defmt::info;

pub enum MetricName {
    AudioLoopTime,
}

const SAMPLE_SIZE: usize = 128;

pub struct Metrics {
    report_interval_us: u32,
    report_interval_counter: u32,
    audio_loop_time: [u32; SAMPLE_SIZE],
    audio_loop_time_index: usize,
}

impl Metrics {
    pub fn new() -> Self {
        Self {
            audio_loop_time: [0; SAMPLE_SIZE],
            audio_loop_time_index: 0,
            report_interval_us: 1_000_000,
            report_interval_counter: 0,
        }
    }

    pub fn update(&mut self, dt_us: u32) {
        // Report metrics every `report_interval_us`
        self.report_interval_counter += dt_us;
        if self.report_interval_counter >= self.report_interval_us {
            let audio_loop_time_avg: u32 =
                self.audio_loop_time.iter().sum::<u32>() / SAMPLE_SIZE as u32;
            info!(
                ">>>>>>>>>>>>>>>>>>>>>> Audio loop time: {} us",
                audio_loop_time_avg
            );
            self.report_interval_counter = 0;
        }
    }

    pub fn observe(&mut self, metric: MetricName, value: u32) {
        match metric {
            MetricName::AudioLoopTime => {
                self.audio_loop_time[self.audio_loop_time_index] = value;
                self.audio_loop_time_index = (self.audio_loop_time_index + 1) % SAMPLE_SIZE;
            }
        }
    }
}
