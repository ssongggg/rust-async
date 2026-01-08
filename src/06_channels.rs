// 06_channels.rs - Channel 通信模式
//
// 本示例演示：
// 1. mpsc channel（多生产者单消费者）
// 2. oneshot channel（一次性通信）
// 3. broadcast channel（广播）
// 4. watch channel（状态共享）

use tokio::sync::{mpsc, oneshot, broadcast, watch};
use tokio::time::{sleep, Duration};

/// === 1. MPSC Channel - 多生产者单消费者 ===
async fn mpsc_demo() {
    println!("\n=== 1. MPSC Channel（多生产者单消费者）===");
    println!("📝 多个发送者可以向一个接收者发送消息\n");
    
    // 创建一个容量为 10 的 channel
    let (tx, mut rx) = mpsc::channel::<String>(10);
    
    // 生产者 1
    let tx1 = tx.clone();
    tokio::spawn(async move {
        for i in 1..=3 {
            let msg = format!("生产者1发送: 消息{}", i);
            tx1.send(msg).await.unwrap();
            println!("📤 生产者1发送消息{}", i);
            sleep(Duration::from_millis(100)).await;
        }
    });
    
    // 生产者 2
    let tx2 = tx.clone();
    tokio::spawn(async move {
        for i in 1..=3 {
            let msg = format!("生产者2发送: 消息{}", i);
            tx2.send(msg).await.unwrap();
            println!("📤 生产者2发送消息{}", i);
            sleep(Duration::from_millis(150)).await;
        }
    });
    
    // 释放原始的 tx，这样当所有克隆都被 drop 时，channel 会关闭
    drop(tx);
    
    // 消费者
    println!("📥 消费者开始接收：\n");
    while let Some(msg) = rx.recv().await {
        println!("   📥 收到: {}", msg);
    }
    
    println!("\n✅ 所有生产者完成，channel 关闭\n");
}

/// === 2. Bounded vs Unbounded ===
async fn bounded_unbounded_demo() {
    println!("=== 2. 有界 vs 无界 Channel ===\n");
    
    // 有界 channel - 有容量限制
    println!("📌 有界 Channel（容量=2）:");
    let (tx, mut rx) = mpsc::channel::<i32>(2);
    
    tokio::spawn(async move {
        for i in 1..=5 {
            println!("   发送 {}", i);
            tx.send(i).await.unwrap(); // 如果满了会等待
            println!("   发送 {} 成功", i);
        }
    });
    
    sleep(Duration::from_secs(1)).await;
    println!("   开始接收...");
    
    while let Some(msg) = rx.recv().await {
        println!("   收到 {}", msg);
        sleep(Duration::from_millis(200)).await;
    }
    
    println!("\n📌 无界 Channel:");
    let (tx, mut rx) = mpsc::unbounded_channel::<i32>();
    
    tokio::spawn(async move {
        for i in 1..=5 {
            println!("   发送 {}", i);
            tx.send(i).unwrap(); // 立即返回，不会阻塞
        }
    });
    
    sleep(Duration::from_millis(500)).await;
    
    while let Some(msg) = rx.recv().await {
        println!("   收到 {}", msg);
    }
    
    println!();
}

/// === 3. Oneshot Channel - 一次性通信 ===
async fn oneshot_demo() {
    println!("=== 3. Oneshot Channel（一次性通信）===");
    println!("📝 用于发送单个值，常用于请求-响应模式\n");
    
    let (tx, rx) = oneshot::channel::<String>();
    
    // 模拟异步计算
    tokio::spawn(async move {
        println!("🔢 开始复杂计算...");
        sleep(Duration::from_secs(1)).await;
        let result = "计算结果：42".to_string();
        println!("✅ 计算完成");
        tx.send(result).unwrap();
    });
    
    println!("⏳ 等待结果...");
    match rx.await {
        Ok(result) => println!("📥 收到: {}\n", result),
        Err(_) => println!("❌ 发送者被 drop\n"),
    }
}

/// === 4. Broadcast Channel - 广播 ===
async fn broadcast_demo() {
    println!("=== 4. Broadcast Channel（广播）===");
    println!("📝 一个发送者，多个接收者都能收到消息\n");
    
    let (tx, _rx) = broadcast::channel::<String>(10);
    
    // 创建 3 个订阅者
    let mut rx1 = tx.subscribe();
    let mut rx2 = tx.subscribe();
    let mut rx3 = tx.subscribe();
    
    // 订阅者 1
    tokio::spawn(async move {
        while let Ok(msg) = rx1.recv().await {
            println!("   📻 订阅者1收到: {}", msg);
        }
    });
    
    // 订阅者 2
    tokio::spawn(async move {
        while let Ok(msg) = rx2.recv().await {
            println!("   📻 订阅者2收到: {}", msg);
        }
    });
    
    // 订阅者 3
    tokio::spawn(async move {
        while let Ok(msg) = rx3.recv().await {
            println!("   📻 订阅者3收到: {}", msg);
        }
    });
    
    sleep(Duration::from_millis(100)).await;
    
    // 广播消息
    println!("📡 广播消息...\n");
    for i in 1..=3 {
        let msg = format!("广播消息 {}", i);
        tx.send(msg).unwrap();
        sleep(Duration::from_millis(200)).await;
    }
    
    sleep(Duration::from_millis(500)).await;
    println!();
}

