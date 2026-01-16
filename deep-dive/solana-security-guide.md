# Solana Security Deep Dive  
## Understanding Real Vulnerabilities Beyond the Framework

---

## Introduction

Most Solana exploits do not come from advanced cryptography failures or novel attack techniques.  
They come from **simple, repeated logic mistakes** made during program development.

These mistakes are often subtle, easy to overlook, and frequently misunderstood by developers who assume that frameworks like Anchor automatically provide security guarantees.

This guide explains **why these vulnerabilities happen**, **how Solana really works under the hood**, and **how developers should think about security when writing programs**.

The goal is not to teach syntax, but to build the correct **mental model** for secure Solana development.

---

## Why most Solana hacks are simple mistakes

Many Solana hacks share the same root causes:

- Trusting user-supplied accounts
- Assuming ownership or authority without enforcing it
- Performing unchecked arithmetic
- Assuming external programs (CPIs) behave as expected
- Using untyped or loosely validated accounts

These issues are not theoretical. They appear repeatedly in real exploits because Solana programs operate in a **hostile execution environment** where attackers fully control transaction inputs.

Solana does not prevent logic bugs.  
It only enforces rules that developers explicitly define.

---

## How Solana accounts REALLY work

To understand Solana security, you must understand this rule:

> **All accounts passed into a transaction are controlled by the user unless proven otherwise.**

Key implications:

- A program does **not** automatically know which account is “correct”
- A program does **not** automatically know who is authorized
- A program does **not** automatically verify relationships between accounts

When your instruction receives accounts, Solana guarantees only that:
- The accounts exist
- They are readable or writable as requested
- Signatures are valid for declared signers

Everything else ownership, authority, intent is **your responsibility**.

This is why missing account validation and missing authority checks are so dangerous.

---

## Why Anchor is not “auto-secure”

Anchor is a **developer productivity framework**, not a security engine.

Anchor provides:
- Account deserialization
- Type safety (when using `Account<T>`)
- Constraint helpers
- CPI helpers

Anchor does **not**:
- Decide who should be authorized
- Enforce ownership by default
- Prevent logic errors
- Automatically make instructions safe

Anchor will happily execute insecure logic if the developer does not explicitly define constraints.

This is why many insecure programs are written using Anchor not because Anchor is unsafe, but because **security rules are application-specific**.

---

## Common mental traps developers fall into

### 1. “If it deserializes, it must be valid”

False.

Anchor deserializing an account does **not** mean:
- The account belongs to the expected user
- The account is the correct one
- The account should be mutable

Always validate relationships explicitly.

---

### 2. “I stored an owner field, so it’s secure”

False.

Storing an authority in state does nothing unless you:
- Require the authority to be a signer
- Enforce the relationship at runtime

An unenforced authority is equivalent to no authority at all.

---

### 3. “Rust is safe, so arithmetic is safe”

False.

Rust allows unchecked arithmetic using `+` and `-`.
On-chain arithmetic bugs can:
- Wrap values
- Create or destroy funds
- Break invariants permanently

Always use checked arithmetic for balances and counters.

---

### 4. “If the CPI succeeded, everything is fine”

False.

A CPI returning `Ok(())` does not guarantee:
- Tokens moved as expected
- State changed as intended
- Accounting remains consistent

Programs must verify **effects**, not just success.

---

### 5. “AccountInfo gives me flexibility”

True — but dangerous.

`AccountInfo` bypasses:
- Ownership checks
- Data layout validation
- Deserialization guarantees

Unless absolutely necessary, `AccountInfo` should be avoided for state.

---

## How to audit your own Solana program

When reviewing your own code (or others’), ask these questions for **every instruction**:

### Account validation
- Can an attacker pass a different account here?
- Are all critical accounts validated with constraints or seeds?
- Are relationships between accounts enforced?

---

### Authority enforcement
- Who is allowed to call this instruction?
- Is the required authority a signer?
- Is authority enforced before state mutation?

---

### Arithmetic safety
- Are all balance changes using checked arithmetic?
- Can user input cause overflow or underflow?

---

### CPI safety
- Do CPIs affect critical state?
- Are CPI effects verified?
- Can internal accounting diverge from reality?

---

### Account typing
- Are typed accounts used wherever possible?
- Is `AccountInfo` truly necessary?
- Are ownership and layout guaranteed?

If you cannot confidently answer these questions, the instruction is likely vulnerable.

---

## Final mindset shift

Secure Solana development requires one key assumption:

> **Assume attackers control everything except what you explicitly validate.**

If a rule is not enforced in code, it does not exist.

Frameworks help reduce mistakes, but **security comes from intent, clarity, and explicit validation** — not abstractions.

---

## Closing note

The examples in this repository are intentionally small and focused, because most real Solana exploits are not complex.

They are simple mistakes made repeatedly.

Understanding these patterns and the mindset behind them is the first step toward writing truly secure Solana programs.
