# STEP 6. Rust 비동기 프로그래밍

> async/await, Future, tokio를 활용한 비동기 처리

---

## 학습 목표
- async/await 문법 이해
- Future 트레이트 이해
- tokio 런타임 사용
- 비동기 에러 처리

---

## 6-1. async/await 기초

### 핵심 개념

```rust
async fn hello() -> String {
    "Hello, async!".to_string()
}

async fn main_async() {
    let result = hello().await;
    println!("{}", result);
}
```

### JavaScript 비교
```javascript
async function hello() {
    return "Hello, async!";
}

async function main() {
    const result = await hello();
    console.log(result);
}
```

### Rust의 차이점
- Rust의 async 함수는 호출 시 Future를 반환 (즉시 실행 안 됨)
- `.await`를 해야 실제 실행
- 런타임(tokio, async-std 등)이 필요

---

## 6-2. Future 트레이트

### 핵심 개념

```rust
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

// Future 트레이트 (단순화)
trait SimpleFuture {
    type Output;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}

// Poll 열거형
enum Poll<T> {
    Ready(T),      // 완료됨
    Pending,       // 아직 대기 중
}
```

### async 함수는 Future를 반환
```rust
// 이 함수는
async fn foo() -> i32 {
    42
}

// 이것과 동일
fn foo() -> impl Future<Output = i32> {
    async { 42 }
}
```

---

## 6-3. tokio 런타임

### 핵심 개념

```toml
# Cargo.toml
[dependencies]
tokio = { version = "1", features = ["full"] }
```

```rust
use tokio;

#[tokio::main]
async fn main() {
    let result = async_task().await;
    println!("Result: {}", result);
}

async fn async_task() -> i32 {
    // 비동기 작업
    42
}
```

### 여러 런타임
- **tokio**: 가장 인기, 완전한 기능
- **async-std**: std 라이브러리와 유사한 API
- **smol**: 가벼운 런타임

---

## 6-4. 비동기 sleep

### 핵심 개념

```rust
use tokio::time::{sleep, Duration};

async fn delayed_hello() {
    println!("Starting...");
    sleep(Duration::from_secs(2)).await;
    println!("Hello after 2 seconds!");
}

#[tokio::main]
async fn main() {
    delayed_hello().await;
}
```

### 주의: std::thread::sleep vs tokio::time::sleep
```rust
// BAD: 스레드를 블로킹
std::thread::sleep(Duration::from_secs(1));

// GOOD: 비동기적으로 대기
tokio::time::sleep(Duration::from_secs(1)).await;
```

---

## 6-5. 동시 실행 (join!)

### 핵심 개념

```rust
use tokio::join;

async fn task1() -> i32 {
    tokio::time::sleep(Duration::from_secs(1)).await;
    1
}

async fn task2() -> i32 {
    tokio::time::sleep(Duration::from_secs(1)).await;
    2
}

#[tokio::main]
async fn main() {
    // 동시에 실행, 모두 완료될 때까지 대기
    let (r1, r2) = join!(task1(), task2());
    println!("Results: {}, {}", r1, r2);  // 1초 후 출력 (2초 아님!)
}
```

### JavaScript 비교
```javascript
const [r1, r2] = await Promise.all([task1(), task2()]);
```

---

## 6-6. 경쟁 실행 (select!)

### 핵심 개념

```rust
use tokio::select;

async fn race_example() {
    select! {
        _ = async_task_slow() => {
            println!("Slow task finished first");
        }
        _ = async_task_fast() => {
            println!("Fast task finished first");
        }
    }
}
```

### JavaScript 비교
```javascript
await Promise.race([slowTask(), fastTask()]);
```

---

## 6-7. spawn (태스크 생성)

### 핵심 개념

```rust
use tokio::spawn;

#[tokio::main]
async fn main() {
    // 새 태스크 생성 (백그라운드 실행)
    let handle = spawn(async {
        println!("Running in background");
        42
    });

    // 다른 작업 수행
    println!("Main task continues");

    // 태스크 완료 대기
    let result = handle.await.unwrap();
    println!("Background task returned: {}", result);
}
```

---

## 6-8. 비동기 에러 처리

### 핵심 개념

```rust
use std::io;

async fn read_file_async(path: &str) -> Result<String, io::Error> {
    tokio::fs::read_to_string(path).await
}

#[tokio::main]
async fn main() -> Result<(), io::Error> {
    let content = read_file_async("hello.txt").await?;
    println!("Content: {}", content);
    Ok(())
}
```

### anyhow 크레이트
```rust
use anyhow::{Result, Context};

async fn do_something() -> Result<()> {
    let content = tokio::fs::read_to_string("file.txt")
        .await
        .context("Failed to read file")?;
    Ok(())
}
```

---

## 6-9. 비동기 채널

### 핵심 개념

```rust
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(32);

    // 송신자
    let tx2 = tx.clone();
    tokio::spawn(async move {
        tx.send("hello").await.unwrap();
    });

    tokio::spawn(async move {
        tx2.send("world").await.unwrap();
    });

    // 수신자
    while let Some(message) = rx.recv().await {
        println!("Received: {}", message);
    }
}
```

---

## 6-10. 비동기 Mutex

### 핵심 개념

```rust
use tokio::sync::Mutex;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let counter = Arc::new(Mutex::new(0));

    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = tokio::spawn(async move {
            let mut num = counter.lock().await;
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.await.unwrap();
    }

    println!("Result: {}", *counter.lock().await);
}
```

---

## 예제 파일
- `examples/async_basic.rs` - async/await 기초
- `examples/tokio_tasks.rs` - tokio 태스크와 동시성
- `examples/channels.rs` - 비동기 채널

---

## 다음 단계
→ STEP 7: Rust 웹 개발
