//! Blinks the LED on a Pico board
//!
//! This will blink an LED attached to GP25, which is the pin the Pico uses for the on-board LED.
#![no_std]
#![no_main]

mod adsr;
mod i2c;
mod intercore;
mod metrics;
mod synth;
mod wavetables;

use crate::i2c::dials::Dials;
use crate::i2c::refcelldevice::RefCellDevice;
use crate::metrics::{MetricName, Metrics};
use ads1x1x;
use bsp::entry;
use core::cell::RefCell;
use defmt::*;
use defmt_rtt as _;
use embedded_hal::pwm::SetDutyCycle;
use intercore::IntercoreMessage;
use panic_probe as _;
use rp_pico::hal::Clock;
use usb_device::class_prelude::UsbBusAllocator;
use usb_device::prelude::{StringDescriptors, UsbDeviceBuilder, UsbVidPid};
use usbd_midi::data::midi::channel::Channel::Channel1;
use usbd_midi::data::midi::message::Message;
use usbd_midi::data::usb_midi::midi_packet_reader::MidiPacketBufferReader;
use usbd_midi::midi_device::MidiClass;

// Provide an alias for our BSP so we can switch targets quickly.
// Uncomment the BSP you included in Cargo.toml, the rest of the code does not need to change.
use rp_pico::{self as bsp};
// use sparkfun_pro_micro_rp2040 as bsp;

use bsp::hal::{
    clocks::init_clocks_and_plls,
    fugit::RateExtU32,
    gpio::{FunctionI2C, Pin, PullUp},
    multicore::{Multicore, Stack},
    pac,
    sio::Sio,
    timer::Timer,
    usb::UsbBus,
    watchdog::Watchdog,
    I2C,
};

use crate::synth::Synth;

const XTAL_FREQ_HZ: u32 = 12_000_000u32;

static mut CORE1_STACK: Stack<4096> = Stack::new();
// static mut CORE1_STACK: Stack<8192> = Stack::new();

fn core1_task(loop_timer: &bsp::hal::timer::Timer) -> ! {
    let mut pac = unsafe { pac::Peripherals::steal() };
    // let _core = unsafe { pac::CorePeripherals::steal() };
    // let mut _delay = cortex_m::delay::Delay::new(_core.SYST, sys_freq);

    let mut sio = Sio::new(pac.SIO);
    let pins = bsp::hal::gpio::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // Init PWMs
    let mut pwm_slices = bsp::hal::pwm::Slices::new(pac.PWM, &mut pac.RESETS);

    // Configure PWM4
    let pwm = &mut pwm_slices.pwm4;
    pwm.set_ph_correct();
    pwm.set_top(256);
    pwm.enable();

    // Output channel B on PWM4 to GPIO 25
    let channel = &mut pwm.channel_b;
    channel.output_to(pins.gpio25);

    // Setup the polyphonic synthesizer
    let mut poly_synth = synth::PolySynth::new();
    let mut previous_time_us: u32 = loop_timer.get_counter_low();

    let mut metrics = Metrics::new();

    loop {
        let current_time_us = loop_timer.get_counter_low();
        let elapsed_time_us = current_time_us.wrapping_sub(previous_time_us);
        previous_time_us = current_time_us;

        metrics.observe(MetricName::AudioLoopTime, elapsed_time_us);
        // Report metrics every seconds
        metrics.update(elapsed_time_us);

        let sample = poly_synth.update(elapsed_time_us);

        channel.set_duty_cycle(sample as u16).unwrap();

        // Check for messages from the other core
        let msg = sio.fifo.read();
        if let Some(word) = msg {
            let message = IntercoreMessage::from_u32(word);
            match message {
                Some(IntercoreMessage::NoteOn { note, velocity }) => {
                    info!("NoteOn: note: {}, velocity: {}", note, velocity);
                    poly_synth.note_on(note, velocity);
                }
                Some(IntercoreMessage::NoteOff { note }) => {
                    info!("NoteOff: note: {}", note);
                    poly_synth.note_off(note);
                }
                Some(IntercoreMessage::AttackControl { attack_ms }) => {
                    info!("AttackControl: attack_ms: {}", attack_ms);
                    poly_synth.attack_control(attack_ms);
                }
                Some(IntercoreMessage::DecayControl { decay_ms }) => {
                    info!("DecayControl: decay_ms: {}", decay_ms);
                    poly_synth.decay_control(decay_ms);
                }
                Some(IntercoreMessage::SustainControl { sustain_level }) => {
                    info!("SustainControl: sustain_level: {}", sustain_level);
                    poly_synth.sustain_control(core::cmp::min(
                        sustain_level as u16,
                        adsr::MAX_LEVEL as u16,
                    ));
                }
                Some(IntercoreMessage::ReleaseControl { release_ms }) => {
                    info!("ReleaseControl: release_ms: {}", release_ms);
                    poly_synth.release_control(release_ms);
                }
                Some(IntercoreMessage::WaveformControl { waveform }) => {
                    let wavetable = match waveform {
                        intercore::Waveform::Sine => &wavetables::SINE_WAVETABLE,
                        intercore::Waveform::Square => &wavetables::SQUARE_WAVETABLE,
                        intercore::Waveform::Triangle => &wavetables::TRIANGLE_WAVETABLE,
                        intercore::Waveform::Sawtooth => &wavetables::SAWTOOTH_WAVETABLE,
                    };
                    info!("WaveformControl: waveform: {:?}", waveform);
                    poly_synth.set_wavetable(&wavetable);
                }
                Some(IntercoreMessage::PortamentoControl { portamento_time_ms }) => {
                    info!(
                        "PortamentoControl: portamento_time_ms: {}",
                        portamento_time_ms
                    );
                    poly_synth.portamento_control(portamento_time_ms);
                }
                None => {
                    info!("Unknown message: {}", word);
                }
            }
        }
    }
}

