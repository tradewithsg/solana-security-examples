# Unsafe CPI Assumptions

## What is the vulnerability?

Unsafe CPI assumptions occur when a program calls another program
and blindly assumes the CPI behaved as expected.

In this example, the program transfers tokens via CPI and updates
internal accounting without verifying the actual token movement.

---

## How attackers exploit this

An attacker can exploit this by:

1. Triggering a CPI under unexpected conditions
2. Causing token movement to differ from assumptions
3. Desynchronizing internal state from real balances

Once internal accounting is incorrect, further exploits become possible.

---

## Real-world context

Unsafe CPI assumptions have appeared in Solana exploits involving
token vaults, DeFi protocols, and reward systems.

Programs that trusted CPI outcomes without verification
ended up with broken accounting or unintended fund loss.

---

## Why Anchor doesn’t protect you automatically

Anchor helps construct CPI calls but does not verify outcomes.

Once a CPI returns successfully, Anchor does not check
whether the external program performed the intended action.

It is the developer’s responsibility to validate CPI effects.

---

## How the fix works

The secure version verifies the CPI result by:

1. Measuring token balances before and after the CPI
2. Confirming the expected balance change occurred
3. Updating internal state only if verification passes

This ensures internal accounting remains consistent.

---

## Key takeaway

> **Never assume a CPI did what you expected — verify its effects.**

Always validate state changes caused by external programs.
