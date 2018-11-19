fn main() {
    for i in (0..10) {
        println!("{}", fibonacci(i))
    }
}




fn f_to_c(f: i32) -> f64 {
    (f as f64 - 32.) * (5./9.)
}

fn fibonacci(n: i32) -> i32 {
    if n == 0 || n == 1 {
        return 1;
    }

    return fibonacci(n - 1) + fibonacci(n - 2);
}