# 🚀 快速入门指南

## 第一步：构建项目

```bash
cargo build
```

## 第二步：按顺序运行示例

建议按照以下顺序学习，每个示例都建立在前面的基础上：

### 1️⃣ Async/Await 基础（约5分钟）
```bash
cargo run --bin 01_async_basics
```
**你将学到：**
- async 和 await 的基本用法
- Future 的惰性特性
- 如何并发执行多个任务

---

### 2️⃣ Tokio Spawn（约5分钟）
```bash
cargo run --bin 02_tokio_spawn
```
**你将学到：**
- 如何创建独立的异步任务
- JoinHandle 的使用
- 任务取消机制

---

### 3️⃣ 并发模型（约8分钟）
```bash
cargo run --bin 03_concurrent_tasks
```
**你将学到：**
- select! 宏的竞争式并发
- 超时处理
- 如何限制并发数量

---

### 4️⃣ Futures 和 Pin（约10分钟）
```bash
cargo run --bin 04_futures_pin
```
**你将学到：**
- Future trait 的底层原理
- 为什么需要 Pin
- Stream 异步迭代器

---

### 5️⃣ Send 和 Sync（约10分钟）
```bash
cargo run --bin 05_send_sync
```
**你将学到：**
- 线程安全的核心概念
- 什么类型可以在线程间传递
- 如何共享可变数据

---

### 6️⃣ Channel 通信（约8分钟）
```bash
cargo run --bin 06_channels
```
**你将学到：**
- 不同类型的 Channel
- 如何在任务间通信
- 工作队列模式

---

### 7️⃣ 综合实战（约10分钟）
```bash
cargo run --bin 07_practical_example
```
**你将学到：**
- 如何构建完整的异步应用
- 负载均衡和流量控制
- 优雅关闭机制

---

## 💡 学习建议

1. **边看代码边运行**
   - 每个文件都有详细的中文注释
   - 先看注释理解概念，再运行观察输出

2. **尝试修改代码**
   - 改变参数看看会发生什么
   - 注释掉某些代码观察差异

3. **循序渐进**
   - 不要跳过前面的示例
   - 每个概念都建立在之前的基础上

4. **动手实践**
   - 学完后尝试写一个小项目
   - 比如：简单的聊天服务器、网页爬虫等

## 📊 估计时间

- **总学习时间：** 约 1-1.5 小时
- **每个示例：** 5-10 分钟
- **完成后你将：** 掌握 Rust 异步编程的核心概念

## 🎯 完成标志

当你能回答以下问题时，说明你已经掌握了核心概念：

✅ async/await 是什么？为什么需要它们？  
✅ tokio::spawn 和直接 await 有什么区别？  
✅ 什么时候用 select!，什么时候用 join!？  
✅ Future 为什么需要 Pin？  
✅ Send 和 Sync 有什么区别？  
✅ 如何选择合适的 Channel 类型？  

## 🆘 遇到问题？

1. 仔细阅读代码注释
2. 查看 README.md 的常见问题部分
3. 参考 Tokio 官方文档

---

**准备好了吗？让我们开始吧！🎉**

```bash
cargo run --bin 01_async_basics
```

