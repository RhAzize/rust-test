struct Vehicle {
    model: String,
    year: i32,
    mileage: i32,
    is_running: bool,
}
#[derive(Debug)]
struct User {
    email: String,
    first_name: String,
    password: String,
    active: bool,
}

fn build_user(email: String, first_name: String, password: String, active: bool) -> User {
    let user = User {
        email,
        first_name,
        password,
        active,
    };
    user
}

fn main() {
    //test user
    let person = build_user(
        String::from("a@a.a"),
        String::from("john"),
        String::from("doe"),
        true,
    );

    println!(
        "Build user with email : {}, name : {}, password : {} and is active : {}",
        person.email, person.first_name, person.password, person.active
    );

    // update
    let person2 = User {
        email: String::from("test"),
        ..person // Copy and set directive #[derive(Debug)]
    };

    println!("person2:{:?}", person2);

    //test vehicle exo
    let vehicle: Vehicle = Vehicle {
        model: String::from("Chevrolet"),
        year: 1964,
        mileage: 20000,
        is_running: true,
    };

    println!("Vehicle Model: {}", vehicle.model);
    println!("Vehicle Year: {}", vehicle.year);
    println!("Mileage {}", vehicle.mileage);
    println!(
        "Status {}",
        if vehicle.is_running {
            "Running"
        } else {
            "Stopped"
        }
    );
}
