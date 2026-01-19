# Example 5: Integer Overflow

## Vulnerability

Rust's default integer arithmetic in release mode wraps on overflow. If unchecked, this allows attackers to manipulate values in unexpected ways.

## The Bug

```rust
pub fn vulnerable_add(ctx: Context<VulnerableAdd>, amount: u64) -> Result<()> {
    // BUG: Unchecked addition can wrap around on overflow
    ctx.accounts.counter.value = ctx.accounts.counter.value.wrapping_add(amount);
    Ok(())
}
```

If `counter.value` is near `u64::MAX`, adding a large `amount` wraps around to a small number.

## The Fix

Use checked arithmetic that returns an error on overflow:

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

Alternatives:
- `saturating_add()` - Clamps at max value instead of erroring
- Anchor's `#[overflow-checks]` feature - Enables overflow checks for entire program

## Files

- [Vulnerable + Secure Implementation](../programs/solana-security-template/src/instructions/integer_overflow.rs)
- [Tests](../tests/src/test_integer_overflow/mod.rs)

## Reference

- [Rust Book - Integer Overflow](https://doc.rust-lang.org/book/ch03-02-data-types.html#integer-overflow)
