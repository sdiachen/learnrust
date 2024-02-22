fn main() {
    let mut user1 = build_user(
        String::from("someusername123"),
        String::from("someusername123@mail.com"),
    );

    user1.email = String::from("someoneelse@example.com");

    let user2 = User {
        email: String::from("anotheruser@mail.com"),
        ..user1
    };

    let black = Color(0,0,0);
    println!("{}", black.0);
}

fn build_user(username: String, email: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: i64,
}

struct Color(i32,i32,i32);
