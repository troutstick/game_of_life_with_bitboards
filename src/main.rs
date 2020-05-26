const LEFT: u64 = 0x8080808080808080;
const RIGHT: u64 = 0x0101010101010101;
const TOP: u64 = 0x00000000000000FF;
const BOTTOM: u64 = 0xFF00000000000000;
const NLEFT: u64 = 0x7F7F7F7F7F7F7F7F; // ~LEFT
const NRIGHT: u64 = 0xFEFEFEFEFEFEFEFE; // ~RIGHT

/// Reproduction of an 8x8 game of life in Java (Browne and Tavener, 2012b).
fn main() {
    print_board(BOTTOM);
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
    let files = "    a  b  c  d  e  f  g  h  ";
    let border = "  +------------------------+";
    println!("{}", border);

    let mut counter = 8;
    for i in (0..64).step_by(8).rev() {
        let mut eight_chars = String::from(&string_repr[i..i+8]);
        for i in (1..8).rev() {
            eight_chars.insert_str(i, "  ");
        }
        println!("{} | {} |", counter, eight_chars);
        counter -= 1;
    }
    println!("{}\n{}", border, files);
}