// STEP 5-9 ~ 5-11: 스마트 포인터

use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    // ========================================
    // 5-9. Box<T>
    // ========================================

    println!("=== Box<T> ===");

    // 힙에 값 저장
    let b = Box::new(5);
    println!("b = {}", b);

    // Box는 Deref를 구현하여 참조처럼 사용 가능
    let x = 5;
    let y = Box::new(x);
    assert_eq!(5, *y);
    println!("Dereferenced: {}", *y);

    // 재귀적 타입 - Box 없이는 컴파일 불가
    println!("\n=== 재귀적 타입 (Cons List) ===");

    use List::{Cons, Nil};

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    fn print_list(list: &List) {
        match list {
            Cons(value, next) => {
                print!("{} -> ", value);
                print_list(next);
            }
            Nil => println!("Nil"),
        }
    }

    print_list(&list);

    // ========================================
    // 5-10. Rc<T> (Reference Counting)
    // ========================================

    println!("\n=== Rc<T> ===");

    let a = Rc::new(String::from("hello"));
    println!("Reference count after creating a: {}", Rc::strong_count(&a));

    let b = Rc::clone(&a);  // 참조 카운트 증가 (deep copy 아님)
    println!("Reference count after creating b: {}", Rc::strong_count(&a));

    {
        let c = Rc::clone(&a);
        println!("Reference count after creating c: {}", Rc::strong_count(&a));
        println!("a, b, c all point to: {}", c);
    }

    println!("Reference count after c goes out of scope: {}", Rc::strong_count(&a));

    // Rc로 공유 데이터 (그래프, 트리 등)
    println!("\n=== Rc로 공유 리스트 ===");

    use SharedList::{SharedCons, SharedNil};

    let shared = Rc::new(SharedCons(5, Rc::new(SharedCons(10, Rc::new(SharedNil)))));
    println!("count after shared = {}", Rc::strong_count(&shared));

    let branch1 = SharedCons(3, Rc::clone(&shared));
    println!("count after branch1 = {}", Rc::strong_count(&shared));

    let branch2 = SharedCons(4, Rc::clone(&shared));
    println!("count after branch2 = {}", Rc::strong_count(&shared));

    // ========================================
    // 5-11. RefCell<T> (Interior Mutability)
    // ========================================

    println!("\n=== RefCell<T> ===");

    let data = RefCell::new(5);

    println!("data before: {:?}", data.borrow());

    // 런타임에 빌림 규칙 체크
    *data.borrow_mut() += 10;

    println!("data after: {:?}", data.borrow());

    // borrow()와 borrow_mut() 사용
    {
        let borrowed = data.borrow();
        println!("Borrowed value: {}", borrowed);
        // let mut borrowed_mut = data.borrow_mut();  // 패닉! 이미 불변 빌림 중
    }

    // Rc + RefCell 조합 (여러 소유자 + 가변성)
    println!("\n=== Rc<RefCell<T>> 조합 ===");

    let shared_data = Rc::new(RefCell::new(vec![1, 2, 3]));

    let owner1 = Rc::clone(&shared_data);
    let owner2 = Rc::clone(&shared_data);

    owner1.borrow_mut().push(4);
    println!("After owner1 push: {:?}", shared_data.borrow());

    owner2.borrow_mut().push(5);
    println!("After owner2 push: {:?}", shared_data.borrow());

    // Mock 객체 예제
    println!("\n=== Mock 객체 패턴 ===");

    let mock = MockMessenger::new();
    let mut limiter = LimitTracker::new(&mock, 100);

    limiter.set_value(80);
    println!("Messages sent: {:?}", mock.sent_messages.borrow());
}

// 재귀적 타입
enum List {
    Cons(i32, Box<List>),
    Nil,
}

// Rc를 사용한 공유 리스트
enum SharedList {
    SharedCons(i32, Rc<SharedList>),
    SharedNil,
}

// Mock 객체 패턴 (테스트에서 유용)
trait Messenger {
    fn send(&self, msg: &str);
}

struct MockMessenger {
    sent_messages: RefCell<Vec<String>>,
}

impl MockMessenger {
    fn new() -> MockMessenger {
        MockMessenger {
            sent_messages: RefCell::new(vec![]),
        }
    }
}

impl Messenger for MockMessenger {
    fn send(&self, message: &str) {
        // &self인데도 내부 상태 변경 가능! (Interior Mutability)
        self.sent_messages.borrow_mut().push(String::from(message));
    }
}

struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T: Messenger> LimitTracker<'a, T> {
    fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage = self.value as f64 / self.max as f64;

        if percentage >= 1.0 {
            self.messenger.send("Error: Over quota!");
        } else if percentage >= 0.9 {
            self.messenger.send("Warning: 90% of quota used");
        } else if percentage >= 0.75 {
            self.messenger.send("Warning: 75% of quota used");
        }
    }
}
