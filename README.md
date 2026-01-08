# Rust 异步编程学习项目

🎓 一个全面的 Rust 异步编程教程项目，帮助初学者深入理解 Tokio、async/await、并发模型、Futures、Pin、Send/Sync 等核心概念。

## 📚 项目结构

本项目包含 7 个循序渐进的示例程序，每个都专注于特定的概念：

```
rust-async-learning/
├── Cargo.toml
├── README.md
└── src/
    ├── 01_async_basics.rs      # Async/Await 基础
    ├── 02_tokio_spawn.rs        # Tokio 任务生成
    ├── 03_concurrent_tasks.rs   # 并发模型
    ├── 04_futures_pin.rs        # Futures 和 Pin
    ├── 05_send_sync.rs          # Send/Sync traits
    ├── 06_channels.rs           # Channel 通信
    └── 07_practical_example.rs  # 综合实战示例
```

## 🚀 快速开始

### 前置要求

- Rust 1.70 或更高版本
- Cargo

### 安装依赖

```bash
cargo build
```

### 运行示例

按照顺序运行每个示例来学习不同的概念：

```bash
# 1. Async/Await 基础
cargo run --bin 01_async_basics

# 2. Tokio Spawn 和任务管理
cargo run --bin 02_tokio_spawn

# 3. 并发模型深入
cargo run --bin 03_concurrent_tasks

# 4. Futures 和 Pin 深入理解
cargo run --bin 04_futures_pin

# 5. Send 和 Sync Traits
cargo run --bin 05_send_sync

# 6. Channel 通信模式
cargo run --bin 06_channels

# 7. 综合实战：异步 HTTP 服务器模拟
cargo run --bin 07_practical_example
```

## 📖 学习路径

### 01. Async/Await 基础 (`01_async_basics.rs`)

**学习内容：**
- `async` 关键字的作用
- `await` 关键字的使用
- Future 的惰性特性
- `#[tokio::main]` 运行时
- 顺序执行 vs 并发执行 (`tokio::join!`)

**核心要点：**
- async 函数返回 Future
- Future 必须被 await 才会执行
- tokio::join! 可以并发执行多个 Future

---

### 02. Tokio Spawn (`02_tokio_spawn.rs`)

**学习内容：**
- `tokio::spawn` 创建新任务
- `JoinHandle` 的使用
- 任务的并发执行
- 任务取消 (`abort`)
- `spawn_blocking` 处理阻塞代码

**核心要点：**
- spawn 创建独立的异步任务
- 任务在后台并发执行
- spawn 的任务必须是 'static 生命周期

---

### 03. 并发模型 (`03_concurrent_tasks.rs`)

**学习内容：**
- `select!` 宏（竞争式并发）
- 超时处理 (`timeout`)
- 信号量 (`Semaphore`) 限制并发
- `oneshot` channel
- 取消安全性
- `FuturesUnordered` 动态任务集合

**核心要点：**
- select! 处理第一个完成的 Future
- 未完成的分支会被取消
- Semaphore 控制并发数量

---

### 04. Futures 和 Pin (`04_futures_pin.rs`)

**学习内容：**
- Future trait 的定义
- 手动实现 Future
- `Pin` 和 `Unpin` 的作用
- 自引用结构体的问题
- Stream（异步迭代器）
- Waker 唤醒机制

**核心要点：**
- Future::poll() 返回 Poll::Ready 或 Poll::Pending
- Pin 保证值不会在内存中移动
- async/await 是 Future 的语法糖

---

### 05. Send 和 Sync (`05_send_sync.rs`)

**学习内容：**
- Send trait：类型可以在线程间转移
- Sync trait：类型可以在线程间共享引用
- `!Send` 类型（如 Rc、RefCell）
- `Arc<Mutex<T>>` 共享可变数据模式
- `tokio::sync::Mutex` vs `std::sync::Mutex`
- `RwLock` 读写锁

**核心要点：**
- tokio::spawn 要求 Future 是 Send
- Rc 不是 Send，使用 Arc 代替
- RefCell 不是 Send，使用 Mutex 代替

