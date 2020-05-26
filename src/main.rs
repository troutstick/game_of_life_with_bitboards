const LEFT: u64 = 0x8080808080808080;
const RIGHT: u64 = 0x0101010101010101;
const TOP: u64 = 0x00000000000000FF;
const BOTTOM: u64 = 0xFF00000000000000;
const NLEFT: u64 = 0x7F7F7F7F7F7F7F7F; // !LEFT
const NRIGHT: u64 = 0xFEFEFEFEFEFEFEFE; // !RIGHT

struct GameOfLife {
    state: u64,
    bit1: u64,
    bit2: u64,
    bit3: u64,
}

impl GameOfLife {
    fn build_game(starting_state: u64) -> GameOfLife {
        GameOfLife {
            state: starting_state,
            bit1: 0,
            bit2: 0,
            bit3: 0,
        }
    }

    fn step(&mut self) {
        let c10 = self.state >> 8 | ((self.state & TOP) << 56);
        let c12 = self.state << 8 | ((self.state & BOTTOM) >> 56);
        let c00 = (c10 & NLEFT) << 1 | ((c10 & LEFT) >> 7);
        let c01 = (self.state & NLEFT) << 1 | ((self.state & LEFT) >> 7);
        let c02 = (c12 & NLEFT) << 1 | ((c12 & LEFT) >> 7);
        let c20 = (c10 & NRIGHT) >> 1 | ((c10 & RIGHT) << 7);
        let c21 = (self.state & NRIGHT) >> 1 | ((self.state & RIGHT) << 7);
        let c22 = (c12 & NRIGHT) >> 1 | ((c12 & RIGHT) << 7);

        self.bit1 = 0;
        self.bit2 = 0;
        self.bit3 = 0;

        self.add(c00); self.add(c01); self.add(c02); self.add(c10);
        self.add(c12); self.add(c20); self.add(c21); self.add(c22);

        self.state = (self.state | self.bit1) & self.bit2 & !self.bit3;
    }

    fn add(&mut self, cxx: u64) {
        let carry1 = self.bit1 & cxx;
        let carry2 = self.bit2 & carry1;
        self.bit1 ^= cxx;
        self.bit2 ^= carry1;
        self.bit3 ^= carry2;
    }
}

/// Reproduction of an 8x8 Conway's Game of Life in Rust (Browne and Tavener, 2012b).
fn main() {
    let mut game = GameOfLife::build_game(0x0000003808100000);
    print_board(game.state);
    for i in 0..100 {
        game.step();
        print_board(game.state);
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
    let files = "    a  b  c  d  e  f  g  h  ";
    let border = "  +------------------------+";
    println!("{}", border);

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
    println!("{}\n{}", border, files);
}