/// === 5. Watch Channel - 状态共享 ===
async fn watch_demo() {
    println!("=== 5. Watch Channel（状态共享）===");
    println!("📝 用于共享状态，接收者总能看到最新值\n");
    
    let (tx, mut rx) = watch::channel("初始状态");
    
    // 观察者 1
    let mut rx1 = rx.clone();
    tokio::spawn(async move {
        loop {
            rx1.changed().await.unwrap();
            let value = rx1.borrow_and_update();
            println!("   👀 观察者1看到状态变化: {}", *value);
        }
    });
    
    // 观察者 2
    tokio::spawn(async move {
        loop {
            rx.changed().await.unwrap();
            let value = rx.borrow_and_update();
            println!("   👀 观察者2看到状态变化: {}", *value);
        }
    });
    
    sleep(Duration::from_millis(100)).await;
    
    // 更新状态
    println!("🔄 更新状态...\n");
    for state in &["状态A", "状态B", "状态C"] {
        tx.send(*state).unwrap();
        sleep(Duration::from_millis(500)).await;
    }
    
    println!();
}

/// === 6. 实战示例：工作队列 ===
async fn work_queue_demo() {
    println!("=== 6. 实战：工作队列 ===");
    println!("📝 多个工作者从队列中获取任务并处理\n");
    
    let (tx, rx) = mpsc::channel::<i32>(10);
    let rx = std::sync::Arc::new(tokio::sync::Mutex::new(rx));
    
    // 启动 3 个工作者
    let mut workers = vec![];
    for id in 1..=3 {
        let rx = rx.clone();
        let worker = tokio::spawn(async move {
            loop {
                let task = {
                    let mut rx = rx.lock().await;
                    rx.recv().await
                };
                
                match task {
                    Some(task) => {
                        println!("   👷 工作者{} 处理任务{}", id, task);
                        sleep(Duration::from_millis(500)).await;
                        println!("   ✅ 工作者{} 完成任务{}", id, task);
                    }
                    None => break,
                }
            }
        });
        workers.push(worker);
    }
    
    // 发送任务
    println!("📤 发送 6 个任务...\n");
    for task in 1..=6 {
        tx.send(task).await.unwrap();
    }
    
    drop(tx); // 关闭队列
    
    // 等待所有工作者完成
    for worker in workers {
        worker.await.unwrap();
    }
    
    println!("\n✅ 所有任务完成\n");
}

/// === 7. 选择最合适的 Channel ===
async fn channel_selection_guide() {
    println!("=== 7. 如何选择 Channel 类型 ===\n");
    
    println!("📋 Channel 选择指南：\n");
    
    println!("🔹 mpsc::channel");
    println!("   用途：多生产者→单消费者");
    println!("   特点：有界，发送满时会等待");
    println!("   场景：工作队列、事件处理\n");
    
    println!("🔹 mpsc::unbounded_channel");
    println!("   用途：多生产者→单消费者");
    println!("   特点：无界，永不阻塞");
    println!("   场景：快速发送，但要注意内存\n");
    
    println!("🔹 oneshot::channel");
    println!("   用途：单个值的一次性传递");
    println!("   特点：只能发送一次");
    println!("   场景：请求-响应、Future 结果传递\n");
    
    println!("🔹 broadcast::channel");
    println!("   用途：一对多广播");
    println!("   特点：所有订阅者都收到相同消息");
    println!("   场景：事件通知、消息分发\n");
    
    println!("🔹 watch::channel");
    println!("   用途：状态共享");
    println!("   特点：接收者只关心最新值");
    println!("   场景：配置更新、状态监控\n");
}

#[tokio::main]
async fn main() {
    println!("🎓 Channel 通信模式教程\n");
    println!("💡 Channel 是任务间通信的主要方式");
    
    mpsc_demo().await;
    bounded_unbounded_demo().await;
    oneshot_demo().await;
    broadcast_demo().await;
    watch_demo().await;
    work_queue_demo().await;
    channel_selection_guide().await;
    
    println!("🎉 教程完成！\n");
    println!("💡 关键要点：");
    println!("   • mpsc: 多生产者单消费者，最常用");
    println!("   • oneshot: 一次性通信，用于单个值传递");
    println!("   • broadcast: 广播给所有订阅者");
    println!("   • watch: 状态共享，接收者看到最新值");
    println!("   • 有界 channel 有背压控制");
    println!("   • 无界 channel 需要注意内存使用");
}

