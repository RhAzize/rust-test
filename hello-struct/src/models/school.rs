pub struct School {
    pub total: u32,
    pub professor: u32,
}

impl School {
    pub fn new(total: u32, professor: u32) -> School {
        School { total, professor }
    }

    pub fn sum(&self) -> u32 {
        self.total + self.professor
    }
}
