#![deny(unsafe_code)]
#![no_main]
#![no_std]

use aux5::{entry, prelude::*, Delay, Leds};

#[entry]
fn main() -> ! {
    let (mut delay, mut leds): (Delay, Leds) = aux5::init();

    let half_period = 200_u16;

    loop {

        //      For Forward Direction

        leds[0].on();
        delay.delay_ms(half_period);
        leds[0].off();

        leds[1].on();
        delay.delay_ms(half_period);
        leds[1].off();

        leds[2].on();
        delay.delay_ms(half_period);
        leds[2].off();

        leds[3].on();
        delay.delay_ms(half_period);
        leds[3].off();

        leds[4].on();
        delay.delay_ms(half_period);
        leds[4].off();

        leds[5].on();
        delay.delay_ms(half_period);
        leds[5].off();

        leds[6].on();
        delay.delay_ms(half_period);
        leds[6].off();

        leds[7].on();
        delay.delay_ms(half_period);
        leds[7].off();

        leds[0].on();
        delay.delay_ms(half_period);
        leds[0].off();

        //          For Reverse Direction

        leds[7].on();
        delay.delay_ms(half_period);
        leds[7].off();

        leds[6].on();
        delay.delay_ms(half_period);
        leds[6].off();

        leds[5].on();
        delay.delay_ms(half_period);
        leds[5].off();

        leds[4].on();
        delay.delay_ms(half_period);
        leds[4].off();

        leds[3].on();
        delay.delay_ms(half_period);
        leds[3].off();

        leds[2].on();
        delay.delay_ms(half_period);
        leds[2].off();

        leds[1].on();
        delay.delay_ms(half_period);
        leds[1].off();

        leds[0].on();
        delay.delay_ms(half_period);
        leds[0].off();
                
    }
}