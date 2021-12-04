mod util;
mod day1;

fn main() {
    let (day, answer) = day0(util::read_input_file());
    println!("{} answer: {}", day, answer);
}

fn day0<I>(_input: I) -> (&'static str, i32)
    where I: Iterator<Item=String>
{
    ("day0", 0)
}