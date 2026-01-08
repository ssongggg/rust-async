// 04_futures_pin.rs - Futures 和 Pin 深入理解
//
// 本示例演示：
// 1. Future trait 的基本概念
// 2. 手动实现 Future
// 3. Pin 和 Unpin 的作用
// 4. 自引用结构体的问题

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};
use tokio::time::sleep;

/// === 1. 理解 Future Trait ===
/// 
/// Future 的定义（简化版）：
/// ```
/// trait Future {
///     type Output;
///     fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output>;
/// }
/// ```

/// 一个简单的自定义 Future - 延迟完成
struct DelayFuture {
    when: Instant,
}

impl DelayFuture {
    fn new(duration: Duration) -> Self {
        DelayFuture {
            when: Instant::now() + duration,
        }
    }
}

impl Future for DelayFuture {
    type Output = String;
    
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if Instant::now() >= self.when {
            // 时间到了，Future 完成
            Poll::Ready("⏰ 延迟完成！".to_string())
        } else {
            // 还没到时间，需要稍后重新 poll
            // 在实际实现中，应该注册 waker 来通知运行时
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

/// 演示自定义 Future
async fn custom_future_demo() {
    println!("\n=== 1. 自定义 Future ===");
    println!("📝 手动实现 Future trait\n");
    
    let future = DelayFuture::new(Duration::from_secs(1));
    let result = future.await;
    println!("{}\n", result);
}

/// === 2. 理解 Pin ===
/// 
/// Pin 的作用：保证被 pin 的值不会在内存中移动
/// 这对于自引用结构体非常重要

/// 一个自引用结构体的例子（仅用于概念演示）
#[allow(dead_code)]
struct SelfReferential {
    data: String,
    // 注意：这是一个指向 data 的指针（实际中很危险！）
    // 如果结构体移动，指针会失效
    pointer: *const String,
}

#[allow(dead_code)]
impl SelfReferential {
    fn new(text: String) -> Self {
        SelfReferential {
            data: text,
            pointer: std::ptr::null(),
        }
    }
    
    fn init(self: Pin<&mut Self>) {
        let self_ptr: *const String = &self.data;
        // 安全地设置自引用指针
        unsafe {
            let mut_ref = Pin::get_unchecked_mut(self);
            mut_ref.pointer = self_ptr;
        }
    }
    
    fn get_data(&self) -> &str {
        &self.data
    }
}

/// 演示 Pin 的必要性
async fn pin_demo() {
    println!("=== 2. Pin 的作用 ===");
    println!("📝 Pin 防止值在内存中移动，保护自引用结构体\n");
    
    // 大多数类型实现了 Unpin，可以安全移动
    let x = String::from("可以移动");
    let pinned = Box::pin(x);
    println!("✅ Unpin 类型可以安全地 pin: {}", pinned);
    
    println!("\n💡 关键概念：");
    println!("   • Pin<P> 是一个智能指针，保证内部值不会移动");
    println!("   • Unpin trait：表示类型可以安全移动（大部分类型）");
    println!("   • !Unpin：需要 Pin 保护的类型（如自引用结构体）");
    println!("   • Future 需要 Pin 因为 async 可能产生自引用\n");
}

/// === 3. 组合 Future ===

/// 手动实现一个组合 Future
struct JoinFuture<F1, F2> {
    future1: Option<F1>,
    future2: Option<F2>,
}

impl<F1, F2> JoinFuture<F1, F2> {
    fn new(f1: F1, f2: F2) -> Self {
        JoinFuture {
            future1: Some(f1),
            future2: Some(f2),
        }
    }
}

impl<F1, F2> Future for JoinFuture<F1, F2>
where
    F1: Future + Unpin,
    F2: Future + Unpin,
{
    type Output = (F1::Output, F2::Output);
    
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // 尝试 poll 第一个 future
        let result1 = if let Some(ref mut f1) = self.future1 {
            match Pin::new(f1).poll(cx) {
                Poll::Ready(val) => {
                    self.future1 = None;
                    Some(val)
                }
                Poll::Pending => None,
            }
        } else {
            None
        };
        
        // 尝试 poll 第二个 future
        let result2 = if let Some(ref mut f2) = self.future2 {
            match Pin::new(f2).poll(cx) {
                Poll::Ready(val) => {
                    self.future2 = None;
                    Some(val)
                }
                Poll::Pending => None,
            }
        } else {
            None
        };
        
        // 如果两个都完成了，返回结果
        if let (None, None) = (&self.future1, &self.future2) {
            Poll::Ready((result1.unwrap(), result2.unwrap()))
        } else {
            Poll::Pending
        }
    }
}

/// 演示组合 Future
async fn combined_future_demo() {
    println!("=== 3. 组合 Future ===");
    println!("📝 手动实现类似 join! 的功能\n");
    
    let future1 = Box::pin(async {
        sleep(Duration::from_secs(1)).await;
        "Future 1 完成"
    });
    
    let future2 = Box::pin(async {
        sleep(Duration::from_secs(1)).await;
        "Future 2 完成"
    });
    
    let combined = JoinFuture::new(future1, future2);
    let (r1, r2) = combined.await;
    
    println!("✅ {}", r1);
    println!("✅ {}\n", r2);
}

/// === 4. Stream - 异步迭代器 ===

/// Stream 类似于异步版本的 Iterator
/// trait Stream {
///     type Item;
///     fn poll_next(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>>;
/// }

use futures::stream::{self, StreamExt};

async fn stream_demo() {
    println!("=== 4. Stream（异步迭代器）===");
    println!("📝 Stream 是 Future 的集合版本\n");
    
    // 创建一个 Stream
    let mut stream = stream::iter(vec![1, 2, 3, 4, 5]);
    
    println!("🔄 处理 Stream 中的每个元素：");
    while let Some(value) = stream.next().await {
        println!("   项: {}", value);
        sleep(Duration::from_millis(200)).await;
    }
    
    println!("\n📝 Stream 的常用操作：");
    
    // map 转换
    let doubled = stream::iter(vec![1, 2, 3])
        .map(|x| x * 2)
        .collect::<Vec<_>>()
        .await;
    println!("   map 结果: {:?}", doubled);
    
    // filter 过滤
    let evens = stream::iter(vec![1, 2, 3, 4, 5, 6])
        .filter(|&x| async move { x % 2 == 0 })
        .collect::<Vec<_>>()
        .await;
    println!("   filter 结果: {:?}", evens);
    
    // fold 累积
    let sum = stream::iter(vec![1, 2, 3, 4, 5])
        .fold(0, |acc, x| async move { acc + x })
        .await;
    println!("   fold 求和: {}\n", sum);
}

/// === 5. Waker 和唤醒机制 ===

async fn waker_concept() {
    println!("=== 5. Waker 唤醒机制 ===");
    println!("📝 理解异步运行时如何知道何时重新 poll Future\n");
    
    println!("💡 工作流程：");
    println!("   1. Runtime 调用 Future::poll()");
    println!("   2. 如果返回 Poll::Pending，Future 保存 Waker");
    println!("   3. 当 Future 准备好时，调用 waker.wake()");
    println!("   4. Runtime 重新 poll 该 Future");
    println!("   5. 如果返回 Poll::Ready(value)，Future 完成\n");
    
    println!("🔄 实际例子：定时器");
    println!("   • Timer 注册到事件循环");
    println!("   • poll() 返回 Pending 并保存 Waker");
    println!("   • 时间到后，定时器调用 wake()");
    println!("   • Runtime 重新 poll，返回 Ready\n");
}

#[tokio::main]
async fn main() {
    println!("🎓 Futures 和 Pin 深入理解教程\n");
    println!("💡 理解 Rust 异步的底层机制");
    
    custom_future_demo().await;
    pin_demo().await;
    combined_future_demo().await;
    stream_demo().await;
    waker_concept().await;
    
    println!("🎉 教程完成！\n");
    println!("💡 关键要点：");
    println!("   • Future trait 定义了异步计算的接口");
    println!("   • poll() 方法返回 Poll::Ready 或 Poll::Pending");
    println!("   • Pin 保证值不会在内存中移动，保护自引用");
    println!("   • Unpin 表示类型可以安全移动");
    println!("   • async/await 是 Future 的语法糖");
    println!("   • Stream 是异步版本的 Iterator");
    println!("   • Waker 机制让运行时知道何时重新 poll");
}

