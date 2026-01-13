// STEP 6-1 ~ 6-4: async/await 기초
// 실행: cargo run --example async_basic
// Cargo.toml에 tokio = { version = "1", features = ["full"] } 필요

use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    // ========================================
    // 6-1. async/await 기초
    // ========================================

    println!("=== async/await 기초 ===");

    // async 함수 호출 (즉시 실행 안 됨, Future 반환)
    let future = hello_async();
    println!("Future created, not yet executed");

    // .await로 실행
    let result = future.await;
    println!("Result: {}", result);

    // 직접 await
    let greeting = greet("Rust").await;
    println!("{}", greeting);

    // ========================================
    // 6-2. Future 이해
    // ========================================

    println!("\n=== Future ===");

    // async 블록
    let async_block = async {
        println!("Inside async block");
        42
    };

    let value = async_block.await;
    println!("Async block returned: {}", value);

    // ========================================
    // 6-3, 6-4. tokio와 sleep
    // ========================================

    println!("\n=== tokio sleep ===");

    println!("Starting delayed task...");
    let start = std::time::Instant::now();

    delayed_message("First", 2).await;
    delayed_message("Second", 1).await;

    println!("Sequential execution took: {:?}", start.elapsed());

    // 동시 실행과 비교
    println!("\n=== 동시 실행 비교 ===");
    let start2 = std::time::Instant::now();

    // join!으로 동시 실행
    let (r1, r2) = tokio::join!(
        delayed_message("First", 2),
        delayed_message("Second", 1)
    );

    println!("Concurrent execution took: {:?}", start2.elapsed());
    println!("Results: {}, {}", r1, r2);
}

// 기본 async 함수
async fn hello_async() -> String {
    "Hello from async!".to_string()
}

// 파라미터를 받는 async 함수
async fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

// 딜레이가 있는 async 함수
async fn delayed_message(msg: &str, seconds: u64) -> String {
    println!("  {} starting...", msg);
    sleep(Duration::from_secs(seconds)).await;
    println!("  {} done after {} seconds", msg, seconds);
    format!("{} completed", msg)
}
