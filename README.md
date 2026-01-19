Solana programs fail in predictable ways. After reviewing a number of exploits, similar patterns keep appearing: unchecked accounts, missing validations, arithmetic bugs. This guide walks through five(5) common vulnerabilities with working exploit code.

Each example includes a vulnerable version that compiles and runs, a secure fix, and tests that prove the exploit works.

---

## 1. [Duplicate Account Aliasing](docs/duplicate-account.md)

**What happens:** A program accepts two mutable account parameters (source and destination). An attacker passes the same account for both.

**The bug:**

```rust
pub fn vulnerable_transfer(ctx: Context<VulnerableTransfer>, amount: u64) -> Result<()> {
    // Cache both balances
    let source_balance = ctx.accounts.source.balance;
    let dest_balance = ctx.accounts.destination.balance;

    // Write both
    ctx.accounts.source.balance = source_balance.saturating_sub(amount);
    ctx.accounts.destination.balance = dest_balance.saturating_add(amount);
    Ok(())
}
```

If source and destination are the same account with balance 100:
- Source reads 100, writes 50
- Destination reads 100 (stale), writes 150
- Final balance: 150 (tokens created from nothing)

**The fix:** Add a constraint that rejects aliased accounts:

```rust
#[account(mut, constraint = source.key() != destination.key() @ ErrorCode::DuplicateAccount)]
pub destination: Account<'info, Wallet>,
```

---

## 2. [Type Confusion](docs/type-confusion.md)

**What happens:** A program reads raw bytes without checking the account discriminator. An attacker substitutes a different account type with matching byte layout.

**The bug:**

```rust
#[derive(Accounts)]
pub struct VulnerableAdmin<'info> {
    #[account(mut)]
    pub admin_config: AccountInfo<'info>,  // No type check
    pub signer: Signer<'info>,
}

pub fn vulnerable_admin_action(ctx: Context<VulnerableAdmin>) -> Result<()> {
    let data = ctx.accounts.admin_config.try_borrow_data()?;
    let authority = Pubkey::try_from(&data[8..40])?;  // Reads bytes 8-40 as authority
    require!(authority == ctx.accounts.signer.key(), ErrorCode::Unauthorized);
    Ok(())
}
```

An attacker creates a `UserData` account where they control the `owner` field. Since `owner` sits at the same byte offset as `authority`, the program accepts it.

**The fix:** Use typed accounts:

```rust
pub admin_config: Account<'info, AdminConfig>,  // Checks 8-byte discriminator
```

---

## 3. [Arbitrary CPI](docs/arbitrary-cpi.md)

**What happens:** A program invokes another program without validating the program ID. An attacker substitutes a malicious program.

**The bug:**

```rust
#[derive(Accounts)]
pub struct VulnerableCpi<'info> {
    #[account(mut)]
    pub from: Signer<'info>,
    #[account(mut)]
    pub to: SystemAccount<'info>,
    pub system_program: UncheckedAccount<'info>,  // No validation
}
```

The attacker passes their own program as `system_program`. That program can do anything with the provided accounts.

**The fix:** Use Anchor's `Program` wrapper:

```rust
pub system_program: Program<'info, System>,  // Validates program ID
```

---

## 4. [Closing Account Theft](docs/closing-account.md)

**What happens:** When closing an account, lamports must go somewhere. If the destination is not validated, an attacker redirects the refund.

**The bug:**

```rust
#[derive(Accounts)]
pub struct VulnerableClose<'info> {
    #[account(mut, has_one = authority)]
    pub data_account: Account<'info, DataAccount>,
    pub authority: Signer<'info>,
    #[account(mut)]
    pub destination: AccountInfo<'info>,  // Unvalidated
}
```

The authority closes the account, but lamports go to `destination` (which the attacker controls).

**The fix:** Use Anchor's `close` constraint:

```rust
#[account(mut, has_one = authority, close = authority)]
pub data_account: Account<'info, DataAccount>,
```

---

## 5. [Integer Overflow](docs/integer-overflow.md)

**What happens:** Arithmetic operations wrap around on overflow. An attacker triggers overflow to manipulate values.

**The bug:**

```rust
pub fn vulnerable_add(ctx: Context<VulnerableAdd>, amount: u64) -> Result<()> {
    ctx.accounts.counter.value = ctx.accounts.counter.value.wrapping_add(amount);
    Ok(())
}
```

If `counter.value` is `u64::MAX - 10` and `amount` is 100, the result wraps to 89.

**The fix:** Use checked arithmetic:

```rust
ctx.accounts.counter.value = ctx.accounts.counter.value
    .checked_add(amount)
    .ok_or(ErrorCode::Overflow)?;
```

---

## Running the Examples

Clone the repository and run:

```bash
anchor build
cargo test --manifest-path tests/Cargo.toml
```

Each vulnerability has two tests:
1. `test_vulnerable_*` - Proves the exploit works
2. `test_secure_*` - Proves the fix blocks it

---

## Key Takeaways

| Vulnerability | Root Cause | Fix |
|--------------|------------|-----|
| Duplicate Account | No aliasing check | `constraint = a.key() != b.key()` |
| Type Confusion | `AccountInfo` skips discriminator | Use `Account<T>` |
| Arbitrary CPI | `UncheckedAccount` for programs | Use `Program<T>` |
| Closing Theft | Unvalidated refund destination | `close = authority` |
| Integer Overflow | Unchecked arithmetic | `checked_add()` |

These five patterns cover most Solana exploits. The fixes are simple: use Anchor's type system. When you see `AccountInfo` or `UncheckedAccount`, verify that the program explicitly validates what Anchor would check automatically.

## Project Structure

```
programs/solana-security-template/src/
├── instructions/
│   ├── duplicate_account.rs   # Example 1
│   ├── type_confusion.rs      # Example 2
│   ├── arbitrary_cpi.rs       # Example 3
│   ├── closing_account.rs     # Example 4
│   └── integer_overflow.rs    # Example 5
├── error.rs
└── lib.rs

tests/src/
├── test_duplicate_account/
├── test_type_confusion/
├── test_arbitrary_cpi/
├── test_closing_account/
└── test_integer_overflow/

docs/
├── duplicate-account.md
├── type-confusion.md
├── arbitrary-cpi.md
├── closing-account.md
└── integer-overflow.md
```

## References

- [Sealevel Attacks](https://github.com/coral-xyz/sealevel-attacks)
- [Anchor Documentation](https://www.anchor-lang.com/)