#![deny(unsafe_code)]
#![no_main]
#![no_std]

use rtt_target::{rtt_init_print, rprintln};

use microbit::{
    display::blocking::Display,
    hal::Timer,
    pac::TIMER0,
};

/// first ASCII symbol available for printing - 48 is 0
const PRINTABLE_START: usize = 48;

/// last ASCII symbol available for printing - 90 is Z
const PRINTABLE_END: usize = 90;

const ASCII_TABLE: [[[u8; 5]; 5]; PRINTABLE_END - PRINTABLE_START + 1]  = [ // table of column-first 5x5 led representation of each character  
    [[0, 1, 1, 1, 0], [1, 0, 0, 0, 1], [1, 0, 0, 0, 1], [1, 0, 0, 0, 1], [0, 1, 1, 1, 0]], // 0
    [[0, 1, 0, 0, 1], [0, 1, 0, 0, 1], [1, 1, 1, 1, 1], [0, 0, 0, 0, 1], [0, 0, 0, 0, 1]], // 1
    [[1, 0, 0, 1, 1], [1, 0, 1, 0, 1], [1, 0, 1, 0, 1], [1, 0, 1, 0, 1], [0, 1, 0, 0, 1]], // 2
    [[1, 0, 0, 0, 1], [1, 0, 1, 0, 1], [1, 0, 1, 0, 1], [1, 0, 1, 0, 1], [0, 1, 0, 1, 0]], // 3
    [[0, 0, 1, 0, 0], [0, 1, 1, 0, 0], [1, 0, 1, 0, 0], [1, 1, 1, 1, 1], [0, 0, 1, 0, 0]], // 4
    [[1, 1, 1, 0, 1], [1, 0, 1, 0, 1], [1, 0, 1, 0, 1], [1, 0, 1, 0, 1], [1, 0, 0, 1, 0]], // 5
    [[0, 1, 1, 1, 0], [1, 0, 1, 0, 1], [1, 0, 1, 0, 1], [1, 0, 1, 0, 1], [0, 0, 0, 1, 0]], // 6
    [[1, 0, 0, 0, 0], [1, 0, 0, 0, 0], [1, 0, 0, 1, 1], [1, 0, 1, 0, 0], [1, 1, 0, 0, 0]], // 7
    [[0, 1, 0, 1, 0], [1, 0, 1, 0, 1], [1, 0, 1, 0, 1], [1, 0, 1, 0, 1], [0, 1, 0, 1, 0]], // 8
    [[0, 1, 0, 0, 0], [1, 0, 1, 0, 1], [1, 0, 1, 0, 1], [1, 0, 1, 0, 1], [0, 1, 1, 1, 0]], // 9
    [[0, 0, 0, 0, 0], [0, 0, 0, 0, 0], [0, 1, 0, 1, 0], [0, 0, 0, 0, 0], [0, 0, 0, 0, 0]], // :
    [[0, 0, 0, 0, 0], [0, 0, 0, 0, 1], [0, 1, 0, 1, 0], [0, 0, 0, 0, 0], [0, 0, 0, 0, 0]], // ;
    [[0, 0, 1, 0, 0], [0, 1, 0, 1, 0], [0, 1, 0, 1, 0], [1, 0, 0, 0, 1], [1, 0, 0, 0, 1]], // <
    [[0, 0, 0, 0, 0], [0, 1, 0, 1, 0], [0, 1, 0, 1, 0], [0, 1, 0, 1, 0], [0, 0, 0, 0, 0]], // =
    [[1, 0, 0, 0, 1], [1, 0, 0, 0, 1], [0, 1, 0, 1, 0], [0, 1, 0, 1, 0], [0, 0, 1, 0, 0]], // >
    [[0, 1, 0, 0, 0], [1, 0, 0, 0, 0], [1, 0, 1, 0, 1], [1, 0, 1, 0, 0], [0, 1, 0, 0, 0]], // ?
    [[1, 1, 1, 1, 1], [1, 0, 0, 0, 1], [1, 0, 1, 0, 1], [1, 0, 1, 0, 1], [1, 1, 1, 0, 1]], // @
    [[0, 1, 1, 1, 1], [1, 0, 0, 1, 0], [1, 0, 0, 1, 0], [1, 0, 0, 1, 0], [0, 1, 1, 1, 1]], // A
    [[1, 1, 1, 1, 1], [1, 0, 1, 0, 1], [1, 0, 1, 0, 1], [1, 0, 1, 0, 1], [1, 1, 0, 1, 1]], // B
    [[1, 1, 1, 1, 1], [1, 0, 0, 0, 1], [1, 0, 0, 0, 1], [1, 0, 0, 0, 1], [1, 0, 0, 0, 1]], // C
    [[1, 1, 1, 1, 1], [1, 0, 0, 0, 1], [1, 0, 0, 0, 1], [1, 0, 0, 0, 1], [0, 1, 1, 1, 0]], // D
    [[1, 1, 1, 1, 1], [1, 0, 1, 0, 1], [1, 0, 1, 0, 1], [1, 0, 1, 0, 1], [1, 0, 0, 0, 1]], // E
    [[1, 1, 1, 1, 1], [1, 0, 1, 0, 0], [1, 0, 1, 0, 0], [1, 0, 0, 0, 0], [1, 0, 0, 0, 0]], // F
    [[0, 1, 1, 1, 1], [1, 0, 0, 0, 1], [1, 0, 0, 0, 1], [1, 0, 1, 0, 1], [1, 0, 1, 1, 1]], // G
    [[1, 1, 1, 1, 1], [0, 0, 1, 0, 0], [0, 0, 1, 0, 0], [0, 0, 1, 0, 0], [1, 1, 1, 1, 1]], // H
    [[1, 0, 0, 0, 1], [1, 0, 0, 0, 1], [1, 1, 1, 1, 1], [1, 0, 0, 0, 1], [1, 0, 0, 0, 1]], // I
    [[0, 0, 0, 1, 1], [0, 0, 0, 0, 1], [0, 0, 0, 0, 1], [1, 0, 0, 0, 1], [1, 1, 1, 1, 1]], // J
    [[1, 1, 1, 1, 1], [0, 0, 1, 0, 0], [0, 0, 1, 0, 0], [0, 1, 0, 1, 0], [1, 0, 0, 0, 1]], // K
    [[1, 1, 1, 1, 1], [0, 0, 0, 0, 1], [0, 0, 0, 0, 1], [0, 0, 0, 0, 1], [0, 0, 0, 0, 1]], // L
    [[1, 1, 1, 1, 1], [0, 1, 0, 0, 0], [0, 0, 1, 0, 0], [0, 1, 0, 0, 0], [1, 1, 1, 1, 1]], // M
    [[1, 1, 1, 1, 1], [0, 1, 0, 0, 0], [0, 0, 1, 0, 0], [0, 0, 0, 1, 0], [1, 1, 1, 1, 1]], // N
    [[0, 1, 1, 1, 0], [1, 0, 0, 0, 1], [1, 0, 0, 0, 1], [1, 0, 0, 0, 1], [0, 1, 1, 1, 0]], // O
    [[1, 1, 1, 1, 1], [1, 0, 1, 0, 0], [1, 0, 1, 0, 0], [1, 0, 1, 0, 0], [0, 1, 0, 0, 0]], // P
    [[1, 1, 1, 1, 0], [1, 0, 0, 1, 0], [1, 0, 0, 1, 1], [1, 0, 0, 1, 0], [1, 1, 1, 1, 0]], // Q
    [[1, 1, 1, 1, 1], [1, 0, 1, 0, 0], [1, 0, 1, 0, 0], [1, 0, 1, 0, 0], [0, 1, 0, 1, 1]], // R
    [[1, 1, 1, 0, 1], [1, 0, 1, 0, 1], [1, 0, 1, 0, 1], [1, 0, 1, 0, 1], [1, 0, 1, 1, 1]], // S
    [[1, 0, 0, 0, 0], [1, 0, 0, 0, 0], [1, 1, 1, 1, 1], [1, 0, 0, 0, 0], [1, 0, 0, 0, 0]], // T
    [[1, 1, 1, 1, 1], [0, 0, 0, 0, 1], [0, 0, 0, 0, 1], [0, 0, 0, 0, 1], [1, 1, 1, 1, 1]], // U
    [[1, 1, 0, 0, 0], [0, 0, 1, 1, 0], [0, 0, 0, 0, 1], [0, 0, 1, 1, 0], [1, 1, 0, 0, 0]], // V
    [[1, 1, 1, 1, 0], [0, 0, 0, 0, 1], [0, 0, 1, 1, 0], [0, 0, 0, 0, 1], [1, 1, 1, 1, 0]], // W
    [[1, 0, 0, 0, 1], [0, 1, 0, 1, 0], [0, 0, 1, 0, 0], [0, 1, 0, 1, 0], [1, 0, 0, 0, 1]], // X
    [[1, 1, 0, 0, 0], [0, 0, 1, 0, 0], [0, 0, 0, 1, 1], [0, 0, 1, 0, 0], [1, 1, 0, 0, 0]], // Y
    [[1, 0, 0, 0, 1], [1, 0, 0, 1, 1], [1, 0, 1, 0, 1], [1, 1, 0, 0, 1], [1, 0, 0, 0, 1]], // Z
];

