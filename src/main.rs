fn gcd(a: u64, b: u64) -> u64 {
    let mut x = a;
    let mut y = b;
    while y != 0 {
        let temp = y;
        y = x % y;
        x = temp;
    }
    x
}

fn relatively_prime_elements(n: u64) -> Vec<u64> {
    let mut result = Vec::new();
    for i in 1..n {
        if gcd(i, n) == 1 {
            result.push(i);
        }
    }
    result
}

fn multiplication_table(group: &[u64], mod_value: u64) {
    println!("Multiplication Table (mod {}):", mod_value);
    // Print the header row
    print!("    ");
    for &b in group {
        print!("{:4}", b);
    }
    println!();

    // Print the table
    for &a in group {
        print!("{:2} ", a);
        for &b in group {
            print!("{:4}", (a * b) % mod_value);
        }
        println!();
    }
}

fn main() {
    let n = 16; // Example value
    let group = relatively_prime_elements(n);
    multiplication_table(&group, n);
}




