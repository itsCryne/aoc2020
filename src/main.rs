mod days;
use days::{one, two, three};
use std::fs::read_to_string;
use std::time::Instant;

fn fmt_time(ms: f64) -> String {
    if ms <= 1.0 {
        let micro_sec = ms * 1000.0;
        return String::from(format!("{}µs", micro_sec.round()));
    }

    if ms < 1000.0 {
        let whole_ms = ms.floor();
        let rem_ms = ms - whole_ms;
        return String::from(format!("{}ms ", whole_ms) + &fmt_time(rem_ms));
    }

    let sec: f64 = ms / 1000.0;
    if sec < 60.0 {
        let whole_sec = sec.floor();
        let rem_ms = ms - whole_sec * 1000.0;

        return format!("{}s ", whole_sec) + &fmt_time(rem_ms);
    }

    let min: f64 = sec / 60.0;
    return format!("{}m ", min.floor()) + &fmt_time((sec % 60.0) * 1000.0);
}

fn main() {
    let one_input = read_to_string("src/inputs/one.txt").expect("Konnte Eingabe für Tag 1 nicht lesen");

    let one_a_start_time = Instant::now();
    let one_a_result = one::a(&one_input);
    let one_a_end_time = one_a_start_time.elapsed().as_secs_f64() * 1000.0;

    let one_b_start_time = Instant::now();
    let one_b_result = one::b(&one_input);
    let one_b_end_time = one_b_start_time.elapsed().as_secs_f64() * 1000.0;

    println!("Day 1:\n A: {} in {}\n B: {} in {}", one_a_result, fmt_time(one_a_end_time), one_b_result, fmt_time(one_b_end_time));


    let two_input = read_to_string("src/inputs/two.txt").expect("Konnte Eingabe für Tag 2 nicht lesen");

    let two_a_start_time = Instant::now();
    let two_a_result = two::a(&two_input);
    let two_a_end_time = two_a_start_time.elapsed().as_secs_f64() * 1000.0;

    let two_b_start_time = Instant::now();
    let two_b_result = two::b(&two_input);
    let two_b_end_time = two_b_start_time.elapsed().as_secs_f64() * 1000.0;

    println!("Day 2:\n A: {} in {}\n B: {} in {}", two_a_result, fmt_time(two_a_end_time), two_b_result, fmt_time(two_b_end_time));


    let three_input = read_to_string("src/inputs/three.txt").expect("Konnte Eingabe für Tag 3 nicht lesen");

    let three_a_start_time = Instant::now();
    let three_a_result = three::a(&three_input);
    let three_a_end_time = three_a_start_time.elapsed().as_secs_f64() * 1000.0;

    let three_b_start_time = Instant::now();
    let three_b_result = three::b(&three_input);
    let three_b_end_time = three_b_start_time.elapsed().as_secs_f64() * 1000.0;

    println!("Day 3:\n A: {} in {}\n B: {} in {}", three_a_result, fmt_time(three_a_end_time), three_b_result, fmt_time(three_b_end_time));

}
