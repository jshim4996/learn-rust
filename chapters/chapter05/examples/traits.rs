// STEP 5-3, 5-4: 트레이트

use std::fmt::Display;

fn main() {
    // ========================================
    // 5-3. 트레이트 정의와 구현
    // ========================================

    println!("=== 트레이트 ===");

    let article = Article {
        title: String::from("Rust 2024 Released"),
        author: String::from("Rust Team"),
        content: String::from("The Rust team is happy to announce a new version of Rust with amazing features..."),
    };

    let tweet = Tweet {
        username: String::from("rustlang"),
        content: String::from("Rust is awesome! Memory safety without garbage collection."),
        reply: false,
        retweet: false,
    };

    println!("Article: {}", article.summarize());
    println!("Tweet: {}", tweet.summarize());

    // 기본 구현 사용
    println!("Article default: {}", article.default_summary());
    println!("Tweet default: {}", tweet.default_summary());

    // ========================================
    // 5-4. 트레이트 바운드
    // ========================================

    println!("\n=== 트레이트 바운드 ===");

    // impl Trait 문법
    notify(&article);
    notify(&tweet);

    // 제네릭 바운드
    notify_generic(&article);

    // 반환 타입으로 impl Trait
    let summarizable = returns_summarizable();
    println!("Returned: {}", summarizable.summarize());

    // 여러 트레이트 바운드
    let displayable = DisplayableArticle {
        title: String::from("Display + Summary"),
        content: String::from("This implements both traits"),
    };
    notify_and_display(&displayable);

    // 조건부 메서드 구현
    let pair = Pair::new(10, 20);
    pair.cmp_display();
}

// ========================================
// 트레이트 정의
// ========================================

trait Summary {
    fn summarize(&self) -> String;

    // 기본 구현
    fn default_summary(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }

    fn summarize_author(&self) -> String {
        String::from("Unknown")
    }
}

// ========================================
// 트레이트 구현
// ========================================

struct Article {
    title: String,
    author: String,
    content: String,
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{}, by {}", self.title, self.author)
    }

    fn summarize_author(&self) -> String {
        self.author.clone()
    }
}

struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("@{}: {}", self.username, self.content)
    }

    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

// ========================================
// 트레이트 바운드
// ========================================

// impl Trait 문법 (간단한 경우)
fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// 제네릭 바운드 (동일한 의미)
fn notify_generic<T: Summary>(item: &T) {
    println!("Generic notify: {}", item.summarize());
}

// 반환 타입으로 impl Trait
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("bot"),
        content: String::from("Auto-generated tweet"),
        reply: false,
        retweet: false,
    }
}

// 여러 트레이트 바운드
struct DisplayableArticle {
    title: String,
    content: String,
}

impl Summary for DisplayableArticle {
    fn summarize(&self) -> String {
        self.title.clone()
    }
}

impl Display for DisplayableArticle {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[{}] {}", self.title, self.content)
    }
}

fn notify_and_display<T: Summary + Display>(item: &T) {
    println!("Display: {}", item);
    println!("Summary: {}", item.summarize());
}

// where 절을 사용한 바운드
fn _some_function<T, U>(_t: &T, _u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + std::fmt::Debug,
{
    42
}

// 조건부 메서드 구현
struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// Display + PartialOrd를 구현하는 T에만 cmp_display 메서드 추가
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
