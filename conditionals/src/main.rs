fn main() {
    let mut x: u32 = 0;

    'breakboi: loop {
        x += 1;
        if x%3 == 0 && x%5 == 0 {
            println!("FizzBuzz - {}", x);
        } else if x%3 == 0 {
            println!("Fizz     - {}", x);
        } else if x%5 == 0 {
            println!("Buzz     - {}", x);
        } else {
            // Skip
        }

        if x == 100 {
            break 'breakboi;
        }

    }
    
    let arr = [2,42,4121,5322];
    for elem in arr {
        println!("{elem}");
    }
}
