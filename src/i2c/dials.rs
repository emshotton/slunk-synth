use crate::i2c::refcelldevice::RefCellDevice;
use core::cell::RefCell;

pub enum Channel {
    A0,
    A1,
    A2,
    A3,
}

pub struct DialChange {
    pub channel: Channel,
    pub value: u16,
}

pub struct Dials<I2C> {
    pub pot_0: u16,
    pub pot_1: u16,
    pub pot_2: u16,
    pub pot_3: u16,
    ads1115: ads1x1x::Ads1x1x<
        I2C,
        ads1x1x::ic::Ads1115,
        ads1x1x::ic::Resolution16Bit,
        ads1x1x::mode::OneShot,
    >,
    channel_index: Channel,
    timer_us: u32,
}

impl<I2C, E> Dials<I2C>
where
    I2C: embedded_hal::i2c::I2c<Error = E>,
{
    pub fn new(i2c_device: I2C, address: ads1x1x::SlaveAddr) -> Self {
        let mut adc = ads1x1x::Ads1x1x::new_ads1115(i2c_device, address);
        adc.set_full_scale_range(ads1x1x::FullScaleRange::Within4_096V)
            .unwrap();
        Dials {
            pot_0: 0,
            pot_1: 0,
            pot_2: 0,
            pot_3: 0,
            ads1115: adc,
            channel_index: Channel::A0,
            timer_us: 0,
        }
    }

    pub fn update(&mut self, dt_us: u32) -> Option<DialChange> {
        self.timer_us += dt_us;
        if self.timer_us < 1000 {
            return None;
        }
        self.timer_us = 0;

        match self.channel_index {
            Channel::A0 => match self.ads1115.read(ads1x1x::channel::SingleA0) {
                Ok(pot_0) => {
                    let pot_0 = i16::max(pot_0, 0) as u16;
                    let changed = self.pot_0 != pot_0;
                    self.pot_0 = pot_0;
                    self.channel_index = Channel::A1;
                    if changed {
                        Some(DialChange {
                            channel: Channel::A0,
                            value: pot_0,
                        })
                    } else {
                        None
                    }
                }
                Err(_) => None,
            },
            Channel::A1 => match self.ads1115.read(ads1x1x::channel::SingleA1) {
                Ok(pot_1) => {
                    let pot_1 = i16::max(pot_1, 0) as u16;
                    let changed = self.pot_1 != pot_1;
                    self.pot_1 = pot_1;
                    self.channel_index = Channel::A2;
                    if changed {
                        Some(DialChange {
                            channel: Channel::A1,
                            value: pot_1,
                        })
                    } else {
                        None
                    }
                }
                Err(_) => None,
            },
            Channel::A2 => match self.ads1115.read(ads1x1x::channel::SingleA2) {
                Ok(pot_2) => {
                    let pot_2 = i16::max(pot_2, 0) as u16;
                    let changed = self.pot_2 != pot_2;
                    self.pot_2 = pot_2;
                    self.channel_index = Channel::A3;
                    if changed {
                        Some(DialChange {
                            channel: Channel::A2,
                            value: pot_2,
                        })
                    } else {
                        None
                    }
                }
                Err(_) => None,
            },
            Channel::A3 => match self.ads1115.read(ads1x1x::channel::SingleA3) {
                Ok(pot_3) => {
                    let pot_3 = i16::max(pot_3, 0) as u16;
                    let changed = self.pot_3 != pot_3;
                    self.pot_3 = pot_3;
                    self.channel_index = Channel::A0;

                    if changed {
                        Some(DialChange {
                            channel: Channel::A3,
                            value: pot_3,
                        })
                    } else {
                        None
                    }
                }
                Err(_) => None,
            },
        }
    }
}
