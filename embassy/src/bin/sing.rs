// +---------------------------------------------------------------------------+
// |                            Embassy March 2025                             |
// |                            Sing your own tune                             |
// +---------------------------------------------------------------------------+

#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_mar_2025::music::{Note, OCTAVE};
use embassy_rp::pwm::{self, Pwm};
use embassy_time::{Duration, Timer};
use fixed::traits::ToFixed;
// Use the `panic_probe` crate to provided the panic handler and the
// `defmt_rtt` to import the runtime for defmt, to be able to use
// the print macros.
use {defmt_rtt as _, panic_probe as _};
// Use the logging macros provided by defmt.
#[allow(unused)]
use defmt::*;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    // Get a handle to the RP's peripherals.
    let p = embassy_rp::init(Default::default());


    /// Beats per minute.
    const TEMPO: u64 = 100;
    /// A whole note duration in milliseconds.
    const WHOLE_NOTE: u64 = 4 * (60_000 / TEMPO);
    /// The microcontroller clock frequency
    const CLOCK_FREQ: u64 = 150_000_000;
    /// PWM clock divider
    const PWM_DIV: u64 = 64;


    // TODO: Configure the PWM pin.
    let mut buzzer_cfg = pwm::Config::default();
    buzzer_cfg.divider = PWM_DIV.to_fixed();
    let mut buzzer =  Pwm::new_output_a(p.PWM_SLICE1, p.PIN_2, buzzer_cfg.clone());



    for (note, length) in OCTAVE {
        // TODO: Compute the note's duration based on
        // the length variable.
        let duration: u64 = (length as u64) * WHOLE_NOTE / 4;
        
        
        match note {
            Some(note) => {
                // TODO: Configure the `top` and `compare_X` registers
                // based on the note's type and change the PWM's config.
                // Keep in mind that we are aiming for a 50% duty cycle.
                // "Play" the note for 90% of the duration, then insert
                // a 10% pause before playing the next note.
                // Calculate PWM period from the note's frequency
                
                
                let note_frequency = note as u64;
                let top = CLOCK_FREQ / (PWM_DIV * note_frequency) - 1;
               
               // Set PWM config
                buzzer_cfg.top = top as u16;
                buzzer_cfg.compare_a = (top as u16) / 2;
                buzzer.set_config(&buzzer_cfg);
                
                // Await 90% duration
                Timer::after(Duration::from_millis(duration * 9 / 10)).await;
                
                // Turn buzzer off
                buzzer_cfg.compare_a = 0;
                buzzer.set_config(&buzzer_cfg);

                // Await 10% duration
                Timer::after(Duration::from_millis(duration / 10)).await;
            },
            None => {
                // TODO: Just wait the whole duration.
                Timer::after(Duration::from_millis(duration)).await;
            }
        };
    }
}
