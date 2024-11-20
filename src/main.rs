use ayan_learn_crate::{authenticate, Credentials};



fn main() {
    let user = Credentials {
        username: String::from("user1"),
        password: String::from("password123"),
    };
    authenticate(user);
    // You can now use 'user' directly
}
