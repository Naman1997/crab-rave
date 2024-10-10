use std::collections::HashMap;

fn divide(num: f64, den:f64) -> Result<f64, String> {
    if den == 0.0 {
        Err("Cannot divide by zero".to_string())
    } else {
        Ok(num/den)
    }
}



fn main() {
    let result = divide(10.0, 0.000);
    match result {
        Ok(x) => println!("Result is {}", x),
        Err(err) => println!("{err}"),
    }

    let mut vec: Vec<i32> = vec![1,2,3,4,5];
    vec.push(6);
    vec.push(7);
    vec.push(8);
    vec.push(9);
    vec.push(10);

    println!("{:?}", vec);

    let fifth: Option<&i32> = vec.get(40);
    match fifth {
        Some(x) => println!("Fifth item is {}", x),
        None => println!("Error!"),
    };

    //HashMaps
    let mut map: HashMap<String, u32> = HashMap::new();
    map.insert(String::from("Blue"), 10);
    map.insert(String::from("Red"), 150);

    for (key,value) in &map {
        println!("{key} = {value}");
    }
    
}
