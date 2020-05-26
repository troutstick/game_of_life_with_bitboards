const LEFT: u64 = 0x8080808080808080;
const RIGHT: u64 = 0x0101010101010101;
const TOP: u64 = 0x00000000000000FF;
const BOTTOM: u64 = 0xFF00000000000000;
const NLEFT: u64 = 0x7F7F7F7F7F7F7F7F; // ~LEFT
const NRIGHT: u64 = 0xFEFEFEFEFEFEFEFE; // ~RIGHT

/// Reproduction of an 8x8 game of life in Java (Browne and Tavener, 2012b).
fn main() {
    println!("Hello, world!");
}

/// Take a u64 and return a human readable 8x8 grid of its bits.
fn print_board(int: u64) {

}