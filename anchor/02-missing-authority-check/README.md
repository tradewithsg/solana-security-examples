# Missing Authority Check

## What is the vulnerability?

Missing authority checks occur when a program performs privileged actions
without verifying that the caller is authorized to do so.

In this example, a vault has a defined owner, but the withdraw instruction
does not require the owner to sign the transaction.

As a result, any user can withdraw funds from the vault.

---

## How attackers exploit this

An attacker can:

1. Identify a vault account
2. Call the withdraw instruction
3. Drain funds without owning the vault

The program modifies the correct account,
but fails to verify **who is calling the instruction**.

---

## Real-world context

Missing authority checks have appeared in real Solana exploits involving
vaults, staking programs, and treasury contracts.

In several incidents, programs stored an authority field in state
but failed to enforce signer validation during sensitive operations.

Attackers exploited this by calling privileged instructions directly,
without needing to bypass any cryptographic protections.

---

## Why Anchor doesn’t protect you automatically

Anchor does not assume who should be authorized to perform actions.

While it provides tools like `Signer<'info>` and `has_one`,
it is the developer’s responsibility to define and enforce authority rules.

If no signer is required, Anchor will allow any caller.

---

## How the fix works

The secure version enforces authority in two ways:

1. `Signer<'info>`  
   Requires the vault owner to explicitly approve the transaction

2. `has_one = owner`  
   Ensures the vault belongs to the signer

Together, these checks prevent unauthorized withdrawals.

---

## Key takeaway

> **Storing an authority is not enough — you must enforce it.**

Always require and validate the signer for privileged operations.