fn reverse_number(number: i32) -> i32 {
    let mut tmp: i32 = number;
    let mut reversed: i32 = 0;
    while tmp != 0 {
        reversed = reversed * 10 + tmp % 10;
        tmp = tmp / 10;
    }
    return reversed;
}

fn transpose_5x5_u8(led_matrix: [[u8; 5]; 5]) -> [[u8; 5]; 5] {
    let mut tmp_matrix: [[u8; 5]; 5] = led_matrix;
    tmp_matrix[0][1] = led_matrix[1][0]; tmp_matrix[1][0] = led_matrix[0][1];
    tmp_matrix[0][2] = led_matrix[2][0]; tmp_matrix[2][0] = led_matrix[0][2];
    tmp_matrix[0][3] = led_matrix[3][0]; tmp_matrix[3][0] = led_matrix[0][3];
    tmp_matrix[0][4] = led_matrix[4][0]; tmp_matrix[4][0] = led_matrix[0][4];
    tmp_matrix[1][2] = led_matrix[2][1]; tmp_matrix[2][1] = led_matrix[1][2];
    tmp_matrix[1][3] = led_matrix[3][1]; tmp_matrix[3][1] = led_matrix[1][3];
    tmp_matrix[1][4] = led_matrix[4][1]; tmp_matrix[4][1] = led_matrix[1][4];
    tmp_matrix[2][3] = led_matrix[3][2]; tmp_matrix[3][2] = led_matrix[2][3];
    tmp_matrix[2][4] = led_matrix[4][2]; tmp_matrix[4][2] = led_matrix[2][4];
    tmp_matrix[3][4] = led_matrix[4][3]; tmp_matrix[4][3] = led_matrix[3][4];
    return tmp_matrix;
}

