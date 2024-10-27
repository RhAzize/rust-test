pub struct Book {
    pub title: String,
    pub author: String,
    pub page: u32,
    available: bool,
}

impl Book {
    pub fn new(title: String, author: String, page: u32) -> Book {
        Book {
            title,
            author,
            page,
            available: true,
        }
    }

    pub fn borrowed(&mut self) {
        if self.available {
            self.available = false;
            println!("Tu as emprunté {}.", self.title);
        } else {
            println!("Le livre {} n'est plus dispo.", self.title);
        }
    }

    pub fn return_book(&mut self) {
        self.available = true;
        println!("Tu as retourné {}.", self.title);
    }
}
