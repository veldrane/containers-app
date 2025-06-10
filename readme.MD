# Vulnerable Ping Web Service

A simple Rust-based HTTP service that accepts an `ip` parameter via the query string, URL-decodes it, and executes a ping command (or any provided shell command) against that parameter. This intentionally vulnerable example illustrates how improper input sanitization can lead to command injection.

---

## Features

- Listens on `0.0.0.0:8080` for HTTP GET requests.
- Parses query string, URL-decodes values (e.g., `%20` → space, `%3B` → `;`, `%2F` → `/`).
- Passes the decoded string directly into a shell command (e.g., `ping`, `ip a`).
- Demonstrates the danger of shelling out without validation.

---

## Prerequisites

- Rust toolchain (rustc & Cargo)
- musl-tools (for static builds, optional)
- Bash (for running the ping command)

---

## Setup & Build

1. **Clone the repository**

   ```bash
   git clone https://github.com/yourname/vulnerable-ping-service.git
   cd vulnerable-ping-service
   ```

2. **Add the MUSL target for static linking (optional)**

   ```bash
   rustup target add x86_64-unknown-linux-musl
   ```

3. **Build**

   - Dynamic build (glibc):
     ```bash
     cargo build --release
     ```

   - Static build (musl):
     ```bash
     cargo build --release --target x86_64-unknown-linux-musl
     ```

4. **Strip binary (optional)**

   ```bash
   strip target/x86_64-unknown-linux-musl/release/pinger
   ```

---

## Usage

Run the server:

```bash
# Dynamic
./target/release/pinger

# Or static
./target/x86_64-unknown-linux-musl/release/pinger
```

Send requests:

```bash
# Ping localhost
curl 'http://127.0.0.1:8080?ip=127.0.0.1'       

# Run another command (e.g., list interfaces)
curl 'http://127.0.0.1:8080?ip=127.0.0.1%3Bip%20a'  
```

The response will be the raw output of the executed command, URL-escaped in JSON or plain text depending on your server settings.

---

## Security Warning

**This service is intentionally insecure.**

- **DO NOT** expose it to untrusted networks.
- **DO NOT** use in production or with privileged commands.
- Demonstrates command injection vulnerabilities when input is not sanitized.

To secure it, escape or whitelist inputs, avoid direct shell execution, and prefer using libraries or system calls instead of `bash -c`.

---

## License

This project is released under the MIT License.
