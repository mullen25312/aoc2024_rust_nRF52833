#![deny(unsafe_code)]
#![no_main]
#![no_std]

use rtt_target::{rtt_init_print, rprintln};

use microbit::{
    display::blocking::Display,
    hal::Timer,
    pac::TIMER0,
};

/// first ASCII symbol available for printing - 48 is  --> 47 is remapped space
const PRINTABLE_START: usize = 47;

/// last ASCII symbol available for printing - 90 is Z
const PRINTABLE_END: usize = 90;

const NUMBERS: [[[u8; 5]; 5]; 10] = [
    [[0, 1, 1, 1, 0], [1, 0, 0, 0, 1], [1, 0, 0, 0, 1], [1, 0, 0, 0, 1], [0, 1, 1, 1, 0]], // zero
    [[0, 0, 1, 0, 0], [1, 1, 1, 0, 0], [0, 0, 1, 0, 0], [0, 0, 1, 0, 0], [1, 1, 1, 1, 1]], // one
    [[1, 1, 1, 1, 0], [0, 0, 0, 0, 1], [0, 1, 1, 1, 0], [1, 0, 0, 0, 0], [1, 1, 1, 1, 1]], // two
    [[1, 1, 1, 1, 0], [0, 0, 0, 0, 1], [0, 1, 1, 1, 0], [0, 0, 0, 0, 1], [1, 1, 1, 1, 0]], // three
    [[0, 0, 1, 1, 0], [0, 1, 0, 1, 0], [1, 1, 1, 1, 1], [0, 0, 0, 1, 0], [0, 0, 0, 1, 0]], // four
    [[1, 1, 1, 1, 1], [1, 0, 0, 0, 0], [1, 1, 1, 1, 0], [0, 0, 0, 0, 1], [1, 1, 1, 1, 0]], // five
    [[0, 1, 1, 1, 0], [1, 0, 0, 0, 0], [1, 1, 1, 1, 0], [1, 0, 0, 0, 1], [0, 1, 1, 1, 0]], // six
    [[1, 1, 1, 1, 1], [0, 0, 0, 0, 1], [0, 0, 0, 1, 0], [0, 0, 1, 0, 0], [0, 0, 1, 0, 0]], // seven
    [[0, 1, 1, 1, 0], [1, 0, 0, 0, 1], [0, 1, 1, 1, 0], [1, 0, 0, 0, 1], [0, 1, 1, 1, 0]], // eight
    [[0, 1, 1, 1, 0], [1, 0, 0, 0, 1], [0, 1, 1, 1, 1], [0, 0, 0, 0, 1], [0, 1, 1, 1, 0]], // nine
];

