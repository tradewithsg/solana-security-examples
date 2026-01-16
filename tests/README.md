# Tests

This folder contains **exploit demonstration tests** for the Solana security examples in this repository.

The goal of these tests is **not full protocol coverage**, but to clearly and practically demonstrate how real vulnerabilities can be exploited and how the corresponding fixes prevent those exploits.

---

## Purpose of the tests

Each test is designed to:

- Reproduce a **realistic attack scenario**
- Show that the vulnerable version **accepts malicious behavior**
- Prove that the secure version **correctly rejects the same attack**
- Confirm that legitimate behavior still works as intended

These tests serve as **executable proof** of the security issues discussed in each example.

---

## Scope and philosophy

The tests in this repository intentionally focus on:

- Correctness over completeness
- Clarity over complexity
- Realistic attacker behavior
- Minimal setup required to demonstrate the issue

They are **not** intended to represent production test suites for full applications, but rather **educational security demonstrations**.

---

## Current coverage

At the moment, the following exploit scenarios are covered:

- **Missing Account Validation**
  - Unauthorized user successfully mutates another user’s state in the vulnerable program
  - The same action is rejected in the secure program
  - Authorized users can still perform valid updates

Additional tests may be added to cover other vulnerability categories such as unsafe arithmetic or CPI assumptions.

---

## Notes on test structure

- Tests use standard **Anchor testing conventions**
- Separate programs are tested independently to avoid state leakage
- `createProfile` or similar setup instructions may be used **only to initialize test state**
- The security examples themselves focus on the vulnerable and secure instructions under discussion

---

## How to run the tests

From the project root:

```bash
anchor test
