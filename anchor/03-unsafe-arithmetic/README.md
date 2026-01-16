# Unsafe Arithmetic

## What is the vulnerability?

Unsafe arithmetic occurs when a program performs numeric operations
without checking for overflow or underflow.

In this example, the vulnerable instruction subtracts an amount
from a wallet balance without verifying that sufficient funds exist.

If the subtraction underflows, the balance wraps to a very large value.

---

## How attackers exploit this

An attacker can:

1. Call the withdraw instruction
2. Specify an amount larger than the wallet balance
3. Trigger an underflow
4. End up with an inflated balance

This exploit requires no special permissions and no complex logic —
only unsafe arithmetic.

---

## Real-world context

Arithmetic bugs are a well-known class of blockchain vulnerabilities.

On Solana, improper handling of arithmetic has caused:
- Broken accounting systems
- Inflation bugs
- Funds being created unintentionally

While Rust provides checked arithmetic methods,
developers must explicitly use them.

---

## Why Anchor doesn’t protect you automatically

Anchor does not enforce safe arithmetic by default.

If a developer uses native `+` or `-` operators,
Anchor will not prevent overflows or underflows.

It is the developer’s responsibility to use safe arithmetic patterns.

---

## How the fix works

The secure version uses `checked_sub` to safely perform subtraction.

If the operation would underflow, the transaction fails
before any state is modified.

This ensures balances remain consistent and predictable.

---

## Key takeaway

> **Never trust arithmetic involving user-controlled values.**

Always use checked arithmetic when updating balances or counters.
