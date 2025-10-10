// clippy2.rs
// 
// Execute `rustlings hint clippy2` or use the `hint` watch subcommand for a
// hint.


fn main() {
    let mut res = 42;
    let mut option = Some(12);
    
    // 使用 if let 代替 while let，避免无限循环
    if let Some(x) = option.take() {
        res += x;
    }

    println!("{}", res);
}
