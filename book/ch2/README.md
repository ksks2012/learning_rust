# Rust Ch.2

Last Edit Time: July 17, 2023 1:37 AM
Tags: Rust, Tech Note

# 目錄

# Cargo

- create project
    
    ```bash
    cargo new ${project name}
    ```
    
- build project
    
    ```bash
    cargo build
    ```
    
- execute project
    
    ```bash
    cargo run
    ```
    
- pre-check project
    
    ```bash
    cargo check
    ```
    

# Basic

## 作用域

- 靜態作用域 → Lexical Scope
- 以大括號來開闢作用域

## CTFE 機制

- Compile Time Function Execution
- EX: 陣列需要再編譯時得知長度 → 可使用函數回傳值

## 閉包

- 匿名函數
    - 可以像函數一樣被呼叫
    - 可以捕捉上下文環境中的自由變數
    - 可以自動推斷輸入和傳回的類型

## 流程控制運算式

## 基本型態

### str

- 可變
    
    ```rust
    &str
    ```
    
- 不可變
    
    ```rust
    String
    ```
    

# Variable Length Array (VLA)

# pointer

- Reference
    - use in Safe Rust
- Raw Pointer
    - use in Unsafe Rust
- fn Pointer
- Smart Pointer

# never

# enum

- standard enum
- c enum
- enum with parameter
- fn-pointer type
- Option<T>

# Collection

- Vec<T>
- VecDeque<T>
- LinkedList<T>
- HashMap<K, V> & BTreeMap<K, V>
    - HashMap → unordered
    - BTreeMap → ordered
- HashSet<K, V> & BTreeSet<K, V>
    - HashSet → random
    - BTreeSet → ordered
- BinaryHeap<T>

# Smart Pointer

- auto free memory from heap
- Box<T>

# Option

- Option<T>
    - T means any type

# Trait

- Interface
- Can be static or dynamic
    - Static
        
        ```rust
        trait Fly {
            fn fly(&self) -> bool;
        }
        
        fn fly_static<T: Fly>(s: T) -> bool {
            s.fly()
        }
        ```
        
    - Dynamic
        
        ```rust
        trait Fly {
            fn fly(&self) -> bool;
        }
        
        fn fly_dyn(s: &dyn Fly) -> bool {
            s.fly()
        }
        ```
        
- Label

# Reference

[Rust 程式設計語言](https://rust-lang.tw/book-tw/title-page.html)

[This Week in Rust 464 · This Week in Rust](https://this-week-in-rust.org/blog/2022/10/12/this-week-in-rust-464/)