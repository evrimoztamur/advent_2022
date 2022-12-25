use convert_base::Convert;

fn main() {
    if let Ok(content) = std::fs::read_to_string("input.txt") {
        let mut total = 0;

        for line in content.lines() {
            let mut line_total = 0;

            for (i, ch) in line.chars().rev().enumerate() {
                let dec = snafu_digit_to_dec(ch);
                let num = 5u64.pow(i as u32) as i64 * dec;

                line_total += num;
            }

            total += line_total;
        }

        println!("P1 {}", total);

        println!("{}", (total as f64).ln() / 5.0f64.ln());

        let mut base = Convert::new(10, 5);

        // let total = 2565;

        let total_digits: Vec<_> = total
            .to_string()
            .chars()
            .map(|d| d.to_digit(10).unwrap() as u8)
            .rev()
            .collect();

        println!("{:?}", total_digits);

        let conv = base.convert::<u8, u8>(&total_digits);

        println!("{:?}", conv);

        let mut carry = 0;
        let mut res = Vec::new();

        for i in 0..=(total_digits.len() + 10) {
            let d = *conv.get(i).unwrap_or(&0) as i8;

            if d + carry > 2 {
                res.push(d + carry - 5);
                carry = 1;
            } else {
                res.push(d + carry);
                carry = 0;
            }
        }

        println!("{:?}", res.iter().map(dec_digit_to_snafu).rev().collect::<String>());
    }
}

fn snafu_digit_to_dec(ch: char) -> i64 {
    match ch {
        '2' => 2,
        '1' => 1,
        '0' => 0,
        '-' => -1,
        '=' => -2,
        _ => panic!(),
    }
}


fn dec_digit_to_snafu(d: &i8) -> char {
    match d {
        2 => '2',
        1 => '1',
        0 => '0',
        -1 => '-',
        -2 => '=',
        _ => panic!(),
    }
}
