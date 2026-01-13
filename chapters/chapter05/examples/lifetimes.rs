// STEP 5-5, 5-6: 라이프타임

fn main() {
    // ========================================
    // 5-5. 라이프타임 기초
    // ========================================

    println!("=== 라이프타임 기초 ===");

    let string1 = String::from("long string is long");
    let string2 = String::from("xyz");

    let result = longest(&string1, &string2);
    println!("Longest: {}", result);

    // 다른 스코프에서
    let string3 = String::from("outer string");
    {
        let string4 = String::from("inner string");
        let result2 = longest(&string3, &string4);
        println!("Longest in scope: {}", result2);
    }

    // 첫 번째 문장 찾기
    let sentence = String::from("Hello world. How are you?");
    let first = first_word(&sentence);
    println!("First word: {}", first);

    // ========================================
    // 5-6. 구조체 라이프타임
    // ========================================

    println!("\n=== 구조체 라이프타임 ===");

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();

    let excerpt = ImportantExcerpt {
        part: first_sentence,
    };

    println!("Excerpt: {}", excerpt.part);
    println!("Level: {}", excerpt.level());

    let announcement = "Attention please!";
    println!("Announced: {}", excerpt.announce_and_return_part(announcement));

    // 정적 라이프타임
    println!("\n=== 정적 라이프타임 ===");
    let s: &'static str = "I have a static lifetime.";
    println!("{}", s);

    // 제네릭 + 트레이트 + 라이프타임 조합
    println!("\n=== 조합 예제 ===");
    let result = longest_with_announcement(&string1, &string2, "Comparing strings");
    println!("Result: {}", result);
}

// 라이프타임이 필요한 함수
// 반환값이 x와 y 중 하나를 참조하므로 라이프타임 명시 필요
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// 하나의 참조만 반환할 때 (라이프타임 생략 가능)
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// 참조를 포함하는 구조체
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    // 라이프타임 생략 규칙 1: &self 반환 없음
    fn level(&self) -> i32 {
        3
    }

    // 라이프타임 생략 규칙 3: &self가 있으면 반환값에 self의 라이프타임 적용
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

// 제네릭, 트레이트 바운드, 라이프타임 모두 사용
fn longest_with_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: std::fmt::Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// 라이프타임 생략 규칙 설명:
// 1. 각 입력 참조는 자체 라이프타임 파라미터를 가짐
// 2. 입력 라이프타임이 하나만 있으면 출력 라이프타임에 적용
// 3. 여러 입력 중 &self 또는 &mut self가 있으면 self의 라이프타임이 출력에 적용
