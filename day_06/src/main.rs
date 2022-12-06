const MARKER_SIZE_P1: usize = 4;
const MARKER_SIZE_P2: usize = 14;

fn main() {
    if let Ok(content) = std::fs::read_to_string("bigboy.txt") {
        let content = content.as_bytes();

        for i in 0..(content.len() - MARKER_SIZE_P1) {
            if charset_unique(&content[i..(i + MARKER_SIZE_P1)]) {
                println!("P1 {}", i + MARKER_SIZE_P1);
                break;
            }
        }

        for i in 0..(content.len() - MARKER_SIZE_P2) {
            if charset_unique(&content[i..(i + MARKER_SIZE_P2)]) {
                println!("P2 {}", i + MARKER_SIZE_P2);
                break;
            }
        }
    }
}

fn charset_unique(marker_candidate: &[u8]) -> bool {
    let mut counts = 0u32;

    for character in marker_candidate {
        counts |= 1 << (*character - 'a' as u8);
    }

    counts.count_ones() == marker_candidate.len() as u32
}

// fn charset_unique(marker_candidate: &[u8]) -> bool {
//     let mut counts = [0u8; 256];

//     for character in marker_candidate {
//         counts[*character as usize] += 1;

//         if counts[*character as usize] > 1 {
//             return false;
//         }
//     }

//     return true;
// }
