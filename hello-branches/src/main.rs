fn main() {
    //Condition
    condition();

    //loop
    looping();

    //while
    while_loop();

    //for
    for_loop();

    //test conversion fahrebheit vers celsius
    let celsius = fahrenheit_to_celsius(95);
    println!("95 °F = {celsius} °C\n", celsius = celsius);

    //fibonacci
    let n:i32 = 10;
    let response:i32 = fibonacci(n);
    println!("fibonacci({n}) = {response}\n", n = n, response = response);
}

////Pseudo-code
// function fibonacci(n) {
//  if (n == 0) return 0
//  if (n == 1) return 1
//  prev = 0
//  curr = 1
//  for i from 2 to n {
//   next = prev + curr
//   prev = curr
//   curr = next
//  }
//  return curr
// }

fn fibonacci(n: i32) -> i32 {
    println!("-- Fibonacci --");
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        let mut prev = 0;
        let mut curr = 1;
        for index in 2..=n {
            let next = prev + curr;
            prev = curr;
            curr = next;
            println!("F({index}) = {next}");
        }
        curr
    }
}
fn fahrenheit_to_celsius(value: i32) -> i32 {
    println!("-- Fahrenheit to Celsius --");

    //Formule : (86 °F − 32) × 5/9 = 30 °C
    (value - 32) * 5 / 9
}
fn condition() {
    println!("-- Condition --");
    let x: bool = true;

    let compare = if x { true } else { false };

    println!("Result = {}\n", compare);
}

fn for_loop() {
    println!("-- For --");
    for number in (1..10).rev() {
        println!("number {number}");
    }
    println!("\n");
}
fn while_loop() {
    println!("-- While --");
    let mut counter = 0;

    while counter < 10 {
        println!("counter {counter}");
        counter += 1;
    }
    println!("\n");
}
fn looping() {
    println!("-- Loop --");
    let mut counter = 0;
    let result = loop {
        counter += 1;

        println!("counter {counter}");

        if counter == 10 {
            // break en retournant la val calculée.
            break counter * 2;
        }
    };

    println!("result = {result}\n"); // 20
}
