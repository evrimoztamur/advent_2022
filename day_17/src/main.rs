use core::num;
use std::collections::HashSet;

const PIECE_A: u16 = 0b0000_0000_0000_1111;
const PIECE_B: u16 = 0b0000_0010_0111_0010;
const PIECE_C: u16 = 0b0000_0100_0100_0111;
const PIECE_D: u16 = 0b0001_0001_0001_0001;
const PIECE_E: u16 = 0b0000_0000_0011_0011;

const PIECES: [u16; 5] = [PIECE_A, PIECE_B, PIECE_C, PIECE_D, PIECE_E];
const PIECE_WIDTHS: [usize; 5] = [4, 3, 3, 1, 2];
const PIECE_HEIGHTS: [usize; 5] = [1, 3, 3, 4, 2];

const LEFT: u8 = '<' as u8;

const TARGET_P1: usize = 1370;

fn main() {
    if let Ok(content) = std::fs::read_to_string("input.txt") {
        let mut seen_combos: HashSet<(usize, usize)> = HashSet::new();
        let content = content.as_bytes();

        let mut num_stopped_rocks: usize = 0;
        let mut highest_point: usize = 0;
        let mut rows: Vec<u8> = Vec::new();
        let mut shifts = 0;

        let mut beginning = 0;
        let mut beginning_i = 0;

        while num_stopped_rocks < TARGET_P1 {
            let index_piece = num_stopped_rocks % PIECES.len();

            let piece = PIECES[index_piece];
            let piece_size = (PIECE_WIDTHS[index_piece], PIECE_HEIGHTS[index_piece]);

            let mut piece_point: (isize, isize) = (2, highest_point as isize + 3);
            let peak_diff = (piece_point.1) + piece_size.1 as isize - rows.len() as isize;
            let peak_diff = peak_diff.max(0) as usize;

            rows.append(&mut vec![0u8; peak_diff]);

            println!("Rock #{} ({})\n=======", num_stopped_rocks, piece_size.1);

            loop {
                let direction = (content[shifts % content.len()] == LEFT) as isize * 2 - 1;

                let next_piece_point = (piece_point.0.saturating_sub(direction), piece_point.1);

                if !try_piece(&rows, piece, next_piece_point, piece_size) {
                    piece_point = next_piece_point;
                }

                shifts += 1;

                let next_piece_point = (piece_point.0, piece_point.1 - 1);

                if !try_piece(&rows, piece, next_piece_point, piece_size) {
                    piece_point = next_piece_point;
                } else {
                    embed_piece(&mut rows, piece, piece_point, piece_size);

                    let piece_point = (piece_point.0 as usize, piece_point.1 as usize);
                    highest_point =
                        highest_point.max(piece_point.1 + piece_size.1 as usize as usize);

                    println!("^ {}\n", highest_point);
                    break;
                }

                if !seen_combos.insert((index_piece, shifts % content.len())) && beginning == 0 {
                    println!("{}", highest_point);
                    draw_board(&rows);
                    beginning = highest_point;
                    beginning_i = num_stopped_rocks;
                }
            }

            for i in 1600..((rows.len() - beginning) / 2) {
                if rows.as_slice()[beginning..(beginning + i)]
                    == rows.as_slice()[(beginning + i)..(beginning + i * 2)]
                {
                    let cycle_duration = (num_stopped_rocks - beginning_i) / 2;
                    let cycles = 1_000_000_000_000 / cycle_duration;
                    println!(
                        "{} @ {} : {} + {} * {} = {}",
                        1_000_000_000_000 % cycle_duration,
                        beginning_i,
                        beginning,
                        cycles,
                        i,
                        beginning + cycles * i
                    );
                    return;
                }
            }

            num_stopped_rocks += 1;
        }
    }
}

fn draw_board(rows: &Vec<u8>) {
    println!(
        "{}",
        rows.iter()
            .rev()
            .map(|v| format!("{:07b}\n", v.reverse_bits() >> 1).replace("0", "."))
            .collect::<String>()
    );
}

fn embed_piece(
    rows: &mut Vec<u8>,
    piece: u16,
    piece_point: (isize, isize),
    piece_size: (usize, usize),
) {
    for i in 0..piece_size.1 {
        let row = ((piece >> (i * 4) & 0b1111) as u8) << piece_point.0;
        let row_point = piece_point.1 as usize + i;
        rows[row_point] |= row;
    }
}

fn try_piece(
    rows: &Vec<u8>,
    piece: u16,
    piece_point: (isize, isize),
    piece_size: (usize, usize),
) -> bool {
    let mut intersection = 0;

    if piece_point.0 < 0 || piece_point.1 < 0 {
        return true;
    }

    let piece_point = (piece_point.0 as isize, piece_point.1 as isize);

    // println!("{:?} {:?} {}", piece_point, piece_size, rows.len());

    if piece_point.0 as usize + piece_size.0 > 7 {
        return true;
    }

    for i in 0..piece_size.1 {
        let row = ((piece >> (i * 4) & 0b1111) as u8) << piece_point.0;
        let row_point = piece_point.1 as usize + i;

        intersection |= rows[row_point] & row;
    }

    intersection != 0
}
