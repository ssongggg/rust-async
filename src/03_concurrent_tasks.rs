// 03_concurrent_tasks.rs - 并发模型深入
//
// 本示例演示：
// 1. select! 宏的使用（竞争式并发）
// 2. 超时处理
// 3. 任务取消和清理
// 4. 并发模式的实际应用

use tokio::time::{sleep, Duration, timeout};
use tokio::select;

/// 模拟不同速度的异步任务
async fn fast_task() -> &'static str {
    sleep(Duration::from_secs(1)).await;
    "⚡ 快速任务完成"
}

async fn slow_task() -> &'static str {
    sleep(Duration::from_secs(3)).await;
    "🐌 慢速任务完成"
}

/// 演示 select! 宏 - 等待多个 Future，返回第一个完成的
async fn select_demo() {
    println!("\n=== 1. select! 宏（竞争式并发）===");
    println!("📝 select! 等待多个 Future，哪个先完成就处理哪个\n");
    
    let start = std::time::Instant::now();
    
    select! {
        result = fast_task() => {
            println!("{}", result);
            println!("   快速任务获胜！");
        }
        result = slow_task() => {
            println!("{}", result);
            println!("   慢速任务获胜！");
        }
    }
    
    println!("   ⏱️  耗时: {:.1} 秒", start.elapsed().as_secs_f64());
    println!("   📌 注意：另一个未完成的任务会被取消\n");
}

/// 演示超时处理
async fn timeout_demo() {
    println!("=== 2. 超时处理 ===");
    
    // 为慢速任务设置 2 秒超时
    println!("⏰ 为慢速任务（3秒）设置 2 秒超时...");
    match timeout(Duration::from_secs(2), slow_task()).await {
        Ok(result) => println!("✅ 任务完成: {}", result),
        Err(_) => println!("⏱️  任务超时！"),
    }
    
    println!();
    
    // 为快速任务设置 2 秒超时
    println!("⏰ 为快速任务（1秒）设置 2 秒超时...");
    match timeout(Duration::from_secs(2), fast_task()).await {
        Ok(result) => println!("✅ 任务完成: {}", result),
        Err(_) => println!("⏱️  任务超时！"),
    }
    
    println!();
}

/// 演示 select! 的多个分支和偏向
async fn select_multiple_branches() {
    println!("=== 3. select! 多分支处理 ===");
    
    let mut count = 0;
    
    loop {
        select! {
            _ = sleep(Duration::from_millis(100)) => {
                count += 1;
                println!("   ⏰ 定时器触发 (第 {} 次)", count);
                if count >= 3 {
                    println!("   🛑 达到 3 次，退出循环");
                    break;
                }
            }
            _ = async { sleep(Duration::from_millis(50)).await; } => {
                println!("   💤 短暂等待完成");
            }
        }
    }
    
    println!();
}

/// 演示并发限制 - 使用信号量
async fn concurrent_limit() {
    use tokio::sync::Semaphore;
    use std::sync::Arc;
    
    println!("=== 4. 并发限制（信号量）===");
    println!("📝 最多允许 2 个任务同时运行\n");
    
    // 创建一个允许 2 个并发访问的信号量
    let semaphore = Arc::new(Semaphore::new(3));
    let mut handles = vec![];
    
    // 启动 5 个任务
    for i in 1..=5 {
        let sem = semaphore.clone();
        let handle = tokio::spawn(async move {
            // 获取许可证
            let _permit = sem.acquire().await.unwrap();
            println!("🚀 任务 {} 开始执行", i);
            sleep(Duration::from_secs(1)).await;
            println!("✅ 任务 {} 完成", i);
            // permit 被 drop，释放许可证
        });
        handles.push(handle);
    }
    
    // 等待所有任务完成
    for handle in handles {
        let _ = handle.await;
    }
    
    println!();
}

/// 演示任务间通信 - 使用 oneshot channel
async fn oneshot_channel_demo() {
    use tokio::sync::oneshot;
    
    println!("=== 5. oneshot Channel（一次性通信）===");
    
    let (tx, rx) = oneshot::channel();
    
    // 生产者任务
    tokio::spawn(async move {
        println!("📤 生产者: 开始计算...");
        sleep(Duration::from_secs(1)).await;
        let result = 42;
        println!("📤 生产者: 发送结果 {}", result);
        let _ = tx.send(result);
    });
    
    // 消费者任务
    println!("📥 消费者: 等待结果...");
    match rx.await {
        Ok(value) => println!("📥 消费者: 收到结果 {}\n", value),
        Err(_) => println!("📥 消费者: 发送者已断开\n"),
    }
}

/// 演示取消安全性
async fn cancellation_safety() {
    println!("=== 6. 取消安全性 ===");
    println!("📝 演示在 select! 中任务可能被取消\n");
    
    let mut counter = 0;
    
    for round in 1..=3 {
        println!("🔄 回合 {}", round);
        
        select! {
            _ = async {
                counter += 1;
                println!("   计数器增加到: {}", counter);
                sleep(Duration::from_secs(2)).await;
                println!("   长任务完成");
            } => {}
            _ = sleep(Duration::from_millis(100)) => {
                println!("   ⏰ 超时触发，长任务被取消");
            }
        }
    }
    
    println!("   最终计数器值: {}", counter);
    println!("   📌 注意：每次 select! 都会重新开始未完成的 Future\n");
}

/// 演示 FuturesUnordered - 处理动态数量的任务
async fn futures_unordered_demo() {
    use futures::stream::{FuturesUnordered, StreamExt};
    
    println!("=== 7. FuturesUnordered（动态任务集合）===");
    println!("📝 按完成顺序处理多个 Future\n");
    
    let mut futures = FuturesUnordered::new();
    
    // 添加不同耗时的任务
    futures.push(async_task_with_delay("任务A", 2));
    futures.push(async_task_with_delay("任务B", 1));
    futures.push(async_task_with_delay("任务C", 3));
    
    // 按完成顺序处理结果
    while let Some(result) = futures.next().await {
        println!("✅ {}", result);
    }
    
    println!();
}

async fn async_task_with_delay(name: &str, seconds: u64) -> String {
    println!("🚀 {} 启动（{}秒）", name, seconds);
    sleep(Duration::from_secs(seconds)).await;
    format!("{} 完成！", name)
}

#[tokio::main]
async fn main() {
    println!("🎓 Rust 并发模型深入教程\n");
    println!("💡 Rust 提供多种并发模式来处理不同场景");
    
    select_demo().await;
    timeout_demo().await;
    select_multiple_branches().await;
    concurrent_limit().await;
    oneshot_channel_demo().await;
    cancellation_safety().await;
    futures_unordered_demo().await;
    
    println!("🎉 教程完成！\n");
    println!("💡 关键要点：");
    println!("   • select! 用于竞争式并发，处理第一个完成的 Future");
    println!("   • timeout 为异步操作设置超时限制");
    println!("   • Semaphore 控制并发数量");
    println!("   • oneshot channel 用于一次性通信");
    println!("   • select! 中未完成的分支会被取消");
    println!("   • FuturesUnordered 按完成顺序处理动态任务集合");
}

