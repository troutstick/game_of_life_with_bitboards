const LEFT: u64 = 0x8080808080808080;
const RIGHT: u64 = 0x0101010101010101;
const TOP: u64 = 0x00000000000000FF;
const BOTTOM: u64 = 0xFF00000000000000;
const NLEFT: u64 = 0x7F7F7F7F7F7F7F7F; // ~LEFT
const NRIGHT: u64 = 0xFEFEFEFEFEFEFEFE; // ~RIGHT

/// Reproduction of an 8x8 game of life in Java (Browne and Tavener, 2012b).
fn main() {
    print_board(NLEFT);
}

/// Take a u64 and return a human readable 8x8 grid of its bits.
fn print_board(int: u64) {
    let mut string_repr = format!("{:064b}", int);
    let border = "+-----------------+";
    println!("{}", border);
    for i in (0..64).step_by(8) {
        let mut eight_chars = String::from(&string_repr[i..i+8]);
        for i in (1..8).rev() {
            eight_chars.insert(i, ' ');
        }
        println!("| {} |", eight_chars);
    }
    println!("{}", border);
}