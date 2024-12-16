
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: i64,
}

fn build_user(email: String, username: String)->User{
    User{
        username: username,
        email: email,
        sign_in_count: 1,
        active: true,
    }
}

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    user1.email = String::from("anotheremail@example.com");
}
