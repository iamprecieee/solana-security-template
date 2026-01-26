# Example 5: Integer Overflow

Integer overflow occurs when an arithmetic operation results in a value that is outside the range that can be represented by the integer type. In Solana, this can lead to unexpected state transitions and fund manipulation.

---

## Vulnerability

Rust's default behavior for integer arithmetic in release mode is to wrap around on overflow. For example, adding 1 to `u8::MAX` results in `0`. If a program uses standard arithmetic operators (+, -, *) without checks, an attacker can trigger this wrapping to bypass logical checks or inflate balances.

### Impact Summary

| Aspect | Description |
|--------|-------------|
| **Severity** | High |
| **Category** | Arithmetic |
| **Exploit** | Manipulation of counters, balances, or logic via wrapping values |

---

## Bug

The vulnerability arises from using `wrapping_add` or standard arithmetic operators in a context where overflow is possible and not intended.

<details>
<summary><strong>View Vulnerable Code</strong></summary>

```rust
pub fn vulnerable_add(ctx: Context<VulnerableAdd>, amount: u64) -> Result<()> {
    // BUG: Unchecked addition can wrap around on overflow
    ctx.accounts.counter.value = ctx.accounts.counter.value.wrapping_add(amount);
    Ok(())
}
```

</details>

> [!WARNING]
> If a counter currently holds `u64::MAX` and an attacker adds `1`, the counter will wrap to `0`. If this counter represents a balance or a limit, the consequences can be catastrophic for the program's logic.

---

## Fix

Use checked arithmetic methods (e.g., `checked_add`, `checked_sub`) which return an `Option`. You should handle the `None` case by returning an error.

<details open>
<summary><strong>View Secure Fix</strong></summary>

```rust
pub fn secure_add(ctx: Context<SecureAdd>, amount: u64) -> Result<()> {
    ctx.accounts.counter.value = ctx
        .accounts
        .counter
        .value
        .checked_add(amount)
        .ok_or(ErrorCode::Overflow)?;
    Ok(())
}
```

</details>

> [!IMPORTANT]
> **Key Mitigation**: Always use checked arithmetic (`checked_add`, `checked_sub`, `checked_mul`, `checked_div`) for any operation involving user-provided input or critical state variables.

---

## Project Files

| File | Description |
|------|-------------|
| [Implementation](../programs/solana-security-template/src/instructions/integer_overflow.rs) | Rust logic for vulnerability and fix |
| [Tests](../tests/src/test_integer_overflow/mod.rs) | Exploit verification tests |

---

## References

- [Rust Book: Integer Overflow](https://doc.rust-lang.org/book/ch03-02-data-types.html#integer-overflow)
- [Solana Checked Arithmetic Best Practices](https://docs.solana.com/developing/programming-model/overview#checked-arithmetic)
