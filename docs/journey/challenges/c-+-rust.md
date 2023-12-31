---
layout:
  title:
    visible: true
  description:
    visible: true
  tableOfContents:
    visible: true
  outline:
    visible: true
  pagination:
    visible: true
---

# C + Rust

The heavy lifting of this WAF will be done by the [libinjection](https://github.com/libinjection/libinjection) project. It’s well maintained and covers a lot of cases we won’t be able to ship as MVP if written from the ground up.

The only issue is it's written entirely in C, so an interface layer between Rust and C will be required. Using the [bindgen library](https://docs.rs/bindgen/latest/bindgen/) we can generate Rust FFI code that will call C functions.

Without bindgen we would have to manage the FFI bindings for each supported platform. Using bindgen also came with a truckload of issues addressed in a later section.&#x20;



```c
#include <stdint.h>

typedef struct Complex {
  int32_t r;
  int32_t i;
} Complex;

void addComplex(Complex *a, Complex *b);
void subComplex(Complex *a, Complex *b);
```

The FFI bindings generated by bindgen will live inside the build folder. VSCode is even nice enough to provide full e2e IntelliSense (assuming the `rust-lang.rust-analyzer` extension is installed).

```rust
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Complex {
    pub r: i32,
    pub i: i32,
}

extern "C" {
    pub fn addComplex(a: *mut Complex, b: *mut Complex);
}

extern "C" {
    pub fn subComplex(a: *mut Complex, b: *mut Complex);
}
```

Now we can start writing Rust code referring to native C code.

```rust
const a: Complex = Complex { r: 10, i: 10 };
const b: Complex = Complex { r: 1, i: -10 };
const z: Complex = Complex { r: a.r + b.r, i: a.i + b.i };
```
