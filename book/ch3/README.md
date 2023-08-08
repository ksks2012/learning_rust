# List

- str_part
- function_arg
- type_size
- botton_type
- type_inference
- generic_example2
- Trait
    - interface_abstraction
    - add_for_new_type
    - inherit_trait
- generic bundle
- abstract_type
- impl_trait
- marker_trait
- Type Conversion
    - deref


# Rust ch.3 Type System

# 目錄

# Intro

- Fine-grained differentiation of information value
- Different occupied Memory → More efficient using memory

# Usage

- Troubleshooting Errors
    - compiler time
    - run time
- Abstract
- Documentation
- Optimizing Efficiency
- Type Security
    - Avoid invalid calculations between types
    - Memory security
    - Avoid logic error
    - 

# Class

## Rust

- Check array’s edge in run time

## Python Ruby

- Run time check
- Duck typing
- 

# Polymorphism

## Parametric Polymorphism

- Almost Static Polymorphism
- generic type

## Ad-hoc Polymorphism

- Almost Static Polymorphism
- Trait in Rust
- 

## Subtype Polymorphism

- JAVA

## Static Polymorphism

- compile time
- more efficient

## Dynamic Polymorphism

- run time
- more flexibility

# Rust Type System

- Static
- All of things is formula

## Size

- Almost determined in compile time
- Dynamic Sized Type (DST)
    - string → reference → &str
    - 
    

## Botton Type

### Intro

- no value
- is sub type of other types
- tpye security

### include

- Diverging Function
    - panic
    - std::process::exit
    - never have return value
- Key word: countinue or break
- loop
- empty enumerate: enum Void{}

# Trait

- Restrictions on Types of Behavior

## Usage

- Interface Abstraction
- Generic Constraints
    - Behavior of generic had restrict by trait in a smaller range
- Abstract Type
    - Run time
    - Dynamic
- Label Trait

# Generic Bundle

- trait bound
- duck typing
- Combination > Inheritance

# Abstract Type

- Existential Type

## trait object

```rust
// strutct equal to trait
pub struct TraitObject {
    pub data: *mut (),
    pub vtable: *mut (),
}
```

- use for unsafe program
- data → heap
- vtable → static read-only area

## Object security

### Size of trait

```rust
<Self:?Sized>
<T: Sized>
```

### Bundle

- type of Self in trait can’t be bound to Sized
- All of method in trait must be object safe

### Object Safe

- method is bound by Self: Sized
- method signed should be:
    - Do not include trait parameters → trait can’t find exact method in vtable
    - First argument should be Self type or can be set as Self type
        
        ```rust
        self
        &self
        &mut self
        self:Box<self>
        ```
        
    - Self cannot appear in a position other than the first parameter
        - cannot do type comparison
    

# impl trait

```rust
use std::ops::Add;
fn sum<T>(a: impl Add<Output=T>, b: impl Add<Output=T>) -> T {
		a + b
}
```

# marker trait

## Sized trait

- ?Sized
    - T: Unsize
    - T: Sized

### Limit of Dynamic Size Type

- Only can control by fat pointer, ex: &[T] or &Trait
- parameter, argument and listing variables
- Only last field can be used as dynamic size type
- Lang Item standard implementation
    
    ```rust
    #[lang = "sized"]
    pub trait Sized {
    
    }
    ```
    

## Unsized trait

## Copy trait

- empty implement
- check clone
- Lang Item standard implementation
    
    ```rust
    #[lang = "copy"]
    pub trait Copy: Clone {
    
    }
    ```
    

## Send trait

- for avoid race condition
- safely send data (value, ownership) between thread
- Lang Item standard implementation
    
    ```rust
    #[lang = "send"]
    pub unsafe trait Send {
    
    }
    ```
    

## Sync trait

- for avoid race condition
- safely send Shared Reference between thread
- Lang Item standard implementation
    
    ```rust
    #[lang = "sync"]
    pub unsafe trait Sync {
    
    }
    ```
    

# Type Conversion

## Implicit Type Conversion (Type Coercion)

- by compiler
- by interpreter

## Explicit Type Conversion (Type Cast)

- by user

## Deref

- Explicit Type Conversion
- Common type in Rust have implement
    - Vec<T>
    - Box<T>
    - Rc<T>
    - Arc<T>
- Simplify program design
- 

### match

- 
    
    ```rust
    match x.deref()
    ```
    
- AsRef trait
    
    ```rust
    match x.as_ref()
    ```
    
- Borrow trait
    
    ```rust
    use std::borrow::Borrow
    match x.borrow()
    ```
    
- index
    
    ```rust
    match &x[..]
    ```
    
- String → str → &str
    
    ```rust
    match &*x
    ```