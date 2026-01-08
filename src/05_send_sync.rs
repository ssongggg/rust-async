// 05_send_sync.rs - Send 和 Sync trait 深入理解
//
// 本示例演示：
// 1. Send trait 的含义和作用
// 2. Sync trait 的含义和作用
// 3. !Send 和 !Sync 类型
// 4. 在并发环境中的实际应用

use std::rc::Rc;
use std::sync::{Arc, Mutex};
use tokio::time::{sleep, Duration};

/// === 核心概念 ===
///
/// Send: 类型可以安全地在线程间转移所有权
/// - 实现 Send 的类型可以被移动到另一个线程
/// - 大部分类型都是 Send
/// 
/// Sync: 类型可以安全地在线程间共享引用
/// - 如果 &T 是 Send，那么 T 就是 Sync
/// - 实现 Sync 的类型可以被多个线程同时访问

/// 演示 Send - 可以在线程间转移
async fn send_demo() {
    println!("\n=== 1. Send Trait ===");
    println!("📝 Send 表示类型可以安全地在线程间转移所有权\n");
    
    // String 是 Send 的，可以在任务间转移
    let message = String::from("这是一个 Send 类型");
    
    let handle = tokio::spawn(async move {
        // message 的所有权被转移到这个任务
        println!("✅ 在新任务中: {}", message);
        message.len()
    });
    
    let len = handle.await.unwrap();
    println!("   字符串长度: {}\n", len);
    
    println!("💡 常见的 Send 类型：");
    println!("   • 基本类型: i32, f64, bool, etc.");
    println!("   • String, Vec<T>, Box<T> (如果 T: Send)");
    println!("   • Arc<T>, Mutex<T> (如果 T: Send)");
}

/// 演示 !Send - 不能在线程间转移的类型
async fn not_send_demo() {
    println!("=== 2. !Send 类型 ===");
    println!("📝 某些类型不是 Send，不能在线程间转移\n");
    
    // Rc 不是 Send 的（引用计数不是原子的）
    let rc = Rc::new(42);
    println!("✅ Rc 在本地线程使用没问题: {}", rc);
    
    // 下面的代码会编译错误！
    // let handle = tokio::spawn(async move {
    //     println!("{}", rc); // ❌ 错误：Rc 不是 Send
    // });
    
    println!("\n💡 常见的 !Send 类型：");
    println!("   • Rc<T> - 非原子引用计数");
    println!("   • *const T, *mut T - 裸指针");
    println!("   • RefCell<T> - 内部可变性，非线程安全");
    
    // 正确的做法：使用 Arc（原子引用计数）
    let arc = Arc::new(42);
    let arc_clone = arc.clone();
    
    let handle = tokio::spawn(async move {
        println!("\n✅ Arc 是 Send 的，可以跨任务: {}", arc_clone);
    });
    
    handle.await.unwrap();
    println!();
}