---

### 06. Channel 通信 (`06_channels.rs`)

**学习内容：**
- `mpsc` channel（多生产者单消费者）
- 有界 vs 无界 channel
- `oneshot` channel（一次性通信）
- `broadcast` channel（广播）
- `watch` channel（状态共享）
- 工作队列模式

**核心要点：**
- mpsc 最常用，适合工作队列
- oneshot 用于请求-响应模式
- broadcast 用于事件通知
- watch 用于状态监控

---

### 07. 综合实战 (`07_practical_example.rs`)

**学习内容：**
- 完整的异步应用架构
- 负载均衡器实现
- 请求处理和响应收集
- 并发限制和流量控制
- 统计和监控
- 优雅关闭机制

**核心要点：**
- 综合运用所有学到的概念
- 实际应用中的最佳实践
- 错误处理和资源管理

## 🎯 核心概念总结

### Async/Await
```rust
async fn my_function() -> i32 {
    // async 函数返回 Future
    tokio::time::sleep(Duration::from_secs(1)).await;
    42
}
```

### Tokio Spawn
```rust
let handle = tokio::spawn(async {
    // 独立的异步任务
    42
});
let result = handle.await.unwrap();
```

### Concurrent Execution
```rust
// 并发执行
tokio::join!(task1(), task2(), task3());

// 竞争执行
tokio::select! {
    result = task1() => { /* ... */ }
    result = task2() => { /* ... */ }
}
```

### Shared State
```rust
// 共享可变数据
let data = Arc::new(Mutex::new(Vec::new()));
```

### Channel Communication
```rust
// 创建 channel
let (tx, mut rx) = mpsc::channel(100);

// 发送
tx.send(value).await?;

// 接收
if let Some(value) = rx.recv().await {
    // 处理 value
}
```

## 💡 最佳实践

1. **选择合适的运行时**
   - 使用 `#[tokio::main]` 快速开始
   - 了解运行时的配置选项

2. **避免阻塞运行时**
   - 使用 `spawn_blocking` 处理 CPU 密集型任务
   - 避免在 async 代码中调用同步阻塞函数

3. **正确使用 Send/Sync**
   - `tokio::spawn` 需要 `Send` bound
   - 使用 `Arc` 代替 `Rc`
   - 使用 `Mutex` 代替 `RefCell`

4. **选择合适的 Mutex**
   - 短时间持有锁：使用 `std::sync::Mutex`
   - 需要在 `.await` 点持有锁：使用 `tokio::sync::Mutex`

5. **Channel 选择**
   - 工作队列：`mpsc`
   - 请求-响应：`oneshot`
   - 广播：`broadcast`
   - 状态共享：`watch`

6. **并发控制**
   - 使用 `Semaphore` 限制并发数
   - 使用 `timeout` 避免无限等待
   - 使用 `select!` 实现超时和取消

## 🔍 常见问题

### 为什么需要 Pin？

Pin 保证值不会在内存中移动，这对于自引用结构体至关重要。async 块可能产生自引用的 Future，因此需要 Pin。

### Send 和 Sync 有什么区别？

- **Send**: 类型可以在线程间**转移所有权**
- **Sync**: 类型可以在线程间**共享引用**（&T 是 Send）

### 何时使用 spawn_blocking？

当需要执行 CPU 密集型计算或调用阻塞的同步代码时，使用 `spawn_blocking` 避免阻塞异步运行时。

### 如何选择 Channel？

- 多对一：`mpsc`
- 一次性：`oneshot`
- 一对多：`broadcast`
- 状态订阅：`watch`

## 📚 推荐资源

- [Tokio 官方文档](https://tokio.rs/)
- [Async Book](https://rust-lang.github.io/async-book/)
- [Rust 异步编程指南](https://rust-lang.github.io/async-book/)

## 🤝 贡献

欢迎提出问题和改进建议！

## 📝 许可证

本项目仅供学习使用。

---

**祝你学习愉快！🎉**

如果这个项目对你有帮助，请给它一个 ⭐

