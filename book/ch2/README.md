# Rust Ch.2

Last Edit Time: September 28, 2023 11:00 AM
Tags: Rust, Tech Note
Status: 🌲
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

## Scopes

- Static Scopes → Lexical Scope
- Scoping with Parentheses

## CTFE Mechanism

- Compile Time Function Execution
- EX: The length of the array is known when it needs to be recompiled
    
    → the value can be returned using a function.
    

## Closures

- Anonymous Function
    - Can be called like functions
    - Captures free variables in contextual environments
    - Input and return types can be automatically inferred.

## Process Control Arithmetic

## Basic type

### str

- Mutable
    
    ```rust
    &str
    ```
    
- Immutable
    
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

[Rust Coding Language](https://rust-lang.tw/book-tw/title-page.html)

[This Week in Rust 464 · This Week in Rust](https://this-week-in-rust.org/blog/2022/10/12/this-week-in-rust-464/)