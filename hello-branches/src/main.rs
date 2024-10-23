fn main() {
    //Condition 
    condition();

    //loop
    looping();
}

fn condition() {

    let x:bool = true;

    let compare = if x { 1 } else {0}; 

    println!("Compare {}", compare);

}

fn looping() {
    let mut counter = 0;
    let result = loop {

        counter +=1;

        println!("counter {counter}");

        if counter == 10 {
            // break en retournant la val calculée.
            break counter * 2;
        }
    };
    
    println!("result = {result}"); // 20

}
