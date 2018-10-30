fn main() {
    let step_size = 371;
    let mut position = 0;
    let mut circular_buffer = Vec::with_capacity(2017);
    for i in 0..2018 {
        if position == circular_buffer.len() {
            circular_buffer.push(i);
        } else {
            circular_buffer.insert(position, i);
        }


        if i == 2017 {
            println!("Part 1 is {:?}", circular_buffer[position + 1])
        }
        position = (position + step_size) % (i + 1) + 1;
    }

    position = 0;
    let mut answer = 0;
    for i in 0..50000000 {
        if position == 1 {
            answer = i;
        }
        position = (position + step_size) % (i + 1) + 1;
    }
    println!("Part 2 is {}", answer);
}
