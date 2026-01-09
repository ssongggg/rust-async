// ownership_example.rs - Rust 所有权、引用、借用完整教程
//
// 本示例将帮助你理解 Rust 三个核心概念：
// 1. 所有权（Ownership）
// 2. 引用（Reference）
// 3. 借用（Borrowing）

use std::fmt;

/// 自定义结构体用于演示
#[derive(Debug, Clone)]
struct Book {
    title: String,
    author: String,
    pages: u32,
}

impl Book {
    fn new(title: &str, author: &str, pages: u32) -> Self {
        Book {
            title: title.to_string(),
            author: author.to_string(),
            pages,
        }
    }
}

impl fmt::Display for Book {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "《{}》作者: {}, {}页", self.title, self.author, self.pages)
    }
}

/// ============================================
/// 第一部分：所有权基础
/// ============================================
fn demo_ownership_basics() {
    println!("\n📚 第一部分：所有权基础");
    println!("{}", "=".repeat(60));
    
    // 1. 基本所有权 - 每个值都有一个所有者
    {
        println!("\n1️⃣  基本所有权概念：");
        let s1 = String::from("Hello");
        println!("   s1 拥有字符串: {}", s1);
        
        // s1 在这里仍然有效
        println!("   s1 仍然可以使用: {}", s1);
    } // s1 在这里离开作用域，内存被释放
    
    // 2. 所有权转移（Move）
    {
        println!("\n2️⃣  所有权转移（Move）：");
        let s1 = String::from("Hello");
        println!("   创建 s1: {}", s1);
        
        let s2 = s1; // s1 的所有权转移给 s2
        println!("   s1 移动到 s2: {}", s2);
        
        // ❌ 错误！s1 已经失效，不能再使用
        // println!("   尝试使用 s1: {}", s1); // 这行会编译错误
        println!("   ⚠️  s1 已经失效，所有权已转移给 s2");
    }
    
    // 3. 克隆（Clone）- 深拷贝
    {
        println!("\n3️⃣  克隆（Clone）- 创建深拷贝：");
        let s1 = String::from("Hello");
        let s2 = s1.clone(); // 显式克隆，创建完整副本
        
        println!("   s1: {}", s1);
        println!("   s2: {}", s2);
        println!("   ✅ s1 和 s2 都有效，因为使用了 clone()");
    }
    
    // 4. Copy trait - 栈上的简单类型
    {
        println!("\n4️⃣  Copy trait - 简单类型自动复制：");
        let x = 5;
        let y = x; // 整数实现了 Copy trait，会自动复制
        
        println!("   x: {}", x);
        println!("   y: {}", y);
        println!("   ✅ x 和 y 都有效，整数类型会自动复制");
    }
}

/// ============================================
/// 第二部分：函数与所有权
/// ============================================

// 这个函数会获取所有权
fn take_ownership(book: Book) {
    println!("   📖 函数内部: {}", book);
    // book 在这里离开作用域并被释放
}

// 这个函数会返回所有权
fn give_ownership() -> Book {
    let book = Book::new("Rust 编程", "Steve Klabnik", 500);
    book // 返回所有权给调用者
}

// 这个函数获取并返回所有权
fn take_and_return_ownership(book: Book) -> Book {
    println!("   📖 处理中: {}", book);
    book // 返回所有权
}

fn demo_ownership_functions() {
    println!("\n📚 第二部分：函数与所有权");
    println!("{}", "=".repeat(60));
    
    // 1. 传递所有权到函数
    {
        println!("\n1️⃣  传递所有权到函数：");
        let book1 = Book::new("深入理解计算机系统", "Randal E. Bryant", 1000);
        println!("   创建 book1: {}", book1);
        
        take_ownership(book1); // book1 的所有权转移到函数内
        
        // ❌ book1 在这里已经失效
        // println!("   {}", book1); // 这行会编译错误
        println!("   ⚠️  book1 已失效，所有权已转移到函数内");
    }
    
    // 2. 函数返回所有权
    {
        println!("\n2️⃣  函数返回所有权：");
        let book2 = give_ownership();
        println!("   从函数获得 book2: {}", book2);
        println!("   ✅ book2 现在拥有所有权");
    }
    
    // 3. 获取并返回所有权
    {
        println!("\n3️⃣  获取并返回所有权：");
        let book3 = Book::new("算法导论", "Thomas H. Cormen", 1200);
        println!("   创建 book3: {}", book3);
        
        let book3 = take_and_return_ownership(book3);
        println!("   取回 book3: {}", book3);
        println!("   ✅ book3 的所有权被返回，仍然有效");
    }
}

/// ============================================
/// 第三部分：引用和借用
/// ============================================

// 不可变引用 - 只读借用
fn read_book(book: &Book) {
    println!("   📖 读取书籍: {}", book);
    println!("   📄 作者: {}", book.author);
    // 不能修改借用的内容
    // book.pages = 100; // ❌ 这会编译错误
}

