# Account Confusion (AccountInfo Misuse)

## What is the vulnerability?

Account confusion occurs when a program accepts untyped `AccountInfo`
and assumes it represents valid program state.

`AccountInfo` provides no guarantees about ownership, data layout,
or initialization. If used incorrectly, it allows attackers to
inject fake or malformed accounts.

---

## How attackers exploit this

An attacker can:

1. Create a fake account with arbitrary data
2. Pass it into an instruction expecting valid state
3. Trigger incorrect behavior or overwrite raw data

Because the program does not enforce structure or ownership,
the instruction succeeds with attacker-controlled input.

---

## Real-world context

Account confusion has appeared in real Solana exploits where
programs trusted `AccountInfo` for critical logic.

These issues often arise in:
- Early-stage programs
- Custom serialization logic
- Programs bypassing Anchor’s type system

---

## Why Anchor doesn’t protect you automatically

Anchor provides strong safety guarantees **only when you use typed accounts**.

If a developer chooses to use `AccountInfo`,
Anchor assumes the developer knows what they are doing
and applies no additional safety checks.

---

## How the fix works

The secure version replaces `AccountInfo` with a typed account:

- `Account<'info, T>` enforces ownership
- Data is automatically deserialized
- Layout is validated at runtime

This prevents fake accounts and malformed state from being used.

---

## Key takeaway

> **Avoid `AccountInfo` unless you absolutely need it.**

When possible, always use typed `Account<T>` to benefit from Anchor’s safety guarantees.
