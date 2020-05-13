use deco::deco;

fn logging<F>(func: F) -> impl Fn(i32) -> i32
where
    F: Fn(i32) -> i32,
{
    move |i| {
        println!("Input = {}", i);
        let out = func(i);
        println!("Output = {}", out);
        out
    }
}

#[deco(logging)]
fn add2(i: i32) -> i32 {
    i + 2
}

fn main() {
    add2(2);
}