/// 演示 Sync - 可以在线程间共享引用
async fn sync_demo() {
    println!("=== 3. Sync Trait ===");
    println!("📝 Sync 表示类型可以安全地在线程间共享引用\n");
    
    // 使用 Arc 共享数据（Arc<T> 是 Send + Sync，如果 T: Send + Sync）
    let shared_data = Arc::new(vec![1, 2, 3, 4, 5]);
    
    let mut handles = vec![];
    
    for i in 0..3 {
        let data = shared_data.clone();
        let handle = tokio::spawn(async move {
            println!("   任务 {} 读取共享数据: {:?}", i, data);
            sleep(Duration::from_millis(100)).await;
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.await.unwrap();
    }
    
    println!("\n💡 T 是 Sync 当且仅当 &T 是 Send");
    println!("   • 如果多个线程可以安全地持有 &T，T 就是 Sync\n");
}

/// 演示 Mutex - 提供内部可变性和同步
async fn mutex_demo() {
    println!("=== 4. Mutex<T> 提供线程安全的内部可变性 ===");
    
    // Arc<Mutex<T>> 是在多任务间共享可变数据的标准模式
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    
    println!("📝 10 个任务并发地增加计数器\n");
    
    for i in 0..10 {
        let counter = counter.clone();
        let handle = tokio::spawn(async move {
            // 正确做法：在 await 前释放锁
            {
                let mut num = counter.lock().unwrap();
                *num += 1;
                println!("   任务 {} 增加计数器到: {}", i, *num);
            } // 锁在这里释放
            sleep(Duration::from_millis(10)).await;
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.await.unwrap();
    }
    
    println!("\n✅ 最终计数器值: {}\n", *counter.lock().unwrap());
}

/// 演示 tokio::sync::Mutex - 异步友好的 Mutex
async fn async_mutex_demo() {
    use tokio::sync::Mutex as AsyncMutex;
    
    println!("=== 5. tokio::sync::Mutex（异步 Mutex）===");
    println!("📝 与 std::sync::Mutex 的区别：可以在 .await 点持有锁\n");
    
    let data = Arc::new(AsyncMutex::new(Vec::new()));
    let mut handles = vec![];
    
    for i in 0..5 {
        let data = data.clone();
        let handle = tokio::spawn(async move {
            let mut vec = data.lock().await; // 异步获取锁
            vec.push(i);
            println!("   任务 {} 添加数据", i);
            
            // 可以在持有锁的情况下 await
            sleep(Duration::from_millis(100)).await;
            
            println!("   任务 {} 释放锁", i);
            // 锁在这里自动释放
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.await.unwrap();
    }
    
    let vec = data.lock().await;
    println!("\n✅ 最终数据: {:?}\n", *vec);
}

/// 演示 RwLock - 读写锁
async fn rwlock_demo() {
    use tokio::sync::RwLock;
    
    println!("=== 6. RwLock（读写锁）===");
    println!("📝 允许多个读者或一个写者\n");
    
    let data = Arc::new(RwLock::new(0));
    let mut handles = vec![];
    
    // 启动多个读任务
    for i in 0..3 {
        let data = data.clone();
        let handle = tokio::spawn(async move {
            let value = data.read().await;
            println!("   读任务 {} 读取值: {}", i, *value);
            sleep(Duration::from_millis(100)).await;
        });
        handles.push(handle);
    }
    
    // 启动一个写任务
    let data_clone = data.clone();
    handles.push(tokio::spawn(async move {
        let mut value = data_clone.write().await;
        *value = 42;
        println!("   写任务修改值为: {}", *value);
    }));
    
    for handle in handles {
        handle.await.unwrap();
    }
    
    println!("\n✅ 最终值: {}\n", *data.read().await);
}

/// 自定义类型的 Send/Sync
struct MyStruct {
    data: Arc<Mutex<i32>>,
}

// MyStruct 自动实现 Send 和 Sync，因为它的所有字段都是 Send + Sync

async fn custom_type_demo() {
    println!("=== 7. 自定义类型的 Send/Sync ===");
    println!("📝 如果结构体的所有字段都是 Send/Sync，结构体自动是 Send/Sync\n");
    
    let my_struct = MyStruct {
        data: Arc::new(Mutex::new(100)),
    };
    
    // 可以将 MyStruct 发送到另一个任务
    let handle = tokio::spawn(async move {
        let value = my_struct.data.lock().unwrap();
        println!("   自定义类型中的数据: {}", *value);
    });
    
    handle.await.unwrap();
    
    println!("\n💡 编译器会自动分析：");
    println!("   • 如果所有字段都是 Send，类型是 Send");
    println!("   • 如果所有字段都是 Sync，类型是 Sync");
    println!("   • 可以使用 unsafe impl 手动实现（需要保证安全性）\n");
}

/// 演示常见错误和解决方案
async fn common_mistakes() {
    println!("=== 8. 常见错误和解决方案 ===\n");
    
    println!("❌ 错误 1：在 spawn 中使用 Rc");
    println!("   解决方案：使用 Arc 代替 Rc\n");
    
    println!("❌ 错误 2：在 spawn 中使用 RefCell");
    println!("   解决方案：使用 Mutex 或 RwLock\n");
    
    println!("❌ 错误 3：在 .await 点持有 std::sync::Mutex");
    println!("   解决方案：使用 tokio::sync::Mutex 或缩小锁的作用域\n");
    
    println!("✅ 示例：正确的模式");
    
    // 正确：使用 Arc + Mutex
    let data = Arc::new(Mutex::new(vec![1, 2, 3]));
    let data_clone = data.clone();
    
    tokio::spawn(async move {
        {
            let mut vec = data_clone.lock().unwrap();
            vec.push(4);
            // 在 await 前释放锁
        }
        sleep(Duration::from_millis(100)).await;
    }).await.unwrap();
    
    println!("   数据: {:?}\n", data.lock().unwrap());
}

#[tokio::main]
async fn main() {
    println!("🎓 Send 和 Sync Trait 深入理解教程\n");
    println!("💡 理解 Rust 的线程安全保证");
    
    send_demo().await;
    not_send_demo().await;
    sync_demo().await;
    mutex_demo().await;
    async_mutex_demo().await;
    rwlock_demo().await;
    custom_type_demo().await;
    common_mistakes().await;
    
    println!("🎉 教程完成！\n");
    println!("💡 关键要点：");
    println!("   • Send: 类型可以安全地在线程间转移所有权");
    println!("   • Sync: 类型可以安全地在线程间共享引用（&T 是 Send）");
    println!("   • Rc/RefCell 不是 Send，不能在任务间转移");
    println!("   • Arc/Mutex 是 Send + Sync，可以在任务间共享");
    println!("   • tokio::spawn 要求 Future 是 Send");
    println!("   • 使用 Arc<Mutex<T>> 或 Arc<RwLock<T>> 共享可变数据");
    println!("   • tokio::sync::Mutex 可以在 .await 点持有锁");
}

