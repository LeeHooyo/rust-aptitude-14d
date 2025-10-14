// Day05 - Structs & Traits

use std::fmt::{self, Debug, Display};

// -------------------------
// 1. 기본 구조체
// -------------------------

#[derive(Debug, Clone)]
struct User {
    username: String,
    email: String,
    active: bool,
}

// -------------------------
// 2. 구조체에 메서드 구현
// -------------------------
impl User {
    fn new(username: &str, email: &str) -> Self {
        Self {
            username: username.to_string(),
            email: email.to_string(),
            active: true,
        }
    }

    fn deactivate(&mut self) {
        self.active = false;
    }

    fn is_active(&self) -> bool {
        self.active
    }

    fn summary(&self) -> String {
        format!("User<{}, {}>", self.username, self.email)
    }
}

// -------------------------
// 3. Trait 정의 및 구현
// -------------------------
trait Summarizable {
    fn summarize(&self) -> String;
}

impl Summarizable for User {
    fn summarize(&self) -> String {
        format!("{} ({})", self.username, if self.active { "active" } else { "inactive" })
    }
}

// -------------------------
// 4. 제네릭 + 트레잇 바운드
// -------------------------
fn print_summary<T: Summarizable + Debug>(item: &T) {
    println!("[Debug] {:?}\n[Summary] {}", item, item.summarize());
}


// -------------------------
// 5. Display 트레잇 수동 구현
// -------------------------
impl Display for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} <{}>", self.username, self.email)
    }
}

// -------------------------
// main()
// -------------------------
fn main() {
    let mut user = User::new("ferris", "ferris@example.com");
    println!("Created: {}", user);

    user.deactivate();
    println!("Active? {}", user.is_active());

    print_summary(&user);
}
