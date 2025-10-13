# `nekotracing`

[![Crates.io](https://img.shields.io/crates/v/nekotracing.svg)](https://crates.io/crates/nekotracing)
[![Docs.rs](https://docs.rs/nekotracing/badge.svg)](https://docs.rs/nekotracing)
[![License](https://img.shields.io/crates/l/nekotracing.svg)](https://github.com/pas2rust/nekotracing/blob/main/LICENSE)
![GitHub top language](https://img.shields.io/github/languages/top/pas2rust/nekotracing?color=orange&logo=rust&style=flat&logoColor=white)
![GitHub stars](https://img.shields.io/github/stars/pas2rust/nekotracing?color=success&style=flat&logo=github)
![GitHub forks](https://img.shields.io/github/forks/pas2rust/nekotracing?color=orange&logo=Furry%20Network&style=flat&logoColor=white)
![Tests](https://raw.githubusercontent.com/pas2rust/badges/main/nekotracing-tests.svg)
![Crates.io downloads](https://img.shields.io/crates/d/nekotracing.svg)
![GitHub last commit](https://img.shields.io/github/last-commit/pas2rust/nekotracing?color=ff69b4&label=update&logo=git&style=flat&logoColor=white)

## What it is and What it Does

**`nekotracing`** is a **Rust** 🦀 **crate** that provides a minimalist solution for **instrumenting** synchronous functions and `async fn` using a single **attribute macro**: `#[nekotrancing]`.

Its primary goal is to generate **execution traces** that record the flow and *timing* of specific instrumented functions.

### 📝 Trace Destination: `tracing.txt`

The central feature of `nekotracing` is that it **appends** these traces, formatted for easy human and machine parsing, to a file named **`tracing.txt`**, which is created in the **root directory of your project**.

### 🔍 Captured Information

For every marked function, the trace records the following data, as seen in the example:

1.  **Timestamp:** The exact moment the function was executed.
2.  **Location:** The file and line number of the code.
3.  **Function Name:** (e.g., `fn sync_user`).
4.  **Arguments:** The serialized values of the input parameters.
5.  **Return Value:** The value or result returned by the function.
6.  **Execution Time:** The total duration of the function call (e.g., `execution time=92.045µs`).

### 🚀 Use Cases

`nekotracing` is ideal for:

* **Quick Profiling:** Measuring the performance of critical code sections, especially within unit tests.
* **Lightweight Diagnostics:** Monitoring runtime behavior for **debugging** or **CI (Continuous Integration)** checks, without the overhead of a more complex tracing system.

---

```rust
use kenzu::Builder;
use nekotracing::nekotrancing;

#[derive(Debug, Builder, Clone)]
pub struct User {
    id: u128,
    name: String,
    age: u8,
}

impl User {
    #[nekotrancing]
    fn sync_user(self) -> Result<Self, String> {
        Ok(self
            .name(UserName::new("sync user")?)
            .age(UserAge::new(18)?)
            .id(UserId::new(0)?))
    }
    #[nekotrancing]
    async fn async_user(self) -> Result<Self, String> {
        Ok(self
            .name(UserName::new("async user")?)
            .age(UserAge::new(19)?)
            .id(UserId::new(1)?))
    }
}

#[test]
fn sync_user() -> Result<(), String> {
    User::new().sync_user()?;
    Ok(())
}

#[tokio::test]
async fn async_user() -> Result<(), String> {
    User::new().async_user().await?;
    Ok(())
}

```
### Example Output (`tracing.txt`)

```text

(2025-10-12 22:13:31.378237998 -03:00 tests/user.rs 12:5)␞fn sync_user␞(self = User { id: 0, name: "", age: 0 }) -> "Ok(User { id: 0, name: \"sync user\", age: 18 })"␞execution time=92.045µs
(2025-10-12 22:13:31.378237974 -03:00 tests/user.rs 19:5)␞async fn async_user␞(self = User { id: 0, name: "", age: 0 }) -> "Ok(User { id: 1, name: \"async user\", age: 19 })"␞execution time=88.799µs

```

<h2 align="center">
  <strong>❤️ Donate</strong>
</h2>

<p align="center">
  <a href="https://github.com/pas2rust/pas2rust/blob/main/pas-monero-donate.png" style="text-decoration:none; color:inherit;">
    <img src="https://img.shields.io/badge/Monero%20QR-FF6600?style=flat&logo=monero&logoColor=white" alt="Monero QR"/>
  </a>
  <a href="https://github.com/pas2rust/pas2rust/blob/main/pas-bitcoin-donate.png" style="text-decoration:none; color:inherit;">
    <img src="https://img.shields.io/badge/BTC%20QR-EAB300?style=flat&logo=bitcoin&logoColor=white" alt="BTC QR"/>
  </a>
  <a href="https://revolut.me/pas2rust" style="text-decoration:none; color:inherit;">
    <img src="https://img.shields.io/badge/Revolut%20QR-Blue?style=flat&logo=revolut&logoColor=white" alt="Revolut QR"/>
  </a>
  <a href="https://wise.com/pay/me/pedroaugustos99" style="text-decoration:none; color:inherit;">
    <img src="https://img.shields.io/badge/Wise%20QR-1CA0F2?style=flat&logo=wise&logoColor=white" alt="Wise QR"/>
  </a>
</p>


---