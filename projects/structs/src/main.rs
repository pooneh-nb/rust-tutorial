 struct User {
        active : bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

fn main() {

    let user1 = User {
        email: String::from("a@hg.com"),
        username: String::from("Pin"),
        active: true,
        sign_in_count: 1
    };
}