// 可变引用 - 可以修改
fn add_pages(book: &mut Book, additional_pages: u32) {
    println!("   ✏️  添加 {} 页", additional_pages);
    book.pages += additional_pages;
}

// 多个不可变引用
fn compare_books(book1: &Book, book2: &Book) {
    println!("   📊 比较: {} vs {}", book1.title, book2.title);
    if book1.pages > book2.pages {
        println!("   结果: {} 页数更多", book1.title);
    } else {
        println!("   结果: {} 页数更多", book2.title);
    }
}

fn demo_references_borrowing() {
    println!("\n📚 第三部分：引用和借用");
    println!("{}", "=".repeat(60));
    
    // 1. 不可变引用（借用）
    {
        println!("\n1️⃣  不可变引用 &T - 只读借用：");
        let book = Book::new("The Rust Programming Language", "Steve Klabnik", 500);
        
        read_book(&book); // 借用，不转移所有权
        println!("   ✅ book 仍然有效: {}", book);
        
        // 可以多次不可变借用
        read_book(&book);
        read_book(&book);
        println!("   ✅ 可以有多个不可变引用");
    }
    
    // 2. 可变引用
    {
        println!("\n2️⃣  可变引用 &mut T - 可修改借用：");
        let mut book = Book::new("Rust in Action", "Tim McNamara", 400);
        println!("   初始: {}", book);
        
        add_pages(&mut book, 50); // 可变借用
        println!("   修改后: {}", book);
        println!("   ✅ 通过可变引用修改了内容");
    }
    
    // 3. 借用规则演示
    {
        println!("\n3️⃣  借用规则：");
        let mut book = Book::new("Programming Rust", "Jim Blandy", 600);
        
        // 规则1: 可以有多个不可变引用
        let r1 = &book;
        let r2 = &book;
        println!("   📚 r1: {}", r1.title);
        println!("   📚 r2: {}", r2.title);
        println!("   ✅ 多个不可变引用可以共存");
        
        // 规则2: 只能有一个可变引用
        {
            let r3 = &mut book;
            r3.pages += 10;
            println!("   ✏️  通过可变引用修改: 现在有 {} 页", r3.pages);
            // ❌ 在 r3 存在时，不能有其他引用
            // let r4 = &book; // 这会编译错误
        } // r3 在这里结束
        
        println!("   ✅ 可变引用结束后，可以再次借用");
        let r4 = &book;
        println!("   📚 r4: {}", r4.title);
    }
    
    // 4. 多个不可变引用的实际应用
    {
        println!("\n4️⃣  多个不可变引用的实际应用：");
        let book1 = Book::new("Clean Code", "Robert C. Martin", 464);
        let book2 = Book::new("Code Complete", "Steve McConnell", 960);
        
        compare_books(&book1, &book2);
        println!("   ✅ 两本书都没有被移动，仍然可用");
    }
}

/// ============================================
/// 第四部分：常见陷阱和解决方案
/// ============================================

fn demo_common_pitfalls() {
    println!("\n📚 第四部分：常见陷阱和解决方案");
    println!("{}", "=".repeat(60));
    
    // 陷阱1: 悬垂引用（Dangling Reference）
    {
        println!("\n1️⃣  悬垂引用（Rust 会阻止）：");
        
        // ❌ 这个函数会产生悬垂引用（编译错误）
        // fn create_dangling() -> &Book {
        //     let book = Book::new("Test", "Test", 100);
        //     &book // book 会在函数结束时被释放，返回的引用无效
        // }
        
        // ✅ 正确做法：返回所有权
        fn create_valid() -> Book {
            Book::new("Valid Book", "Valid Author", 100)
        }
        
        let book = create_valid();
        println!("   ✅ 正确：返回所有权而不是引用");
        println!("   📖 {}", book);
    }
    
    // 陷阱2: 可变和不可变引用冲突
    {
        println!("\n2️⃣  可变和不可变引用不能同时存在：");
        let mut numbers = vec![1, 2, 3, 4, 5];
        
        // ❌ 这样会编译错误
        // let r1 = &numbers;
        // let r2 = &mut numbers; // 在 r1 存在时不能创建可变引用
        // println!("{:?}", r1);
        
        // ✅ 正确做法：分开使用
        {
            let r1 = &numbers;
            println!("   📖 不可变引用: {:?}", r1);
        } // r1 结束
        
        {
            let r2 = &mut numbers;
            r2.push(6);
            println!("   ✏️  可变引用: {:?}", r2);
        } // r2 结束
        
        println!("   ✅ 通过分离作用域避免冲突");
    }
    
    // 陷阱3: 在循环中的所有权
    {
        println!("\n3️⃣  在循环中的所有权：");
        let books = vec![
            Book::new("Book 1", "Author 1", 100),
            Book::new("Book 2", "Author 2", 200),
            Book::new("Book 3", "Author 3", 300),
        ];
        
        // ❌ 这样会移动所有权
        // for book in books {
        //     println!("{}", book);
        // }
        // println!("{:?}", books); // books 已经失效
        
        // ✅ 正确做法：使用引用迭代
        println!("   遍历书籍（使用引用）：");
        for book in &books {
            println!("   - {}", book);
        }
        println!("   ✅ books 仍然有效，可以继续使用");
        println!("   📚 总共 {} 本书", books.len());
    }
}

