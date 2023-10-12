// clippy2.rs
// 
// Execute `rustlings hint clippy2` or use the `hint` watch subcommand for a
// hint.


fn main() {
    let mut res = 42;
    if let Some(value) = Some(12)
    {
        res += value;
    }
    println!("{}", res);
}
