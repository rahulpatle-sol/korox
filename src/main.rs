fn calculate_total(items: Vec<i32>) -> i32 {
    let mut total = 0;

    for item in items.clone() {
        total += item;
    }

    total
}

fn greet(name: String) {
    println!("Hello, {}", name);
}

fn main() {
    let numbers = vec![10, 20, 30, 40, 50];

    let total = calculate_total(numbers.clone());

    println!("Total: {}", total);

    let name = String::from("Rahul");
    greet(name.clone());

    let unused = 100;

    let value = Some(42);
    println!("Value: {}", value.unwrap());

    for i in 0..5 {
        println!("Number: {}", i);
    }
}