const ACSII_TABLE: [[[u8; 5]; 5]; PRINTABLE_END - PRINTABLE_START + 1]  = [
    [[0, 0, 0, 0, 0], [0, 0, 0, 0, 0], [0, 0, 0, 0, 0], [0, 0, 0, 0, 0], [0, 0, 0, 0, 0]], // SPACE
    [[0, 1, 1, 1, 0], [1, 0, 0, 0, 1], [1, 0, 0, 0, 1], [1, 0, 0, 0, 1], [0, 1, 1, 1, 0]], // 0
    [[0, 0, 1, 0, 0], [1, 1, 1, 0, 0], [0, 0, 1, 0, 0], [0, 0, 1, 0, 0], [1, 1, 1, 1, 1]], // 1
    [[1, 1, 1, 1, 0], [0, 0, 0, 0, 1], [0, 1, 1, 1, 0], [1, 0, 0, 0, 0], [1, 1, 1, 1, 1]], // 2
    [[1, 1, 1, 1, 0], [0, 0, 0, 0, 1], [0, 1, 1, 1, 0], [0, 0, 0, 0, 1], [1, 1, 1, 1, 0]], // 3
    [[0, 0, 1, 1, 0], [0, 1, 0, 1, 0], [1, 1, 1, 1, 1], [0, 0, 0, 1, 0], [0, 0, 0, 1, 0]], // 4
    [[1, 1, 1, 1, 1], [1, 0, 0, 0, 0], [1, 1, 1, 1, 0], [0, 0, 0, 0, 1], [1, 1, 1, 1, 0]], // 5
    [[0, 1, 1, 1, 0], [1, 0, 0, 0, 0], [1, 1, 1, 1, 0], [1, 0, 0, 0, 1], [0, 1, 1, 1, 0]], // 6
    [[1, 1, 1, 1, 1], [0, 0, 0, 0, 1], [0, 0, 0, 1, 0], [0, 0, 1, 0, 0], [0, 0, 1, 0, 0]], // 7
    [[0, 1, 1, 1, 0], [1, 0, 0, 0, 1], [0, 1, 1, 1, 0], [1, 0, 0, 0, 1], [0, 1, 1, 1, 0]], // 8
    [[0, 1, 1, 1, 0], [1, 0, 0, 0, 1], [0, 1, 1, 1, 1], [0, 0, 0, 0, 1], [0, 1, 1, 1, 0]], // 9
    [[0, 0, 0, 0, 0], [0, 0, 1, 0, 0], [0, 0, 0, 0, 0], [0, 0, 1, 0, 0], [0, 0, 0, 0, 0]], // :
    [[0, 0, 0, 0, 0], [0, 0, 1, 0, 0], [0, 0, 0, 0, 0], [0, 0, 1, 0, 0], [0, 1, 0, 0, 0]], // ;
    [[0, 0, 0, 0, 0], [0, 0, 0, 0, 0], [0, 0, 0, 0, 0], [0, 0, 0, 0, 0], [0, 0, 0, 0, 0]], // < (todo)
    [[0, 0, 0, 0, 0], [0, 1, 1, 1, 0], [0, 0, 0, 0, 0], [0, 1, 1, 1, 0], [0, 0, 0, 0, 0]], // =
    [[0, 0, 0, 0, 0], [0, 0, 0, 0, 0], [0, 0, 0, 0, 0], [0, 0, 0, 0, 0], [0, 0, 0, 0, 0]], // > (todo)
    [[0, 0, 0, 0, 0], [0, 0, 0, 0, 0], [0, 0, 0, 0, 0], [0, 0, 0, 0, 0], [0, 0, 0, 0, 0]], // ? (todo)
    [[0, 0, 0, 0, 0], [0, 0, 0, 0, 0], [0, 0, 0, 0, 0], [0, 0, 0, 0, 0], [0, 0, 0, 0, 0]], // @ (todo)
    [[1, 1, 1, 1, 1], [1, 0, 0, 0, 1], [1, 0, 0, 0, 1], [1, 1, 1, 1, 1], [1, 0, 0, 0, 1]], // A
    [[1, 1, 1, 1, 1], [1, 0, 0, 0, 1], [1, 1, 1, 1, 0], [1, 0, 0, 0, 1], [1, 1, 1, 1, 1]], // B
    [[1, 1, 1, 1, 1], [1, 0, 0, 0, 0], [1, 0, 0, 0, 0], [1, 0, 0, 0, 0], [1, 1, 1, 1, 1]], // C
    [[1, 1, 1, 1, 0], [1, 0, 0, 0, 1], [1, 0, 0, 0, 1], [1, 0, 0, 0, 1], [1, 1, 1, 1, 0]], // D
    [[1, 1, 1, 1, 1], [1, 0, 0, 0, 0], [1, 1, 1, 1, 0], [1, 0, 0, 0, 0], [1, 1, 1, 1, 1]], // E
    [[1, 1, 1, 1, 1], [1, 0, 0, 0, 0], [1, 1, 1, 0, 0], [1, 0, 0, 0, 0], [1, 0, 0, 0, 0]], // F
    [[1, 1, 1, 1, 1], [1, 0, 0, 0, 0], [1, 0, 0, 1, 1], [1, 0, 0, 0, 1], [1, 1, 1, 1, 1]], // G
    [[1, 0, 0, 0, 1], [1, 0, 0, 0, 1], [1, 1, 1, 1, 1], [1, 0, 0, 0, 1], [1, 0, 0, 0, 1]], // H
    [[1, 1, 1, 1, 1], [0, 0, 1, 0, 0], [0, 0, 1, 0, 0], [0, 0, 1, 0, 0], [1, 1, 1, 1, 1]], // I
    [[0, 0, 0, 1, 1], [0, 0, 0, 0, 1], [0, 0, 0, 0, 1], [1, 0, 0, 0, 1], [1, 1, 1, 1, 1]], // J
    [[1, 0, 0, 0, 1], [1, 0, 0, 1, 0], [1, 1, 1, 0, 0], [1, 0, 0, 1, 0], [1, 0, 0, 0, 1]], // K
    [[1, 0, 0, 0, 0], [1, 0, 0, 0, 0], [1, 0, 0, 0, 0], [1, 0, 0, 0, 0], [1, 1, 1, 1, 1]], // L
    [[1, 0, 0, 0, 1], [1, 1, 0, 1, 1], [1, 0, 1, 0, 1], [1, 0, 0, 0, 1], [1, 0, 0, 0, 1]], // M
    [[1, 0, 0, 0, 1], [1, 1, 0, 0, 1], [1, 0, 1, 0, 1], [1, 0, 0, 1, 1], [1, 0, 0, 0, 1]], // N
    [[0, 1, 1, 1, 0], [1, 0, 0, 0, 1], [1, 0, 0, 0, 1], [1, 0, 0, 0, 1], [0, 1, 1, 1, 0]], // O
    [[1, 1, 1, 1, 0], [1, 0, 0, 0, 1], [1, 1, 1, 1, 0], [1, 0, 0, 0, 0], [1, 0, 0, 0, 0]], // P
    [[1, 1, 1, 1, 1], [1, 0, 0, 0, 1], [1, 0, 0, 0, 1], [1, 1, 1, 1, 1], [0, 0, 1, 0, 0]], // Q
    [[1, 1, 1, 1, 0], [1, 0, 0, 0, 1], [1, 1, 1, 1, 0], [1, 0, 0, 0, 1], [1, 0, 0, 0, 1]], // R
    [[1, 1, 1, 1, 1], [1, 0, 0, 0, 0], [1, 1, 1, 1, 1], [0, 0, 0, 0, 1], [1, 1, 1, 1, 1]], // S
    [[1, 1, 1, 1, 1], [0, 0, 1, 0, 0], [0, 0, 1, 0, 0], [0, 0, 1, 0, 0], [0, 0, 1, 0, 0]], // T
    [[1, 0, 0, 0, 1], [1, 0, 0, 0, 1], [1, 0, 0, 0, 1], [1, 0, 0, 0, 1], [1, 1, 1, 1, 1]], // U
    [[1, 0, 0, 0, 1], [1, 0, 0, 0, 1], [0, 1, 0, 1, 0], [0, 1, 0, 1, 0], [0, 0, 1, 0, 0]], // V
    [[1, 0, 0, 0, 1], [1, 0, 0, 0, 1], [1, 0, 1, 0, 1], [1, 0, 1, 0, 1], [0, 1, 0, 1, 0]], // W
    [[1, 0, 0, 0, 1], [0, 1, 0, 1, 0], [0, 0, 1, 0, 0], [0, 1, 0, 1, 0], [1, 0, 0, 0, 1]], // X
    [[1, 0, 0, 0, 1], [1, 0, 0, 0, 1], [0, 1, 0, 1, 0], [0, 0, 1, 0, 0], [0, 0, 1, 0, 0]], // Y
    [[1, 1, 1, 1, 1], [0, 0, 0, 1, 0], [0, 0, 1, 0, 0], [0, 1, 0, 0, 0], [1, 1, 1, 1, 1]], // Z
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

pub trait Trait {
    fn show_number(&mut self, timer: &mut Timer<TIMER0>, number: i32);
    fn show_text(&mut self, timer: &mut Timer<TIMER0>, text: &str);
    fn show_scroll(&mut self, timer: &mut Timer<TIMER0>, text: &str);
  }

impl Trait for Display {
    fn show_number(&mut self, timer: &mut Timer<TIMER0>, number: i32) {
        // display reversed number one digit after the other
        self.show(timer, NUMBERS[(reverse_number(number) % 10) as usize], 1000);
    }

    fn show_text(&mut self, timer: &mut Timer<TIMER0>, text: &str) {
        for letter in text.chars() {
            if (letter.to_ascii_uppercase() as usize >= PRINTABLE_START) && (letter.to_ascii_uppercase() as usize <= PRINTABLE_END) {
                self.show(timer, ACSII_TABLE[letter.to_ascii_uppercase() as usize - PRINTABLE_START], 1000);
            } else {
                self.show(timer, [[0; 5]; 5], 1000);
            }
        }
    }

    fn show_scroll(&mut self, timer: &mut Timer<TIMER0>, text: &str) {
        let scroll_ticks: i32 = (text.len() as i32) * 6;
        let mut letter = '\0';

        let mut pic: [[u8; 5]; 5] = [[0; 5]; 5];
        let mut tmp = [[0; 5]; 5];

        let mut letter_pic: [[u8;5]; 6] = [[0; 5]; 6];
    
        for scroll_tick in 2..(scroll_ticks-2) {
            pic = [[0; 5]; 5];
            for row in -2..3 {
                // letter = text.chars().nth((scroll_tick/5) as usize).unwrap();
                if (scroll_tick % 6 + row) < 0 {
                    letter = text.chars().nth((scroll_tick/6 - 1) as usize).unwrap();
                } else if (scroll_tick % 6 + row) >= 0 && (scroll_tick % 6 + row) < 6 {
                    letter = text.chars().nth((scroll_tick/6 + 0) as usize).unwrap();
                } else if  (scroll_tick % 6 + row) >= 6 {
                    letter = text.chars().nth((scroll_tick/6 + 1) as usize).unwrap();
                }
                letter = letter.to_ascii_uppercase();
                if letter == ' ' {letter = '/'};
                tmp = transpose_5x5_u8(ACSII_TABLE[letter as usize - PRINTABLE_START]);
                letter_pic = [tmp[0], tmp[1], tmp[2], tmp[3], tmp[4], [0; 5]];
                pic[(row+2) as usize] = letter_pic[(((scroll_tick % 6 + row) + 6) % 6) as usize];
            }
            self.show(timer, transpose_5x5_u8(pic), 125);
        }
    }
}