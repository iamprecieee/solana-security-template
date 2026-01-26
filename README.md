# Solana Security Template

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/iamprecieee/solana-security-template/blob/40af804fc2c5d31aa94afceb75f6c4ce3c0ab534/LICENSE)
[![Anchor](https://img.shields.io/badge/anchor-0.30.1-blue.svg)](https://www.anchor-lang.com/)

A hands-on guide to Solana security. This project implements common vulnerabilities, their secure fixes, and exploit tests to prove both.

---

## Quick Start

<details open>
<summary><strong>Build from Source</strong></summary>

```bash
anchor build
cargo test --manifest-path tests/Cargo.toml
```

</details>

---

## Vulnerabilities Overview

| # | Vulnerability | Core Bug | Anchor Fix |
|---|---------------|----------|------------|
| 1 | [Duplicate Account](docs/duplicate-account.md) | No aliasing check | `constraint = a.key() != b.key()` |
| 2 | [Type Confusion](docs/type-confusion.md) | `AccountInfo` skips discriminator | Use `Account<T>` |
| 3 | [Arbitrary CPI](docs/arbitrary-cpi.md) | `UncheckedAccount` for programs | Use `Program<T>` |
| 4 | [Closing Theft](docs/closing-account.md) | Unvalidated refund destination | `close = authority` |
| 5 | [Integer Overflow](docs/integer-overflow.md) | Unchecked arithmetic | `checked_add()` |

---

## Detailed Examples

### 1. [Duplicate Account Aliasing](docs/duplicate-account.md)

An attacker passes the same account as both source and destination to inflate balances.

```rust
// The Fix
#[account(mut, constraint = source.key() != destination.key() @ ErrorCode::DuplicateAccount)]
pub destination: Account<'info, Wallet>,
```

### 2. [Type Confusion](docs/type-confusion.md)

Unchecked account types allow substituting one data structure for another.

```rust
// The Fix
pub admin_config: Account<'info, AdminConfig>, // Validates discriminator
```

### 3. [Arbitrary CPI](docs/arbitrary-cpi.md)

Passing a malicious program ID to bypass expected logic.

```rust
// The Fix
pub system_program: Program<'info, System>, // Validates program ID
```

### 4. [Closing Account Theft](docs/closing-account.md)

Redirecting the lamports of a closed account to an attacker's address.

```rust
// The Fix
#[account(mut, has_one = authority, close = authority)]
pub data_account: Account<'info, DataAccount>,
```

### 5. [Integer Overflow](docs/integer-overflow.md)

Standard arithmetic wrapping used to manipulate global state.

```rust
// The Fix
ctx.accounts.counter.value = ctx.accounts.counter.value
    .checked_add(amount)
    .ok_or(ErrorCode::Overflow)?;
```

---

## Project Structure

```
.
├── programs/solana-security-template/src/
│   ├── instructions/    # Vulnerable and Secure implementations
│   ├── error.rs         # Custom error codes
│   └── lib.rs           # Program entry point
├── tests/src/           # Exploit and fix verification tests
└── docs/                # Deep-dive documentation
```

---

[Contributing](../docs/CONTRIBUTING.md) | [Security](../docs/SECURITY.md)