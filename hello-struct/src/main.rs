mod models;
use models::book::Book;
use models::character::Character;
use models::school::School;
use models::user::{build_user, User};
use models::vehicle::Vehicle;

fn main() {
    /*
        Test implémentation de structure et de fonction.
        Création et import des modèles.
    */

    // Test Character
    let mut character = Character::new(String::from("Azize"));
    character.take_damage(20);
    character.take_damage(20);
    character.take_damage(20);
    character.take_damage(20);
    character.take_damage(20);
    character.take_damage(20);
    character.level_up();
    character.level_up();
    character.level_up();

    // Test Book
    let mut book = Book::new(String::from("Rust Programming"), String::from("azize"), 300);

    println!(
        "Nouveau livre ajouté: {} de {}, {} pages",
        book.title, book.author, book.page
    );
    book.borrowed();
    book.borrowed();
    book.return_book();
    book.borrowed();

    // Test school
    let school = School::new(10, 2);
    println!("Total var school: {}", school.sum());

    //test Personne
    let person = build_user(
        String::from("a@a.a"),
        String::from("john"),
        String::from("doe"),
        true,
    );

    println!(
        "email : {}, name : {}, password : {} and is active : {}",
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
