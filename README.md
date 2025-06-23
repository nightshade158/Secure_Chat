# Secure Chat Project

## Overview

**Secure Chat** is a command-line based, end-to-end encrypted messaging system designed for secure communication, inspired by military-grade requirements. It provides user authentication, encrypted message exchange, and secure storage using modern cryptography and a local SQLite database. The project is implemented in Rust for performance and safety.

## Features

- **User Authentication:** Secure signup and login with password protection.
- **End-to-End Encryption:** Messages are encrypted using AES-256-GCM with user-provided passphrases.
- **Message Storage:** Encrypted messages are stored in a local SQLite database until read.
- **Unread Message Notification:** Users are notified of unread messages upon login.
- **Message Deletion:** Messages are deleted from the database after being read and decrypted.
- **Command-Line Interface:** Intuitive, styled CLI for ease of use.

## Installation

### Prerequisites
- [Rust](https://www.rust-lang.org/tools/install) (edition 2021 or later)
- SQLite3 (for database inspection, optional)

### Steps
1. **Clone the repository:**
   ```sh
   git clone <repository-url>
   cd secure-chat-project
   ```
2. **Build the project:**
   ```sh
   cargo build --release
   ```
3. **Run the application:**
   ```sh
   cargo run --release
   ```

## Usage

1. **Start the application:**
   ```sh
   cargo run --release
   ```
2. **Main Menu:**
   - Login
   - Signup
   - Exit
3. **After login:**
   - Send Message: Enter recipient, message, and a secret passphrase for encryption.
   - View Messages: Decrypt unread messages with the correct passphrase. Messages are deleted after reading.
   - Logout

## Architecture

```
+-------------------+
|   User Interface  |
+-------------------+
          |
          v
+-------------------+
|   main.rs         |
| - CLI logic       |
| - User flow       |
+-------------------+
   |           |
   v           v
+---------+ +-------------+
| db.rs   | | encryption.rs|
| - DB    | | - AES-256   |
|   logic | | - Key deriv.|
+---------+ +-------------+
```

- **main.rs:** Handles CLI, user input, and application flow.
- **db.rs:** Manages SQLite database, user registration, authentication, message storage, and retrieval.
- **encryption.rs:** Provides AES-256-GCM encryption/decryption and key derivation from passphrases.

## Security

- **Encryption:** Uses AES-256-GCM for message confidentiality and integrity.
- **Key Derivation:** Passphrases are hashed with SHA-256 to derive encryption keys.
- **Database:** Only encrypted messages and IVs are stored; plaintext is never written to disk.
- **Authentication:** User credentials are stored in the database (consider hashing passwords for production).

## Database Schema

- **users**
  - `id` (INTEGER, PRIMARY KEY)
  - `username` (TEXT, UNIQUE)
  - `password` (TEXT)
- **messages**
  - `id` (INTEGER, PRIMARY KEY)
  - `sender` (TEXT)
  - `receiver` (TEXT)
  - `iv` (TEXT)
  - `encrypted_message` (TEXT)

## Dependencies

- [aes-gcm](https://crates.io/crates/aes-gcm) - AES-GCM encryption
- [rand](https://crates.io/crates/rand) - Random number generation
- [rusqlite](https://crates.io/crates/rusqlite) - SQLite database
- [base64](https://crates.io/crates/base64) - Encoding/decoding
- [sha2](https://crates.io/crates/sha2) - SHA-256 hashing
- [async-std](https://crates.io/crates/async-std), [tokio](https://crates.io/crates/tokio) - Async runtime (future scalability)

## License

This project is licensed under the MIT License. See [LICENSE](LICENSE) for details.

## Contribution

Contributions are welcome! Please open issues or submit pull requests for improvements, bug fixes, or new features.

---

*Copyright (c) 2025 nightshade158*
