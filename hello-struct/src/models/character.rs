pub struct Character {
    name: String,
    health: u32,
    level: u32,
}

impl Character {
    pub fn new(name: String) -> Self {
        Self {
            name,
            health: 100,
            level: 1,
        }
    }

    pub fn take_damage(&mut self, damage: u32) {
        if self.health > 0 {
            self.health -= damage;
            println!("Il vous reste {} points de santé.", self.health);
        } else {
            self.health = 0;
            println!("tu es mort");
        }
    }
    pub fn level_up(&mut self) {
        self.level += 1;
        println!(
            "Vous avez atteint le level {} avec {}",
            self.level, self.name
        );
    }
}
