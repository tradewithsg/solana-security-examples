# Missing Account Validation

## What is the vulnerability?

Missing account validation is one of the most common and dangerous security mistakes in Solana programs.

In this example, the vulnerable instruction allows a user profile to be modified **without verifying who owns the profile** or **who is authorized to update it**. The program accepts an account provided by the transaction and mutates it without enforcing any ownership or authority checks.

Because Solana programs must treat all user-supplied accounts as untrusted, this pattern creates a critical security risk.

---

## How attackers exploit this

An attacker can exploit this vulnerability by:

1. Discovering another user’s profile account  
2. Passing that account into the vulnerable instruction  
3. Successfully overwriting the profile data  

Since the program does not enforce ownership or authority, the transaction succeeds even though the caller is **not authorized** to modify the account.

This type of exploit requires no complex attack techniques it relies entirely on the program trusting user-supplied accounts.

---

## Real-world context

Missing account validation has been responsible for **multiple real Solana exploits**.

In several production incidents, programs allowed users to update state or metadata accounts without verifying ownership or authority. The instruction accepted an account from the transaction and mutated it, assuming the caller was authorized.

Attackers exploited this by passing **other users’ accounts** into the instruction. Because the program did not enforce ownership checks or seed constraints, the transaction succeeded and unauthorized state was modified.

This pattern has appeared in NFT programs, staking contracts, and vault-like systems. The root cause in every case was the same: **trusting user-supplied accounts without explicit validation**.

---

## Why Anchor doesn’t protect you automatically

Anchor provides helpful abstractions, including:

- Account deserialization  
- Type safety  
- Basic runtime checks  

However, Anchor does **not**:

- Enforce account ownership  
- Enforce authority or signer requirements  
- Decide who is allowed to mutate state  

These rules are application-specific and must be defined explicitly by the developer. Anchor will happily deserialize and pass accounts to your instruction unless you tell it otherwise.

---

## How the fix works

The secure version introduces two critical protections:

### `has_one = owner`

This constraint ensures that the profile account is explicitly linked to the expected owner. If the account does not belong to the signer, the transaction fails immediately.

### `Signer<'info>`

Requiring the owner to be a signer ensures that only the authorized user can approve modifications to the account.

Together, these constraints prevent unauthorized users from modifying accounts they do not own.

---

## Key takeaway

> **If you do not explicitly validate accounts, Solana will not do it for you.**

Always assume attackers fully control the accounts they pass into your program unless you prove otherwise through explicit validation.

