fn main() {
    let input = include_str!("inputs/1.txt");
    let mut numbers: Vec<i32> = Vec::new();
    let mut digits: Vec<char> = Vec::new();
    let mut counts: [i8; 10] = [0,0,0,0,0,0,0,0,0,0];
    for c in input.chars() {
        match c {
            '0'..='9' => {
                digits.push(c);
                for i in 0..=9 { counts[i] = 0; }
            },
            'e' => {
                counts[0] = if counts[0] == 1 { counts[0] + 1 } else {0}; // zero
                counts[1] = if counts[1] == 2 { counts[1] + 1 } else {0}; // one
                counts[2] = 0; // two
                counts[3] = if counts[3] == 3 || counts[3] == 4 { counts[3] + 1 } else {0}; // three
                counts[4] = 0; // four
                counts[5] = if counts[5] == 3 { counts[5] + 1} else {0}; // five
                counts[6] = 0; // six
                counts[7] = if counts[7] == 1 || counts[7] == 3 { counts[7] + 1} else {0}; // seven
                counts[8] = 1; // eight
                counts[9] = if counts[9] == 3 { counts[9] + 1} else {0}; // nine
            },
            'f' => {
                counts[0] = 0;
                counts[1] = 0;
                counts[2] = 0; // two
                counts[3] = 0;
                counts[4] = 1; // four
                counts[5] = 1; // five
                counts[6] = 0; // six
                counts[7] = 0;
                counts[8] = 0;
                counts[9] = 0;
            },
            'g' => {
                counts[0] = 0;
                counts[1] = 0;
                counts[2] = 0; // two
                counts[3] = 0;
                counts[4] = 0; // four
                counts[5] = 0;
                counts[6] = 0; // six
                counts[7] = 0;
                counts[8] = if counts[8] == 2 {counts[8]+1} else {0}; // eight
                counts[9] = 0;
            },
            'h' => {
                counts[0] = 0;
                counts[1] = 0;
                counts[2] = 0; // two
                counts[3] = if counts[3] == 1 { counts[3] + 1 } else {0}; // three
                counts[4] = 0; // four
                counts[5] = 0;
                counts[6] = 0; // six
                counts[7] = 0;
                counts[8] = if counts[8] == 3 { counts[8] + 1} else {0}; // eight
                counts[9] = 0;
            },
            'i' => {
                counts[0] = 0;
                counts[1] = 0;
                counts[2] = 0; // two
                counts[3] = 0;
                counts[4] = 0; // four
                counts[5] = if counts[5] == 1 { counts[5] + 1} else {0}; // five
                counts[6] = if counts[6] == 1 { counts[6] + 1} else {0}; // six
                counts[7] = 0;
                counts[8] = if counts[8] == 1 { counts[8] + 1} else {0}; // eight
                counts[9] = if counts[9] == 1 { counts[9] + 1} else {0}; // nine
            },
            'n' => {
                counts[0] = 0; // zero
                counts[1] = if counts[1] == 1 { counts[1] + 1 } else {0}; // one
                counts[2] = 0; // two
                counts[3] = 0;
                counts[4] = 0; // four
                counts[5] = 0;
                counts[6] = 0; // six
                counts[7] = if counts[7] == 4 { counts[7] + 1} else {0}; // seven
                counts[8] = 0;
                counts[9] = if counts[9] == 2 { counts[9] +1 } else {1};
            },
            'o' => {
                counts[0] = if counts[0] == 3 { counts[0] + 1 } else {0}; // zero
                counts[1] = 1; // one
                counts[2] = if counts[2] == 2 { counts[2] + 1 } else {0}; // two
                counts[3] = 0;
                counts[4] = if counts[4] == 1 { counts[4] + 1 } else {0}; // four
                counts[5] = 0;
                counts[6] = 0; // six
                counts[7] = 0;
                counts[8] = 0;
                counts[9] = 0;
            },
            'r' => {
                counts[0] = if counts[0] == 3 { counts[0] + 1 } else {0}; // zero
                counts[1] = 0;
                counts[2] = 0; // two
                counts[3] = if counts[3] == 2 { counts[3] + 1 } else {0}; // three
                counts[4] = if counts[4] == 3 { counts[4] + 1 } else {0}; // four
                counts[5] = 0;
                counts[6] = 0; // six
                counts[7] = 0;
                counts[8] = 0;
                counts[9] = 0;
            },
            's' => {
                counts[0] = 0;
                counts[1] = 0;
                counts[2] = 0; // two
                counts[3] = 0;
                counts[4] = 0; // four
                counts[5] = 0;
                counts[6] = 1; // six
                counts[7] = 1; // seven
                counts[8] = 0;
                counts[9] = 0;
            },
            't' => {
                counts[0] = 0;
                counts[1] = 0;
                counts[2] = 1; // two
                counts[3] = 1; // three
                counts[4] = 0; // four
                counts[5] = 0;
                counts[6] = 0; // six
                counts[7] = 0;
                counts[8] = if counts[8] == 4 { counts[8] + 1} else {0}; // eight
                counts[9] = 0;
            },
            'u' => {
                counts[0] = 0;
                counts[1] = 0;
                counts[2] = 0; // two
                counts[3] = 0;
                counts[4] = if counts[4] == 2 { counts[4] + 1} else {0}; // four
                counts[5] = 0;
                counts[6] = 0; // six
                counts[7] = 0;
                counts[8] = 0;
                counts[9] = 0;
            },
            'v' => {
                counts[0] = 0;
                counts[1] = 0;
                counts[2] = 0; // two
                counts[3] = 0;
                counts[4] = 0; // four
                counts[5] = if counts[5] == 2 { counts[5] + 1} else {0}; // five
                counts[6] = 0; // six
                counts[7] = if counts[7] == 2 { counts[7] + 1} else {0}; // seven
                counts[8] = 0;
                counts[9] = 0;
            },
            'w' => {
                counts[0] = 0;
                counts[1] = 0;
                counts[2] = if counts[2] == 1 { counts[2] + 1} else {0}; // two
                counts[3] = 0;
                counts[4] = 0; // four
                counts[5] = 0;
                counts[6] = 0; // six
                counts[7] = 0;
                counts[8] = 0;
                counts[9] = 0;
            },
            'x' => {
                counts[0] = 0;
                counts[1] = 0;
                counts[2] = 0; // two
                counts[3] = 0;
                counts[4] = 0; // four
                counts[5] = 0;
                counts[6] = if counts[6] == 2 { counts[6] + 1} else {0}; // six
                counts[7] = 0;
                counts[8] = 0;
                counts[9] = 0;
            },
            'z' => {
                counts[0] = 1; // zero
                counts[1] = 0;
                counts[2] = 0; // two
                counts[3] = 0;
                counts[4] = 0; // four
                counts[5] = 0;
                counts[6] = 0; // six
                counts[7] = 0;
                counts[8] = 0;
                counts[9] = 0;
            },
            '\n' => {
                // let digits_str: String = digits.iter().collect();

                if !digits.is_empty() {
                    let first = String::from(*digits.first().unwrap());
                    let last = String::from(*digits.last().unwrap());
                    let number: i32 = (first.parse::<i32>().unwrap() * 10) + last.parse::<i32>().unwrap();
                    println!("{}", number);
                    numbers.push(number);
                }
                digits.clear();

                for i in 0..=9 { counts[i] = 0; }
            }
            _ => {
                for i in 0..=9 { counts[i] = 0; }
            }
        }
        if counts[0] == 4 { digits.push('0'); counts[0] = 0; }
        if counts[1] == 3 { digits.push('1'); counts[1] = 0; }
        if counts[2] == 3 { digits.push('2'); counts[2] = 0; }
        if counts[3] == 5 { digits.push('3'); counts[3] = 0; }
        if counts[4] == 4 { digits.push('4'); counts[4] = 0; }
        if counts[5] == 4 { digits.push('5'); counts[5] = 0; }
        if counts[6] == 3 { digits.push('6'); counts[6] = 0; }
        if counts[7] == 5 { digits.push('7'); counts[7] = 0; }
        if counts[8] == 5 { digits.push('8'); counts[8] = 0; }
        if counts[9] == 4 { digits.push('9'); counts[9] = 0; }
    }
    println!("total: {}", numbers.iter().sum::<i32>());
}