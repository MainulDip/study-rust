### Covered Topics and Next:
- Generics
- Traits
- Lifetimes
- Automated Tests
- Functional Programming | Iterators & Closures
- Smart Pointers
- Fearless Concurrency
- Asynchronous programming | Async/Await, Futures & Streams
- OOP
- Patterns and Matching
- Advance Features | Unsafe Rust, Advanced Traits,  Advanced Types
- Advanced Functions and Closures
- Macros | After the Fundamentals -> Little Book of Rust Macro | https://lukaswirth.dev/tlborm/introduction.html | Do it on you own pace
- Building a web server and make that multi threaded

### Generics:
We use generics to create definitions for items like function signatures or structs, which we can then use with many different concrete data types.

```rust
/**
* here both largest_i32 and largest_char are non-generics, but function the same. They both find the largest from numbers or chars vector/list
* the largest_finding_generics function is the combination of both using generics with a single function definition
* note: because we're comparing values, we need bound T with the `std::cmp::PartialOrd` trait
*/

fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// implementing generics for finding largest number or char
fn largest_findings_generics<T: std::cmp::PartialOrd> (list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
```