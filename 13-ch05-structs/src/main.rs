fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("some_username"),
        email: String::from("someemial@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("newemail@example.com");

    // let user2 = User {
    //     active: user1.active,
    //     username: user1.username,
    //     email: String::from("other@example.com"),
    //     sign_in_count: user1.sign_in_count
    // }; bottom example is better

    let user2 = User {
        email: String::from("other@example.com"),
        ..user1
    };

    user1.email = String::from("newemail2@example.com");

    print!("{} {}", user2.email, user1.email);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let subject = AlwaysEqual;
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

struct AlwaysEqual;

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
