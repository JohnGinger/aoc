fn main() {
    let a_start: usize = 679;
    let b_start: usize = 771;
    let a_factor = 16807;
    let b_factor = 48271;
    let remainder = 2147483647;

    let mut a = a_start;
    let mut b = b_start;

    let mut matches = 0;
    let mut a_multipe4 = Vec::new();
    let mut b_multiple8 = Vec::new();
    let mut i = 0;
    while a_multipe4.len() <= 5000000 || b_multiple8.len() <= 5000000 {
        a = (a * a_factor) % remainder;
        b = (b * b_factor) % remainder;
        let mut a_str = format!("{:032b}", a);
        let mut b_str = format!("{:032b}", b);

        if (a % 4) == 0 {
            a_multipe4.push(a_str.clone().split_off(16))
        }

        if (b % 8) == 0 {
            b_multiple8.push(b_str.clone().split_off(16))
        }

        if i <= 400000000 {
            if a_str.split_off(16) == b_str.split_off(16) {
                matches += 1;
            }
        }
        i += 1;
    }

    println!("Part 1 is {}", matches);
    let mut matches2 = 0;
    for i in 0..5000000 {
        if a_multipe4[i] == b_multiple8[i] {
            matches2 += 1;
        }
    }
    println!("Part 2 is {}", matches2);
}
