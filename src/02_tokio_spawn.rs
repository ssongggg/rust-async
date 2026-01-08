// 02_tokio_spawn.rs - Tokio 任务生成与并发
//
// 本示例演示：
// 1. tokio::spawn 创建新任务
// 2. JoinHandle 的使用
// 3. 任务的并发执行
// 4. 任务之间的独立性

use tokio::time::{sleep, Duration};

/// 模拟一个耗时的异步任务
async fn async_task(id: u32, duration: u64) -> String {
    println!("🚀 任务 {} 启动（耗时 {} 秒）", id, duration);
    sleep(Duration::from_secs(duration)).await;
    let result = format!("任务 {} 完成！", id);
    println!("✅ {}", result);
    result
}

/// 演示基本的 spawn 用法
async fn basic_spawn() {
    println!("\n=== 1. 基本的 tokio::spawn ===");
    
    // spawn 创建一个新的异步任务，立即返回 JoinHandle
    let handle = tokio::spawn(async {
        println!("👋 我在一个独立的任务中运行");
        sleep(Duration::from_secs(1)).await;
        println!("✨ 任务执行完毕");
        42 // 返回值
    });
    
    println!("📝 主任务继续执行，不会等待 spawn 的任务");
    
    // 使用 JoinHandle 等待任务完成并获取结果
    match handle.await {
        Ok(result) => println!("🎯 任务返回值: {}\n", result),
        Err(e) => println!("❌ 任务失败: {:?}\n", e),
    }
}

/// 演示多个并发任务
async fn multiple_spawns() {
    println!("=== 2. 多个并发任务 ===");
    
    let start = std::time::Instant::now();
    
    // 创建多个任务，它们会并发执行
    let handle1 = tokio::spawn(async_task(1, 2));
    let handle2 = tokio::spawn(async_task(2, 1));
    let handle3 = tokio::spawn(async_task(3, 3));
    
    println!("📝 所有任务已启动，现在等待它们完成...\n");
    
    // 等待所有任务完成
    let (result1, result2, result3) = tokio::join!(handle1, handle2, handle3);
    
    println!("\n📊 结果汇总：");
    println!("   {}", result1.unwrap());
    println!("   {}", result2.unwrap());
    println!("   {}", result3.unwrap());
    println!("   ⏱️  总耗时: {:.1} 秒（并发执行）\n", start.elapsed().as_secs_f64());
}

/// 演示任务中的错误处理
async fn error_handling() {
    println!("=== 3. 任务错误处理 ===");
    
    let handle = tokio::spawn(async {
        sleep(Duration::from_millis(100)).await;
        // 模拟一个可能失败的操作
        if true {
            return Err("模拟的错误");
        }
        Ok("成功")
    });
    
    match handle.await {
        Ok(Ok(value)) => println!("✅ 任务成功: {}", value),
        Ok(Err(e)) => println!("⚠️  任务返回错误: {}", e),
        Err(e) => println!("❌ 任务 panic: {:?}", e),
    }
    println!();
}

/// 演示 spawn 与普通 await 的区别
async fn spawn_vs_await() {
    println!("=== 4. spawn vs await 对比 ===");
    
    println!("📌 使用 await（串行）：");
    let start = std::time::Instant::now();
    async_task(101, 1).await;
    async_task(102, 1).await;
    println!("   ⏱️  耗时: {:.1} 秒\n", start.elapsed().as_secs_f64());
    
    println!("📌 使用 spawn（并行）：");
    let start = std::time::Instant::now();
    let h1 = tokio::spawn(async_task(201, 1));
    let h2 = tokio::spawn(async_task(202, 1));
    let _ = tokio::join!(h1, h2);
    println!("   ⏱️  耗时: {:.1} 秒\n", start.elapsed().as_secs_f64());
}

/// 演示任务取消
async fn task_cancellation() {
    println!("=== 5. 任务取消 ===");
    
    let handle = tokio::spawn(async {
        for i in 1..=10 {
            println!("   🔄 计数: {}", i);
            sleep(Duration::from_millis(200)).await;
        }
        "完成"
    });
    
    // 让任务运行一段时间
    sleep(Duration::from_millis(500)).await;
    
    // 取消任务（abort）
    handle.abort();
    println!("🛑 任务已被取消");
    
    match handle.await {
        Ok(result) => println!("   结果: {}", result),
        Err(e) if e.is_cancelled() => println!("   ✅ 确认任务已取消"),
        Err(e) => println!("   ❌ 其他错误: {:?}", e),
    }
    println!();
}

/// 演示使用 spawn_blocking 处理 CPU 密集型任务
async fn blocking_task() {
    println!("=== 6. 阻塞任务 (spawn_blocking) ===");
    
    println!("🔢 执行 CPU 密集型计算...");
    
    // spawn_blocking 用于运行会阻塞的同步代码
    let handle = tokio::task::spawn_blocking(|| {
        // 模拟 CPU 密集型计算
        let mut sum = 0u64;
        for i in 0..100_000_000 {
            sum += i;
        }
        sum
    });
    
    println!("📝 主任务可以继续执行其他异步操作");
    
    let result = handle.await.unwrap();
    println!("✅ 计算完成，结果: {}\n", result);
}

#[tokio::main]
async fn main() {
    println!("🎓 Tokio Spawn 与并发任务教程\n");
    println!("💡 tokio::spawn 创建独立的异步任务，类似于操作系统线程，但更轻量");
    
    basic_spawn().await;
    multiple_spawns().await;
    error_handling().await;
    spawn_vs_await().await;
    task_cancellation().await;
    blocking_task().await;
    
    println!("🎉 教程完成！\n");
    println!("💡 关键要点：");
    println!("   • tokio::spawn 创建新的异步任务，返回 JoinHandle");
    println!("   • spawn 的任务在后台并发执行");
    println!("   • 使用 JoinHandle.await 等待任务完成并获取结果");
    println!("   • JoinHandle.abort() 可以取消任务");
    println!("   • spawn_blocking 用于执行阻塞的同步代码");
    println!("   • spawn 的任务必须是 'static 生命周期");
}

