# Fast Spam Requester

A lightweight, asynchronous HTTP benchmarking and load-testing CLI tool built in Rust. It generates controlled concurrent traffic, handles HTTP/HTTPS targets, collects real-time error breakdowns, and shuts down gracefully without dropping active connections.

---

## Features

* **Asynchronous Engine:** Powered by Tokio and Reqwest with Keep-Alive connection pooling.
* **Strict Concurrency Control:** Regulated via tokio semaphores to prevent connection storms.
* **Lock-Free Metrics:** Fast metric collection using `std::sync::atomic::AtomicU64` with relaxed ordering.
* **Zero-Copy Payloads:** Request bodies are wrapped in `bytes::Bytes` for low-overhead multi-task cloning.
* **Cross-Platform:** Single self-contained binary for Linux, Windows, and macOS.

---

## Installation

### Pre-built Binaries

Download the appropriate executable for your platform from the releases:

* **Windows (x86_64):** `fast-spam-requester-x86_64-pc-windows-msvc.exe`
* **Linux (x86_64):** `fast-spam-requester-x86_64-unknown-linux-gnu`
* **macOS (Apple Silicon):** `fast-spam-requester-aarch64-apple-darwin`

On Linux and macOS, make the binary executable before running:

```bash
chmod +x fast-spam-requester-x86_64-unknown-linux-gnu
./fast-spam-requester-x86_64-unknown-linux-gnu --help

```

### Build from Source

```bash
git clone https://github.com/your-username/fast-spam-requester.git
cd fast-spam-requester
cargo build --release

```

The optimized binary will be placed at `target/release/fast-spam-requester` (or `.exe` on Windows).

---

## Usage

```bash
fast-spam-requester [OPTIONS] --url <URL>

```

### Options

| Flag | Short | Description | Default |
| --- | --- | --- | --- |
| `--url` | `-u` | Target HTTP/HTTPS endpoint | *Required* |
| `--concurrency` | `-c` | Maximum number of concurrent connections | `10` |
| `--duration` | `-d` | Test duration (e.g. `10s`, `1m`, `300ms`) | `10s` |
| `--header` | `-H` | Custom header (`Header-Name: Value`), repeatable | *None* |
| `--body` | `-b` | String request payload sent via POST | *None* |

---

## Examples

### 1. Basic 5-second Benchmark

```bash
fast-spam-requester -u https://api.example.com/health -c 20 -d 5s

```

### 2. POST Request with JSON Body and Auth Header

```bash
fast-spam-requester \
  -u https://api.example.com/v1/orders \
  -c 50 \
  -d 30s \
  -H "Authorization: Bearer my-secret-token" \
  -H "Content-Type: application/json" \
  -b '{"item_id": 42, "quantity": 1}'

```

---

## Sample Output

```text
Running test for 5s...

Duration:           5.12s
Requests per sec:   195.31
Total requests:     1000
Successful (2xx):   984
Client errors (4xx):0
Server errors (5xx):16
Network failures:   0

```

---

## License

MIT Licens
