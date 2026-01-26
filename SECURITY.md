# Security Policy

## Threat Model

This project is an **educational repository** designed to showcase vulnerabilities. The "vulnerable" code is **intentionally insecure** and should never be used in production.

### In Scope

| Component | Protection |
|-----------|------------|
| Secure Fixes | The "secure" versions must correctly mitigate the demonstrated attack. |
| Exploit Tests | Vulnerability tests must accurately demonstrate the exploit. |

### Out of Scope

- Use of the "vulnerable" code in external projects.
- Infrastructure security of the user's development machine.

---

## Implementation Standards

| Component | Choice | Rationale |
|-----------|--------|-----------|
| Framework | Anchor | Standard framework for secure Solana development. |
| Language | Rust | Memory-safe language tailored for Solana. |

---

## Vulnerability Disclosure

While this project contains intentional vulnerabilities for educational purposes, if you find a vulnerability in a **secure** implementation or a flaw in the project's own infrastructure, please report it.

**Email:** emmypresh777@gmail.com

Do not file public issues for critical security vulnerabilities.

---

## Dependencies

Security advisories for dependencies are tracked via `cargo-audit`.

```bash
cargo audit
```
