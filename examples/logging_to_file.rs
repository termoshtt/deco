use deco::deco;
use std::{fs, io::Write};

fn logging<InputFunc: 'static>(
    log_filename: &'static str,
) -> impl Fn(InputFunc) -> Box<dyn Fn(i32) -> i32>
where
    InputFunc: Fn(i32) -> i32,
{
    move |func: InputFunc| {
        Box::new(move |i: i32| {
            let mut f = fs::File::create(log_filename).unwrap();
            writeln!(f, "Input = {}", i).unwrap();
            let out = func(i);
            writeln!(f, "Output = {}", out).unwrap();
            out
        })
    }
}

#[deco(logging("test.log"))]
fn add2(i: i32) -> i32 {
    i + 2
}

fn main() {
    add2(2);
}
