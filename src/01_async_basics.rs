// 01_async_basics.rs - Async/Await 基础概念
// 
// 本示例演示：
// 1. async 函数的基本语法
// 2. await 关键字的使用
// 3. Future 的基本概念
// 4. Tokio runtime 的作用

use tokio::time::{sleep, Duration};

/// 一个简单的异步函数
/// async 关键字将函数转换为返回 Future 的函数
async fn say_hello() {
    println!("你好！我是一个异步函数");
}

/// 带有 await 的异步函数
/// await 会暂停当前函数的执行，直到 Future 完成
async fn say_after_delay(message: &str, seconds: u64) {
    println!("⏰ 等待 {} 秒...", seconds);
    sleep(Duration::from_secs(seconds)).await; // .await 暂停执行
    println!("⭐ {}", message);
}

/// 异步函数可以返回值
async fn calculate_async(x: i32, y: i32) -> i32 {
    println!("🔢 开始异步计算: {} + {}", x, y);
    sleep(Duration::from_millis(500)).await; // 模拟耗时操作
    let result = x + y;
    println!("✅ 计算完成: {}", result);
    result
}

/// 组合多个异步操作
async fn sequential_operations() {
    println!("\n=== 顺序执行异步操作 ===");
    
    // 这些操作会依次执行（串行）
    say_after_delay("第一个任务完成", 1).await;
    say_after_delay("第二个任务完成", 1).await;
    say_after_delay("第三个任务完成", 1).await;
    
    println!("📝 总耗时约 3 秒（串行执行）\n");
}

/// 并发执行多个异步操作
async fn concurrent_operations() {
    println!("=== 并发执行异步操作 ===");
    
    // 使用 tokio::join! 宏并发执行多个 Future
    // 所有任务会同时开始，等待所有完成
    let start = std::time::Instant::now();
    
    tokio::join!(
        say_after_delay("并发任务 1 完成", 1),
        say_after_delay("并发任务 2 完成", 1),
        say_after_delay("并发任务 3 完成", 1),
    );
    
    let elapsed = start.elapsed();
    println!("📝 总耗时约 {:.1} 秒（并发执行）\n", elapsed.as_secs_f64());
}

/// 演示 Future 是惰性的（需要被 await 才会执行）
async fn lazy_futures() {
    println!("=== Future 的惰性特性 ===");
    
    // 创建 Future 但不 await - 这不会执行
    let future = say_hello();
    println!("📦 Future 已创建，但还没有执行");
    
    sleep(Duration::from_secs(1)).await;
    
    // 现在 await，Future 才真正执行
    println!("🚀 现在执行 Future：");
    future.await;
    println!();
}

#[tokio::main]
async fn main() {
    println!("🎓 欢迎来到 Rust Async/Await 基础教程！\n");
    
    // 1. 基础异步函数调用
    println!("=== 1. 基础异步函数 ===");
    say_hello().await;
    println!();
    
    // 2. 带延迟的异步函数
    println!("=== 2. 异步等待 ===");
    say_after_delay("延迟后的消息", 1).await;
    println!();
    
    // 3. 异步函数返回值
    println!("=== 3. 异步函数返回值 ===");
    let result = calculate_async(10, 20).await;
    println!("🎯 main 函数收到结果: {}\n", result);
    
    // 4. 顺序执行 vs 并发执行
    sequential_operations().await;
    concurrent_operations().await;
    
    // 5. Future 的惰性
    lazy_futures().await;
    
    println!("🎉 教程完成！\n");
    println!("💡 关键要点：");
    println!("   • async 关键字创建异步函数，返回 Future");
    println!("   • await 关键字等待 Future 完成");
    println!("   • Future 是惰性的，必须被 await 才会执行");
    println!("   • #[tokio::main] 宏创建异步运行时");
    println!("   • tokio::join! 可以并发执行多个 Future");
}

