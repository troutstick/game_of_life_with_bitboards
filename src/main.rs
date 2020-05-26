extern crate time;
use time::Instant;

const L: u64 = 0x8080808080808080;
const R: u64 = 0x0101010101010101;
const T: u64 = 0x00000000000000FF;
const B: u64 = 0xFF00000000000000;
const NL: u64 = 0x7F7F7F7F7F7F7F7F; // !LEFT
const NR: u64 = 0xFEFEFEFEFEFEFEFE; // !RIGHT

/// Reproduction of an 8x8 Conway's Game of Life in Rust (Browne and Tavener, 2012b).
fn main() {
    benchmark(1_000_000_000);
}

/// Measure the amount of time it takes to run INT steps.
fn benchmark(int: u64) {
    let start = Instant::now();

    let glider_state = 0x0000003808100000;
    let mut game = GameOfLife::build_game(glider_state);
    for _ in 0..int {
        game.step();
    }

    println!("It took {} seconds for the program to run {} steps.", start.elapsed().as_seconds_f64(), int)
}

struct GameOfLife(u64, u64, u64, u64);

impl GameOfLife {
    fn build_game(starting_state: u64) -> GameOfLife {
        GameOfLife(starting_state, 0, 0, 0)
    }

    /// Return the game's current state.
    fn state(&self) -> u64 {
        self.0
    }

    /// Step through the game once.
    fn step(&mut self) {
        let c10 = self.0        >> 8 | ((self.0 & T) << 56);
        let c12 = self.0        << 8 | ((self.0 & B) >> 56);
        let c00 = (c10    & NL) << 1 | ((c10    & L) >> 7);
        let c01 = (self.0 & NL) << 1 | ((self.0 & L) >> 7);
        let c02 = (c12    & NL) << 1 | ((c12    & L) >> 7);
        let c20 = (c10    & NR) >> 1 | ((c10    & R) << 7);
        let c21 = (self.0 & NR) >> 1 | ((self.0 & R) << 7);
        let c22 = (c12    & NR) >> 1 | ((c12    & R) << 7);

        self.1 = 0;
        self.2 = 0;
        self.3 = 0;

        self.add(c00); self.add(c01);
        self.add(c02); self.add(c10);
        self.add(c12); self.add(c20);
        self.add(c21); self.add(c22);

        self.0 = (self.0 | self.1) & self.2 & !self.3;
    }

    fn add(&mut self, cxx: u64) {
        let carry1 = self.1 & cxx;
        let carry2 = self.2 & carry1;
        self.1 ^= cxx;
        self.2 ^= carry1;
        self.3 ^= carry2;
    }
}

/// Take a u64 and return a human readable 8x8 grid of its bits.
/// Follows chess bitboard conventions:
/// bottom-left is represented by leftmost bit,
/// bottom-right by bit 7,
/// and top-right by bit 63.
///
/// Below is the output of 0xFF00000000000000:
///
///   +------------------------+
/// 8 | 0  0  0  0  0  0  0  0 |
/// 7 | 0  0  0  0  0  0  0  0 |
/// 6 | 0  0  0  0  0  0  0  0 |
/// 5 | 0  0  0  0  0  0  0  0 |
/// 4 | 0  0  0  0  0  0  0  0 |
/// 3 | 0  0  0  0  0  0  0  0 |
/// 2 | 0  0  0  0  0  0  0  0 |
/// 1 | 1  1  1  1  1  1  1  1 |
///   +------------------------+
///     a  b  c  d  e  f  g  h
fn print_board(int: u64) {
    let mut string_repr = format!("{:064b}", int);
    println!("{}", BORDER_STR);

    let mut counter = 8;
    for i in (0..64).step_by(8).rev() {
        let mut eight_chars = String::from(&string_repr[i..i+8]);
        for i in (1..8).rev() {
            eight_chars.insert_str(i, "  ");
        }
        eight_chars = eight_chars.replace("0", " ");
        eight_chars = eight_chars.replace("1", "█");
        println!("{} | {} |", counter, eight_chars);
        counter -= 1;
    }
    println!("{}\n{}", BORDER_STR, FILES_STR);
}

const FILES_STR: &str = "    a  b  c  d  e  f  g  h  ";
const BORDER_STR: &str = "  +------------------------+";