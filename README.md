# winploit_proto

**⚠️ For educational use only. Do not use on unauthorized systems.**

`winploit_proto` is a *prototype* reverse shell RAT (Remote Access Tool) made purely for learning and experimentation. It does **not** include any obfuscation or AV evasion, so it **will** likely be flagged by antivirus software.

## 🚧 What it does

- Connects a Rust payload (the "client") to a C2 server (written in Python).
- Allows the server to:
  - Run commands remotely.
  - Change directories.
  - Send kill/exit signals to the payload.
  - (Partially implemented) Enable persistence on Windows.

## 🛠 Components

### 🐍 Server (C2)
- Written in Python.
- Accepts incoming client connections.
- Sends commands and handles structured responses.
- Has colored output and basic command handling (e.g., `clear`, `persist`).

### 🦀 Client (Payload)
- Written in Rust.
- Uses XOR to obfuscate some strings (basic AF, not serious protection).
- Tries to reconnect in a loop if the server is unavailable.
- Runs shell commands, handles `cd`, and responds back with stdout/stderr.

## 🚀 How to run

### Server (Python 3)

```bash
python3 main.py
```

### Payload (Rust)

```bash
cargo build --release
# Run the compiled binary on the target (educational/test machine only)
```

> Make sure the IP and port in the Rust code match your C2 server (default: `10.5.22.214:7878`).

## 🔐 Warning

- No encryption.
- No obfuscation.
- No stealth.
- This is **just a prototype** and **not safe** for actual deployment.

## 📚 For Learning

This project was built to understand how basic reverse shells and C2 infrastructure works. It's a prototype. If you're trying to build something more serious (e.g. red team tools), this is just a starting point.

---

**Disclaimer:** Running this on machines you don't own or have permission to access is illegal and unethical. Don't be that guy.
