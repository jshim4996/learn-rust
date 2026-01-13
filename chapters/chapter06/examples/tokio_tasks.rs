// STEP 6-5 ~ 6-7: tokio 태스크와 동시성
// 실행: cargo run --example tokio_tasks
// Cargo.toml에 tokio = { version = "1", features = ["full"] } 필요

use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    // ========================================
    // 6-5. join! - 모두 완료 대기
    // ========================================

    println!("=== join! ===");

    let start = std::time::Instant::now();

    let (a, b, c) = tokio::join!(
        fetch_data("API-1", 1),
        fetch_data("API-2", 2),
        fetch_data("API-3", 1)
    );

    println!("All done in {:?}", start.elapsed());
    println!("Results: {}, {}, {}", a, b, c);

    // ========================================
    // 6-6. select! - 첫 번째 완료 시 반환
    // ========================================

    println!("\n=== select! ===");

    tokio::select! {
        result = fetch_data("Fast", 1) => {
            println!("Fast finished first: {}", result);
        }
        result = fetch_data("Slow", 3) => {
            println!("Slow finished first: {}", result);
        }
    }

    // select! 고급 사용
    println!("\n=== select! with timeout ===");

    let timeout_duration = Duration::from_secs(2);

    tokio::select! {
        result = fetch_data("Task", 5) => {
            println!("Task completed: {}", result);
        }
        _ = sleep(timeout_duration) => {
            println!("Timeout! Task took too long.");
        }
    }

    // ========================================
    // 6-7. spawn - 백그라운드 태스크
    // ========================================

    println!("\n=== spawn ===");

    // 백그라운드 태스크 생성
    let handle1 = tokio::spawn(async {
        println!("  Task 1 starting...");
        sleep(Duration::from_secs(2)).await;
        println!("  Task 1 done!");
        "Result from task 1"
    });

    let handle2 = tokio::spawn(async {
        println!("  Task 2 starting...");
        sleep(Duration::from_secs(1)).await;
        println!("  Task 2 done!");
        "Result from task 2"
    });

    println!("Main continues while tasks run...");

    // 결과 수집
    let result1 = handle1.await.unwrap();
    let result2 = handle2.await.unwrap();

    println!("Task 1 returned: {}", result1);
    println!("Task 2 returned: {}", result2);

    // 여러 태스크 spawn
    println!("\n=== Multiple spawns ===");

    let mut handles = vec![];

    for i in 0..5 {
        let handle = tokio::spawn(async move {
            sleep(Duration::from_millis(100 * i)).await;
            i * 10
        });
        handles.push(handle);
    }

    // 모든 태스크 완료 대기
    let mut results = vec![];
    for handle in handles {
        results.push(handle.await.unwrap());
    }

    println!("All spawn results: {:?}", results);

    // spawn 에러 처리
    println!("\n=== spawn with error ===");

    let handle = tokio::spawn(async {
        if true {
            return Err("Something went wrong");
        }
        Ok("Success")
    });

    match handle.await {
        Ok(Ok(value)) => println!("Success: {}", value),
        Ok(Err(e)) => println!("Task error: {}", e),
        Err(e) => println!("Join error: {}", e),
    }
}

async fn fetch_data(name: &str, delay_secs: u64) -> String {
    println!("  {} fetching...", name);
    sleep(Duration::from_secs(delay_secs)).await;
    println!("  {} completed", name);
    format!("Data from {}", name)
}
