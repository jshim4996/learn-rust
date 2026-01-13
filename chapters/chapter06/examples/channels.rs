// STEP 6-8 ~ 6-10: 비동기 채널과 동기화
// 실행: cargo run --example channels
// Cargo.toml에 tokio = { version = "1", features = ["full"] } 필요

use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{mpsc, oneshot, Mutex, RwLock};
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    // ========================================
    // 6-8. 비동기 에러 처리
    // ========================================

    println!("=== 비동기 에러 처리 ===");

    match read_file_async("nonexistent.txt").await {
        Ok(content) => println!("Content: {}", content),
        Err(e) => println!("Error reading file: {}", e),
    }

    // ? 연산자 사용
    if let Err(e) = async_with_error().await {
        println!("Caught error: {}", e);
    }

    // ========================================
    // 6-9. 비동기 채널 (mpsc)
    // ========================================

    println!("\n=== mpsc 채널 ===");

    // mpsc: Multiple Producer, Single Consumer
    let (tx, mut rx) = mpsc::channel::<String>(32);

    // 여러 송신자
    let tx1 = tx.clone();
    let tx2 = tx.clone();

    tokio::spawn(async move {
        for i in 0..3 {
            tx1.send(format!("Producer 1: message {}", i)).await.unwrap();
            sleep(Duration::from_millis(100)).await;
        }
    });

    tokio::spawn(async move {
        for i in 0..3 {
            tx2.send(format!("Producer 2: message {}", i)).await.unwrap();
            sleep(Duration::from_millis(150)).await;
        }
    });

    // 원래 tx를 drop해야 채널이 닫힘
    drop(tx);

    // 수신자
    while let Some(msg) = rx.recv().await {
        println!("  Received: {}", msg);
    }
    println!("Channel closed");

    // oneshot 채널 (일회성)
    println!("\n=== oneshot 채널 ===");

    let (tx, rx) = oneshot::channel::<String>();

    tokio::spawn(async move {
        sleep(Duration::from_millis(500)).await;
        tx.send("One-time message".to_string()).unwrap();
    });

    match rx.await {
        Ok(msg) => println!("Received oneshot: {}", msg),
        Err(_) => println!("Sender dropped"),
    }

    // ========================================
    // 6-10. 비동기 Mutex
    // ========================================

    println!("\n=== 비동기 Mutex ===");

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for i in 0..5 {
        let counter = Arc::clone(&counter);
        let handle = tokio::spawn(async move {
            // 비동기 락
            let mut num = counter.lock().await;
            *num += 1;
            println!("  Task {} incremented counter to {}", i, *num);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.await.unwrap();
    }

    println!("Final counter value: {}", *counter.lock().await);

    // RwLock (Read-Write Lock)
    println!("\n=== RwLock ===");

    let data = Arc::new(RwLock::new(vec![1, 2, 3]));

    // 여러 읽기 동시 가능
    let data1 = Arc::clone(&data);
    let data2 = Arc::clone(&data);

    let read1 = tokio::spawn(async move {
        let guard = data1.read().await;
        println!("  Reader 1 sees: {:?}", *guard);
    });

    let read2 = tokio::spawn(async move {
        let guard = data2.read().await;
        println!("  Reader 2 sees: {:?}", *guard);
    });

    tokio::join!(read1, read2);

    // 쓰기는 독점
    {
        let mut guard = data.write().await;
        guard.push(4);
        println!("  Writer added 4");
    }

    println!("Final data: {:?}", *data.read().await);
}

// 비동기 파일 읽기 (에러 처리 예시)
async fn read_file_async(path: &str) -> Result<String, std::io::Error> {
    tokio::fs::read_to_string(path).await
}

// ? 연산자와 Result
async fn async_with_error() -> Result<(), Box<dyn std::error::Error>> {
    let _content = tokio::fs::read_to_string("nonexistent.txt").await?;
    Ok(())
}
