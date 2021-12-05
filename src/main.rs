mod util;
mod day1a;
mod day2a;
mod day1b;
mod day2b;
mod day3a;

fn main() {
    let (day, answer) = day3a::day3a(util::read_input_file());
    println!("{} answer: {}", day, answer);
}

#[allow(dead_code)]
fn day0<I>(_input: I) -> (&'static str, i32)
    where I: Iterator<Item=String>
{
    ("day0", 0)
}