fn shift_left_5x5_u8(led_matrix: [[u8; 5]; 5]) -> [[u8; 5]; 5] {
    let mut tmp_matrix: [[u8; 5]; 5] = [[0; 5]; 5];
    tmp_matrix[0] = led_matrix[1];
    tmp_matrix[1] = led_matrix[2];
    tmp_matrix[2] = led_matrix[3];
    tmp_matrix[3] = led_matrix[4];
    tmp_matrix[4] = [0; 5];
    return tmp_matrix;
}

fn char_to_5x5_u8(character: char) -> [[u8; 5]; 5] {
    // map letter to 5x5 led matrix pattern using its ascii code
    if (character.to_ascii_uppercase() as usize >= PRINTABLE_START) && (character.to_ascii_uppercase() as usize <= PRINTABLE_END) {
        return ASCII_TABLE[character.to_ascii_uppercase() as usize - PRINTABLE_START];
    } else { // every other non-available character is mapped to SPACE including SPACE itself
        return [[0; 5]; 5];
    }
}

pub trait ShowExtensions {
    fn show_number(&mut self, timer: &mut Timer<TIMER0>, number: i32);
    fn show_text(&mut self, timer: &mut Timer<TIMER0>, text: &str);
    fn show_scroll(&mut self, timer: &mut Timer<TIMER0>, text: &str);
  }

impl ShowExtensions for Display {
    fn show_number(&mut self, timer: &mut Timer<TIMER0>, number: i32) {
        // display reversed number one digit after the other
        self.show(timer, transpose_5x5_u8(ASCII_TABLE[(reverse_number(number) % 10) as usize]), 1000);
    }

    fn show_text(&mut self, timer: &mut Timer<TIMER0>, text: &str) {
        for letter in text.chars() {
            self.show(timer, transpose_5x5_u8(char_to_5x5_u8(letter)), 1000);
        }
    }

    fn show_scroll(&mut self, timer: &mut Timer<TIMER0>, text: &str) {
        let scroll_ticks: i32 = (text.len() as i32) * 6;

        let mut pic: [[u8; 5]; 5] = [[0; 5]; 5];
        let mut letter_pic: [[u8;5]; 6] = [[0; 5]; 6];

        for scroll_tick in 0..(scroll_ticks+6) {
            self.show(timer, transpose_5x5_u8(pic), 110);

            pic = shift_left_5x5_u8(pic); // shift pic to the left
            if  (scroll_tick % 6) >= 2 { // next letter is needed
                let tmp: [[u8; 5]; 5] = char_to_5x5_u8(text.chars().nth((scroll_tick/6) as usize).unwrap_or(' '));
                letter_pic = [tmp[0], tmp[1], tmp[2], tmp[3], tmp[4], [0; 5]]; // add one empty col for letter separation
            }
            pic[4] = letter_pic[((scroll_tick % 6 + 4) % 6) as usize];
        }
    }
}