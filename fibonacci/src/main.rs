use std::io;

fn fibonacci(n: i64) -> i64 {
    let base_array = [[1, 1], [1, 0]];
    let mut result_array = base_array.clone();
    for _x in 1..n {
        result_array = multiply_matrices(result_array, base_array);
    }
    result_array[0][1]
}

// could generalise for arrays of any size, but for our purposes we'll stick to 2x2
fn multiply_matrices(multiple: [[i64; 2]; 2], base: [[i64; 2]; 2]) -> [[i64; 2]; 2] {
    let mut result_array = [[0, 0], [0, 0]];
    result_array[0][0] = multiple[0][0] * base[0][0] + multiple[0][1] * base[1][0];
    result_array[0][1] = multiple[0][0] * base[0][1] + multiple[0][1] * base[1][1];
    result_array[1][0] = multiple[1][0] * base[0][0] + multiple[1][1] * base[1][0];
    result_array[1][1] = multiple[1][0] * base[0][1] + multiple[1][1] * base[1][1];
    result_array
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();
    // could ask for input until user gives valid integer
    let value: i64 = input.parse().unwrap();
    let result = fibonacci(value);
    println!("Fibonacci value for {} is {}", value, result);
}
