# 🧵 Thread-Safe Logging Library in Rust

A high-performance, thread-safe logging library built in Rust using synchronization primitives like `Mutex`, `Arc`, and `Channels`. This project aims to solve the problem of concurrent log writing in multi-threaded applications.

## 📌 Features

- ✅ Thread-safe logging using `Mutex` and `Arc`
- ✅ Asynchronous logging with `Tokio` or `Crossbeam`
- ✅ Support for log levels: INFO, WARN, ERROR, DEBUG, TRACE
- ✅ File-based and console-based output
- ✅ Customizable and lightweight

---

## 🚀 Getting Started

### 📁 Clone the Repository
```bash
git clone https://github.com/your-username/thread-safe-logger.git
cd thread-safe-logger
```
### 🛠 Build and Run
```bash
cargo build
cargo run
```
📦 Dependencies
log – Logging interface
env_logger – Environment-based logger
tokio – Async runtime
crossbeam – Fast multi-threading support
🔍 Real-Time Applications
Distributed Systems
Game Development
Web Servers & APIs
IoT Devices
Logging in High-Concurrency Environments
