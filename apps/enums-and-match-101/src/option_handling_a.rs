pub fn option_handle_a() {
    println!("\n----------------Option<T> Handling------------------\n");
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    match y {
        Some(v) => println!("v + x = {}", v + x),
        None => println!("y is None"),
    }


    println!("option_unwrapping : {}", option_unwrapping());
    println!("option_if_let : {}", option_if_let());

    match option_with_map() {
        Some(val) => println!("match Some arm's value is {val}"),
        None => println!("Match None arm executed"),
    }
}

fn option_unwrapping() -> i8 {
    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    let sum = x + y.unwrap_or(0);
    sum
}

fn option_if_let() -> i8 {
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    if let Some(val) = y {
        let sum = x + val;
        sum
    } else {
        let sum = x + 0;
        sum
    }
}

fn option_with_map() -> Option<i8> {
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let sum = y.map(|val| x + val);
    sum
}