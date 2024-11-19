// advent of code website: https://adventofcode.com/2024
// github: https://github.com/mullen25312/aoc2024_rust_nRF52833

#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m::itm::write_str;
use cortex_m_rt::entry;
use rtt_target::{rtt_init_print, rprintln};
use panic_rtt_target as _;

use microbit::{
    board::Board,
    display::blocking::Display,
    hal::prelude::*,
    hal::Timer
};

use utils::Trait;


// daily puzzle day: d00 (advent of code 2021 day 1 as template)
#[entry]
fn main() -> ! {
    rtt_init_print!();

    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);

    rprintln!("########### d00 ###########");
    // display.show_scroll(&mut timer, "abcdef");

    let data:[i32; 10] = [199, 200, 208 ,210 ,200, 207 ,240 ,269 ,260 ,263];

    // part one
    let mut sum1: i32 = 0;
    for idx in 1..data.len() {
        if data[idx] - data[idx - 1] > 0 {
            sum1 += 1;
        }
    }
    
    rprintln!("Result of part one: {}", sum1);
    display.show_scroll(&mut timer, "part 1:");
    display.show_number(&mut timer, sum1);

    
    display.clear();
    timer.delay_ms(1_000_u16);


    // part two
    let mut filtered: [i32; 10] = data.clone();
    for idx in 2..data.len() {
        filtered[idx-2] = data[idx] + data[idx - 1] + data[idx - 2];
    }

    let mut sum2: i32 = 0;
    for idx in 1..filtered.len() {
        if filtered[idx] - filtered[idx - 1] > 0 {
            sum2 += 1;
        }
    }

    rprintln!("Result of part two: {}", sum2);
    display.show_scroll(&mut timer, "part 2:");
    display.show_number(&mut timer, sum2);

    loop {
    }
}