/// ============================================
/// 第五部分：实战示例 - 图书管理系统
/// ============================================

struct Library {
    books: Vec<Book>,
    name: String,
}

impl Library {
    fn new(name: &str) -> Self {
        Library {
            books: Vec::new(),
            name: name.to_string(),
        }
    }
    
    // 获取所有权并添加书籍
    fn add_book(&mut self, book: Book) {
        println!("   ➕ 添加书籍: {}", book.title);
        self.books.push(book);
    }
    
    // 借用：不可变引用查找书籍
    fn find_book(&self, title: &str) -> Option<&Book> {
        self.books.iter().find(|book| book.title == title)
    }
    
    // 借用：可变引用更新书籍
    fn update_book_pages(&mut self, title: &str, new_pages: u32) -> bool {
        if let Some(book) = self.books.iter_mut().find(|book| book.title == title) {
            println!("   ✏️  更新 '{}' 的页数: {} -> {}", book.title, book.pages, new_pages);
            book.pages = new_pages;
            true
        } else {
            false
        }
    }
    
    // 借用：不可变引用列出所有书籍
    fn list_books(&self) {
        println!("   📚 {} 的藏书:", self.name);
        for (i, book) in self.books.iter().enumerate() {
            println!("      {}. {}", i + 1, book);
        }
    }
    
    // 返回书籍数量（不需要借用self）
    fn book_count(&self) -> usize {
        self.books.len()
    }
}

fn demo_practical_example() {
    println!("\n📚 第五部分：实战示例 - 图书管理系统");
    println!("{}", "=".repeat(60));
    
    let mut library = Library::new("清华大学图书馆");
    
    println!("\n1️⃣  添加书籍（转移所有权）：");
    let book1 = Book::new("算法导论", "Thomas H. Cormen", 1200);
    let book2 = Book::new("深入理解计算机系统", "Randal E. Bryant", 1000);
    let book3 = Book::new("代码大全", "Steve McConnell", 960);
    
    library.add_book(book1); // book1 所有权转移到 library
    library.add_book(book2);
    library.add_book(book3);
    // ❌ book1, book2, book3 在这里已经失效
    
    println!("\n2️⃣  列出所有书籍（不可变借用）：");
    library.list_books();
    
    println!("\n3️⃣  查找书籍（不可变借用）：");
    if let Some(book) = library.find_book("算法导论") {
        println!("   🔍 找到: {}", book);
    }
    
    println!("\n4️⃣  更新书籍（可变借用）：");
    library.update_book_pages("算法导论", 1300);
    
    println!("\n5️⃣  再次列出（验证修改）：");
    library.list_books();
    
    println!("\n6️⃣  统计信息（不可变借用）：");
    println!("   📊 图书总数: {}", library.book_count());
    
    println!("\n✅ 图书管理系统演示完成！");
}

/// ============================================
/// 第六部分：关键概念总结
/// ============================================

fn print_summary() {
    println!("\n📚 关键概念总结");
    println!("{}", "=".repeat(60));
    
    println!("\n🎯 所有权规则：");
    println!("   1. 每个值都有一个所有者");
    println!("   2. 一次只能有一个所有者");
    println!("   3. 当所有者离开作用域，值被释放");
    
    println!("\n🎯 借用规则：");
    println!("   1. 任意时刻，只能满足以下条件之一：");
    println!("      • 一个可变引用 (&mut T)");
    println!("      • 任意数量的不可变引用 (&T)");
    println!("   2. 引用必须始终有效（不能悬垂）");
    
    println!("\n🎯 何时使用：");
    println!("   • 所有权转移：函数需要完全拥有数据");
    println!("   • 不可变引用：只需要读取数据");
    println!("   • 可变引用：需要修改数据");
    println!("   • Clone：需要保留原数据又需要新副本");
    
    println!("\n🎯 记忆口诀：");
    println!("   📦 所有权：一个值一个主人");
    println!("   👀 不可变借用：多人可看不可改");
    println!("   ✏️  可变借用：独占修改不可看");
    println!("   ⏰ 生命周期：引用不能活过主人");
}

/// ============================================
/// 主函数
/// ============================================

fn main() {
    println!("🎓 Rust 所有权、引用、借用完整教程");
    println!("{}", "=".repeat(60));
    
    // 运行所有演示
    demo_ownership_basics();
    demo_ownership_functions();
    demo_references_borrowing();
    demo_common_pitfalls();
    demo_practical_example();
    print_summary();
    
    println!("\n{}", "=".repeat(60));
    println!("🎉 恭喜！你已经学习了 Rust 最核心的概念！");
    println!("💡 建议：多运行几次，尝试取消注释那些会报错的代码，");
    println!("   观察编译器的错误信息，这有助于加深理解。");
}
