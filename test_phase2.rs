fn bad_loops() {
    let items = vec![1, 2, 3, 4, 5];
    
    for i in 0..items.len() {
        let item = items[i].clone();
        println!("{}", item);
    }
    
    let mut results = Vec::new();
    for i in 0..10 {
        results.push(i * 2);
    }
    
    let mut s = String::new();
    for i in 0..5 {
        s.push_str(&i.to_string());
    }
}

fn error_handling() {
    panic!("something went wrong");
    todo!();
    unreachable!();
    let x: Result<i32, &str> = Err("error");
    x.expect("failed");
}

fn conversions() {
    let s = "hello".to_owned();
    let x = 5 as u32;
    let y = some_value.into();
}

fn collections_in_loop() {
    use std::collections::HashMap;
    
    for i in 0..10 {
        let mut map = HashMap::new();
        map.insert(i, i * 2);
    }
}

fn string_concat() {
    let a = "a";
    let b = "b";
    let c = "c";
    let d = "d";
    let s = a.to_string() + b + c + d;
}

fn redundant_clone() {
    let data = vec![1, 2, 3];
    let a = data.clone().clone();
    let b = data.clone().to_string();
}

fn main() {
    bad_loops();
    error_handling();
    conversions();
    collections_in_loop();
    string_concat();
    redundant_clone();
}