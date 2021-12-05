mod util;
mod day1;
mod day2;

fn main() {
    let (day, answer) = day2::day2b(util::read_input_file());
    println!("{} answer: {}", day, answer);
}

#[allow(dead_code)]
fn day0<I>(_input: I) -> (&'static str, i32)
    where I: Iterator<Item=String>
{
    ("day0", 0)
}