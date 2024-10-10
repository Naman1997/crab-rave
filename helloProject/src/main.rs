struct BankAccount{
    owner: String,
    balance: f64,
}

const PI: f64 = 3.1414231;

impl BankAccount {
    fn check_balance(&self){
        println!("Owner {} has balance {}", self.owner, self.balance);
    }

    fn withdraw(&mut self, amount: f64){
        self.balance -= amount;
        println!("Amount {} was withdrawn from account of {}", amount, self.owner);
    }
}

fn main() {
    // let x: i32 = -45;
    // let y: u64 = 3211;
    // let is_cold: bool = true;
    // let letter: char = 'N';
    // let array: [i32; 5] = [1, 3, 5, 4, 1];
    // let fruits: [&str; 3] = ["Apple", "Banana", "Pine"];
    // let humans: (&str, i32, bool) = ("Alice", 30 ,false);
    // let slices: &[i32] = &[1,2,3,4,5];
    // let mut stone: String = String::from("Hello!");
    // stone.push_str(" World!");
    // let sliced: &str = &stone[0..6];

    human_id(27, "Naman", 182.8);
    test();
    tell_height(200);

    println!("The value of PI is {}", PI);

    let mut x:u16 = 6;
    x = x + 1;
    println!("Value of mutable x is {}", x);

    let y:u16 = 50;
    let y = y*2;
    println!("Value of shadowed y is {}", y);

    let mut _a: i32 = 45;
    let reference: &mut i32 = &mut _a;

    *reference += 1;
    *reference += 1;

    // println!("Value: {}", _a);
    println!("Value Reference: {}", reference);

    let mut bank_account: BankAccount = BankAccount{
        owner: String::from("Naman"),
        balance: 500000.0,
    };

    bank_account.check_balance();
    bank_account.withdraw(500.50);
    bank_account.check_balance();

    

    // println!("Hello, crab from cargo!");
    // println!("Signed Integer: {}", x);
    // println!("Unsigned Integer: {}", y);
    // println!("Boolean: {}", is_cold);
    // println!("Name: {}", letter);
    // println!("Array: {:?}", array);
    // println!("Fruits: {:?}", fruits);
    // println!("First Fruit: {:?}", fruits[0]);
    // println!("Tuples: {:?}", humans);
    // println!("Slices: {:?}", slices);
    // println!("Naman says: {}", stone);
    // println!("Naman slice says: {}", sliced);
    // println!("BMI is: {:.2}", calc_bmi(1.8, 65.2));
    // println!("String is: {:?} and len is {}", stone, calc_len(&stone));
}

fn test() {
    println!("Hello rust!")
}

fn tell_height(height: u32) {
    println!("Height is {}", height);
}

fn human_id(age: u32, name: &str, height: f32){
    println!("My name is {}, my age is {} and my height is {}cm", name, age, height);
}

// fn calc_bmi(height: f32, weight: f32) -> f32 {
//     weight / (height * height)
// }

// fn calc_len(string: &String) -> usize {
//     string.len()
// }