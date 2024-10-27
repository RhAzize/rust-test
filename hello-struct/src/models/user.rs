#[derive(Debug)]
pub struct User {
    pub email: String,
    pub first_name: String,
    pub password: String,
    pub active: bool,
}

pub fn build_user(email: String, first_name: String, password: String, active: bool) -> User {
    let user = User {
        email,
        first_name,
        password,
        active,
    };
    user
}
