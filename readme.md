# Awesome pinger

A simple HTTP probe service that is core of our awesome monitoring stuff ;)

---

## Features

- Listens on `0.0.0.0:8080` for HTTP GET requests from monitoring server.
- Knows how to ping machines inside the network.

---

## Prerequisites

- Rust toolchain (rustc & Cargo)
- musl-tools (for static builds, optional)
- Bash (for running the ping command)

---

## Setup & Build

1. **Clone the repository**

   ```bash
   git clone https://github.com/veldrane/awesome-pinger
   cd awesome-pinger
   ```

2. **Add the MUSL target for static linking (optional)**

   ```bash
   rustup target add x86_64-unknown-linux-musl
   ```

3. **Build**

   - Dynamic build (glibc):
   ```bash
   make
   sudo make install
   ```

- Static build (musl):
   ```bash
   make static
   sudo make install
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
/usr/local/bin/pinger

# Or static
/usr/local/bin/pinger-static
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
