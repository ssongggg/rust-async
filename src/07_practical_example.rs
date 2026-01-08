// 07_practical_example.rs - 综合实战：异步 HTTP 服务器模拟
//
// 本示例演示一个完整的异步应用，综合运用：
// 1. Tokio 任务管理
// 2. Channel 通信
// 3. 并发控制
// 4. 错误处理
// 5. 优雅关闭

use tokio::sync::{mpsc, Semaphore};
use tokio::time::{sleep, Duration, timeout};
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

/// 请求结构
#[derive(Debug, Clone)]
struct Request {
    id: u64,
    path: String,
    processing_time: Duration,
}

/// 响应结构
#[derive(Debug)]
struct Response {
    request_id: u64,
    status: u16,
    #[allow(dead_code)]
    body: String,
}

/// 服务器统计信息
struct ServerStats {
    total_requests: AtomicU64,
    successful_requests: AtomicU64,
    failed_requests: AtomicU64,
}

impl ServerStats {
    fn new() -> Self {
        ServerStats {
            total_requests: AtomicU64::new(0),
            successful_requests: AtomicU64::new(0),
            failed_requests: AtomicU64::new(0),
        }
    }
    
    fn record_request(&self) {
        self.total_requests.fetch_add(1, Ordering::Relaxed);
    }
    
    fn record_success(&self) {
        self.successful_requests.fetch_add(1, Ordering::Relaxed);
    }
    
    fn record_failure(&self) {
        self.failed_requests.fetch_add(1, Ordering::Relaxed);
    }
    
    fn print_stats(&self) {
        let total = self.total_requests.load(Ordering::Relaxed);
        let success = self.successful_requests.load(Ordering::Relaxed);
        let failed = self.failed_requests.load(Ordering::Relaxed);
        
        println!("\n📊 服务器统计:");
        println!("   总请求数: {}", total);
        println!("   成功: {} ({:.1}%)", success, (success as f64 / total as f64) * 100.0);
        println!("   失败: {} ({:.1}%)", failed, (failed as f64 / total as f64) * 100.0);
    }
}

/// 请求处理器
struct RequestHandler {
    id: usize,
    stats: Arc<ServerStats>,
}

impl RequestHandler {
    async fn handle_request(&self, request: Request) -> Response {
        println!("🔧 处理器{} 开始处理请求 #{} ({})", 
            self.id, request.id, request.path);
        
        self.stats.record_request();
        
        // 模拟请求处理
        sleep(request.processing_time).await;
        
        // 模拟偶尔的失败
        let status = if request.id % 7 == 0 {
            self.stats.record_failure();
            500
        } else {
            self.stats.record_success();
            200
        };
        
        let response = Response {
            request_id: request.id,
            status,
            body: format!("Response for {}", request.path),
        };
        
        println!("✅ 处理器{} 完成请求 #{} (状态: {})", 
            self.id, request.id, status);
        
        response
    }
}

/// 负载均衡器
struct LoadBalancer {
    request_tx: mpsc::Sender<Request>,
    response_rx: Arc<tokio::sync::Mutex<mpsc::Receiver<Response>>>,
    semaphore: Arc<Semaphore>,
    #[allow(dead_code)]
    stats: Arc<ServerStats>,
}

impl LoadBalancer {
    fn new(max_concurrent: usize, stats: Arc<ServerStats>) -> Self {
        let (request_tx, request_rx) = mpsc::channel(100);
        let (response_tx, response_rx) = mpsc::channel(100);
        let semaphore = Arc::new(Semaphore::new(max_concurrent));
        
        // 启动工作者池 - 所有工作者共享一个 receiver
        let num_workers = 4;
        let request_rx = Arc::new(tokio::sync::Mutex::new(request_rx));
        
        for worker_id in 0..num_workers {
            let rx = request_rx.clone();
            let tx = response_tx.clone();
            let sem = semaphore.clone();
            let stats = stats.clone();
            
            tokio::spawn(async move {
                let handler = RequestHandler {
                    id: worker_id,
                    stats,
                };
                
                loop {
                    // 从共享 receiver 中获取请求
                    let request = {
                        let mut rx = rx.lock().await;
                        rx.recv().await
                    };
                    
                    match request {
                        Some(request) => {
                            let _permit = sem.acquire().await.unwrap();
                            let response = handler.handle_request(request).await;
                            if tx.send(response).await.is_err() {
                                break;
                            }
                        }
                        None => break,
                    }
                }
                
                println!("⚠️  工作者 {} 退出", worker_id);
            });
        }
        
        drop(response_tx); // 关闭发送端
        
        LoadBalancer {
            request_tx,
            response_rx: Arc::new(tokio::sync::Mutex::new(response_rx)),
            semaphore,
            stats,
        }
    }
    
    async fn submit_request(&self, request: Request) -> Result<(), &'static str> {
        self.request_tx
            .send(request)
            .await
            .map_err(|_| "无法提交请求")
    }
    
    async fn get_response(&self) -> Option<Response> {
        let mut rx = self.response_rx.lock().await;
        rx.recv().await
    }
    
    fn available_slots(&self) -> usize {
        self.semaphore.available_permits()
    }
}

