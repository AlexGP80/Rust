struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    println!(
        "USER DATA\n=========\n{}\n{}\n{}\n{}\n",
        user2.username, user2.email, user2.active, user2.sign_in_count
    );

    // Next line causes compile error because user1 username String was moved to user2
    // println!(
    //     "USER DATA\n=========\n{}\n{}\n{}\n{}\n",
    //     user1.username, user1.email, user1.active, user1.sign_in_count
    // );
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
