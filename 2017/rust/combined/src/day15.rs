pub fn run() {
    let a_start: usize = 679;
    let b_start: usize = 771;
    let a_factor = 16807;
    let b_factor = 48271;
    let remainder = 2_147_483_647;

    let mut a = a_start;
    let mut b = b_start;

    let mut matches = 0;
    let mut a_multipe4 = Vec::new();
    let mut b_multiple8 = Vec::new();
    let mut i = 0;

    let mask = 65535;
    while a_multipe4.len() <= 5_000_000 || b_multiple8.len() <= 5_000_000 {
        a = (a * a_factor) % remainder;
        b = (b * b_factor) % remainder;
        if (a % 4) == 0 {
            a_multipe4.push(a & mask)
        }

        if (b % 8) == 0 {
            b_multiple8.push(b & mask)
        }

        if i <= 400_000_000 && a & mask == b & mask {
            matches += 1;
        }
        i += 1;
    }

    println!("Part 1 is {}", matches);
    let mut matches2 = 0;
    for i in 0..5_000_000 {
        if a_multipe4[i] == b_multiple8[i] {
            matches2 += 1;
        }
    }
    println!("Part 2 is {}", matches2);
}