/// 请求生成器
async fn request_generator(lb: Arc<LoadBalancer>, num_requests: u64) {
    println!("🚀 开始生成 {} 个请求\n", num_requests);
    
    for i in 1..=num_requests {
        let request = Request {
            id: i,
            path: format!("/api/endpoint{}", i % 5),
            processing_time: Duration::from_millis(100 + (i % 5) * 50),
        };
        
        println!("📤 提交请求 #{}", i);
        
        match lb.submit_request(request).await {
            Ok(_) => {},
            Err(e) => {
                println!("❌ 提交请求失败: {}", e);
                break;
            }
        }
        
        // 模拟请求到达的间隔
        sleep(Duration::from_millis(50)).await;
    }
    
    println!("\n✅ 所有请求已提交");
}

/// 响应收集器
async fn response_collector(lb: Arc<LoadBalancer>, expected_count: u64) {
    println!("📥 响应收集器启动\n");
    
    let mut received = 0;
    
    while received < expected_count {
        // 设置超时避免无限等待
        match timeout(Duration::from_secs(10), lb.get_response()).await {
            Ok(Some(response)) => {
                received += 1;
                if response.status == 200 {
                    println!("✅ 收到响应 #{}: 成功", response.request_id);
                } else {
                    println!("⚠️  收到响应 #{}: 失败 (状态: {})", 
                        response.request_id, response.status);
                }
            }
            Ok(None) => {
                println!("⚠️  响应通道关闭");
                break;
            }
            Err(_) => {
                println!("⏱️  等待响应超时");
                break;
            }
        }
    }
    
    println!("\n📦 收集器完成，共收到 {} 个响应", received);
}

/// 监控任务
async fn monitor_task(lb: Arc<LoadBalancer>, duration: Duration) {
    let start = tokio::time::Instant::now();
    let mut interval = tokio::time::interval(Duration::from_secs(2));
    
    while start.elapsed() < duration {
        interval.tick().await;
        println!("\n📊 监控: 可用槽位 = {}", lb.available_slots());
    }
}

/// 主服务器函数
async fn run_server() {
    println!("🎓 综合实战：异步 HTTP 服务器模拟\n");
    println!("{}", "=".repeat(50));
    
    // 创建服务器组件
    let stats = Arc::new(ServerStats::new());
    let load_balancer = Arc::new(LoadBalancer::new(3, stats.clone()));
    
    println!("⚙️  服务器配置:");
    println!("   • 最大并发: 3");
    println!("   • 工作者数量: 4");
    println!("   • 请求队列大小: 100\n");
    
    let num_requests = 20;
    
    // 启动各个组件
    let lb_clone1 = load_balancer.clone();
    let generator = tokio::spawn(async move {
        request_generator(lb_clone1, num_requests).await;
    });
    
    let lb_clone2 = load_balancer.clone();
    let collector = tokio::spawn(async move {
        response_collector(lb_clone2, num_requests).await;
    });
    
    let lb_clone3 = load_balancer.clone();
    let monitor = tokio::spawn(async move {
        monitor_task(lb_clone3, Duration::from_secs(15)).await;
    });
    
    // 等待所有任务完成
    let _ = tokio::join!(generator, collector, monitor);
    
    println!("\n{}", "=".repeat(50));
    println!("{}", "=".repeat(50));
    stats.print_stats();
    println!("{}", "=".repeat(50));
    
    println!("\n🎉 服务器模拟完成！");
}

/// 演示优雅关闭
async fn graceful_shutdown_demo() {
    use tokio::sync::broadcast;
    
    println!("\n\n🛑 优雅关闭演示");
    println!("📝 按 Ctrl+C 不会立即终止，而是等待任务完成\n");
    
    let (shutdown_tx, _) = broadcast::channel::<()>(1);
    
    // 模拟一些长时间运行的任务
    let mut tasks = vec![];
    
    for i in 1..=3 {
        let mut shutdown_rx = shutdown_tx.subscribe();
        let task = tokio::spawn(async move {
            loop {
                tokio::select! {
                    _ = shutdown_rx.recv() => {
                        println!("   🛑 任务 {} 收到关闭信号", i);
                        break;
                    }
                    _ = sleep(Duration::from_millis(500)) => {
                        println!("   🔄 任务 {} 运行中...", i);
                    }
                }
            }
            
            println!("   ✅ 任务 {} 清理完成", i);
        });
        tasks.push(task);
    }
    
    // 模拟接收关闭信号
    sleep(Duration::from_secs(2)).await;
    println!("\n📢 发送关闭信号...\n");
    let _ = shutdown_tx.send(());
    
    // 等待所有任务完成
    for task in tasks {
        let _ = task.await;
    }
    
    println!("\n✅ 所有任务已优雅关闭");
}

#[tokio::main]
async fn main() {
    // 运行主服务器模拟
    run_server().await;
    
    // 演示优雅关闭
    graceful_shutdown_demo().await;
    
    println!("\n💡 本示例展示了：");
    println!("   ✓ 任务生成和管理 (tokio::spawn)");
    println!("   ✓ Channel 通信 (mpsc)");
    println!("   ✓ 并发限制 (Semaphore)");
    println!("   ✓ 原子操作 (AtomicU64)");
    println!("   ✓ 超时处理 (timeout)");
    println!("   ✓ 优雅关闭 (broadcast + select!)");
    println!("   ✓ 错误处理和统计");
    println!("\n🎓 恭喜完成所有教程！你已经掌握了 Rust 异步编程的核心概念！");
}