#[entry]
fn main() -> ! {
    info!("Program start");
    let mut pac = pac::Peripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);

    // External high-speed crystal on the pico board is 12Mhz
    let clocks = init_clocks_and_plls(
        XTAL_FREQ_HZ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .unwrap();

    // The single-cycle I/O block controls our GPIO pins
    let mut sio = bsp::hal::sio::Sio::new(pac.SIO);
    let mut mc = Multicore::new(&mut pac.PSM, &mut pac.PPB, &mut sio.fifo);
    let cores = mc.cores();
    let core1 = &mut cores[1];

    // Setup the GPIO pins
    let pins = bsp::hal::gpio::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // Setup the timer
    let loop_timer = Timer::new(pac.TIMER, &mut pac.RESETS, &clocks);

    info!("Starting Core");

    core1
        .spawn(unsafe { &mut CORE1_STACK.mem }, move || {
            core1_task(&loop_timer);
        })
        .unwrap();

    // Setup the USB device
    info!("Creating USB device");
    let usb_bus = UsbBusAllocator::new(UsbBus::new(
        pac.USBCTRL_REGS,
        pac.USBCTRL_DPRAM,
        clocks.usb_clock,
        true,
        &mut pac.RESETS,
    ));

    // Configure two pins as being I²C, not GPIO
    let sda_pin: Pin<_, FunctionI2C, PullUp> = pins.gpio16.reconfigure();
    let scl_pin: Pin<_, FunctionI2C, PullUp> = pins.gpio17.reconfigure();

    info!("Creating I2C controller");
    let i2c = I2C::new_controller(
        pac.I2C0,
        sda_pin,
        scl_pin,
        400.kHz(),
        &mut pac.RESETS,
        clocks.system_clock.freq(),
    );

    // Share the i2c bus between the two PCA9685 devices
    let i2c_ref_cell = RefCell::new(i2c);
    let mut i2c_device_adsr = RefCellDevice::new(&i2c_ref_cell);
    // Create a MIDI class with 1 input and 0 output jacks.
    let mut midi = MidiClass::new(&usb_bus, 1, 1).unwrap();

    let mut usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x16c0, 0x5e4))
        .strings(&[StringDescriptors::default()
            .manufacturer("b0rk")
            .product("midi-string-instrument")
            .serial_number("0")])
        .unwrap()
        .device_class(0)
        .device_sub_class(0)
        .build();

    info!("Entering main loop");

    let mut adsr_dials = Dials::new(&i2c_device_adsr, ads1x1x::SlaveAddr::default());
    let mut dials = Dials::new(&i2c_ref_cell, ads1x1x::SlaveAddr::Sda);
    let mut previous_time_us = loop_timer.get_counter_low();
    loop {
        let current_time_us = loop_timer.get_counter_low();
        let elapsed_time_us = current_time_us.wrapping_sub(previous_time_us);
        previous_time_us = current_time_us;

        adsr_dials.update(elapsed_time_us).and_then(|dial_change| {
            match dial_change.channel {
                i2c::dials::Channel::A0 => {
                    let msg = IntercoreMessage::AttackControl {
                        attack_ms: dial_change.value as u16 >> 5,
                    };
                    sio.fifo.write(msg.to_u32());
                }
                i2c::dials::Channel::A1 => {
                    let msg = IntercoreMessage::DecayControl {
                        decay_ms: dial_change.value as u16 >> 5,
                    };
                    sio.fifo.write(msg.to_u32());
                }
                i2c::dials::Channel::A2 => {
                    let msg = IntercoreMessage::SustainControl {
                        sustain_level: dial_change.value as u16 >> 3,
                    };
                    sio.fifo.write(msg.to_u32());
                }
                i2c::dials::Channel::A3 => {
                    let msg = IntercoreMessage::ReleaseControl {
                        release_ms: dial_change.value as u16 >> 5,
                    };
                    sio.fifo.write(msg.to_u32());
                }
            }
            Some(())
        });

        if !usb_dev.poll(&mut [&mut midi]) {
            continue;
        }
        let mut buffer = [0; 64];

        if let Ok(size) = midi.read(&mut buffer) {
            let buffer_reader = MidiPacketBufferReader::new(&buffer, size);

            for packet in buffer_reader.into_iter() {
                if let Ok(packet) = packet {
                    match packet.message {
                        Message::NoteOn(Channel1, note, velocity) => {
                            let velocity = u8::from(velocity);
                            let note: u8 = note.into();
                            let msg = IntercoreMessage::NoteOn { note, velocity };
                            sio.fifo.write(msg.to_u32());
                        }
                        Message::NoteOff(Channel1, note, ..) => {
                            let note: u8 = note.into();
                            let msg = IntercoreMessage::NoteOff { note };
                            sio.fifo.write(msg.to_u32());
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}

// End of file
