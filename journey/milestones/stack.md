---
description: Picking the stack
---

# Stack

Many open-source firewall software programs are written in either C or Python. Whilst I have ample experience with both I didn’t want to build my WAF in either for several reasons, but here are the biggest ones:

* Performance
* Maximum type-safety

I needed speed and type safety and also a language with solid concurrency support for later down the track (for when it comes time to beef this thing up). There were two candidates for the position, **Go** and **Rust**.

I liked the idea of Go and it's very simple to learn. But of course, I wanted to challenge myself and learn Rust in a very short amount of time. Plus I wanted to ditch truly garbage Garbage collectors and take Rust’s ownership system for spin. I mean just look at the code below (goodbye null pointer dereferences).

```rust
let maybe_value: Option<i32> = Some(42);
if let Some(value) = maybe_value {
    // Safely access the value
}
```

Overall Go promises a more productive and efficient development experience whilst Rust focuses more on performance and reliability which are paramount to any security project.&#x20;

Building a security system that’s not reliable 100% of the time is as good as building a security system that’s not reliable at all, so Rust was an easy choice.&#x20;

***

One noteworthy mention

[OpenResty™](https://openresty.org/en/) is an extension of NGINX that allows developers to write module logic using Lua which is a lightweight and easy-to-learn language. I chose to stay away from Lua for several [_safety_ _reasons_](https://news.ycombinator.com/item?id=18334407)_,_ but nevertheless this project is very progressive.&#x20;
