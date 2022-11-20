struct UserId(i64);

enum ESex {
    OTHER,
    MALE,
    FEMALE,
}

struct User {
    id: UserId,
    name: String,
    sex: ESex,
}

fn greet(user: &User) -> String {
    match user.sex {
        ESex::FEMALE => format!("good afternoon, miss {}", user.name),
        ESex::MALE => format!("good afternoon, mr {}", user.name),
        ESex::OTHER => format!("good afternoon, {}", user.name),
    }
}

fn main() {
    let tony = User {
        id: UserId(1),
        name: String::from("Tony"),
        sex: ESex::MALE,
    };
    let jenny = User {
        id: UserId(2),
        name: String::from("Jenny"),
        sex: ESex::FEMALE,
    };
    println!("{}", greet(&tony));
    println!("{}", greet(&jenny));
}
