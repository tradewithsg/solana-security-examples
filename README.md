# 🛡️ Solana Security Examples: Vulnerable vs Secure Patterns

## Overview

Security remains one of the biggest challenges in Solana program development.
Many real-world exploits do **not** come from advanced cryptography or complex attacks, but from **simple logic mistakes** such as:

* Missing account validation
* Incorrect authority checks
* Unsafe arithmetic
* Misunderstood CPI behavior
* Account type confusion

Frameworks like **Anchor** and **Pinocchio** provide powerful abstractions, but they **do not automatically make programs secure**. Developers must still understand *why* a pattern is dangerous and *how* to fix it correctly.

This repository is an **educational security reference** for Solana developers.
Each example deliberately contrasts **vulnerable code** with a **secure alternative**, making security concepts practical, obvious, and easy to reason about.

---

## 🎯 Purpose of This Repository

The goal of this project is to:

* Teach **common Solana security pitfalls** using small, focused examples
* Show **realistic vulnerable patterns** that appear in production programs
* Demonstrate **correct fixes**, not just theoretical advice
* Help developers build a **security mindset**, not just write code that “works”

This repository is designed to be read, studied, and referenced not deployed as-is.

---

## 👥 Who This Is For

This repository is useful for:

* Developers **learning Solana or Anchor**
* Builders transitioning from Web2 or EVM-based chains
* Developers who want to **avoid common exploit patterns**
* Auditors or reviewers looking for **clear reference examples**

A basic understanding of:

* Rust
* Solana accounts
* Anchor concepts

is helpful but not strictly required.

---

## 🧩 How the Examples Are Structured

Each security example follows the **same clear structure**:

```text
anchor/
└── 01-example-name/
    ├── README.md        # Explanation of the vulnerability
    ├── vulnerable.rs   # Intentionally insecure code
    └── secure.rs       # Corrected, secure version
```

For every example, you will find:

* **A vulnerable instruction**
  Demonstrates a realistic security mistake

* **A secure version of the same logic**
  Fixes the issue using proper checks and constraints

* **Inline comments**
  Explain:

  * What went wrong
  * Why it is dangerous
  * How the fix works

The focus is **clarity over complexity**.

---

## 🔍 Vulnerabilities Covered

This repository currently covers the following real-world Solana security patterns:

1. **Missing Account Validation**
   Mutating accounts without verifying ownership or relationships

2. **Missing Authority Checks**
   Allowing privileged actions without enforcing signer requirements

3. **Unsafe Arithmetic**
   Overflows and underflows leading to balance manipulation

4. **Unsafe CPI Assumptions**
   Blindly trusting external program behavior

5. **Account Confusion**
   Misuse of `AccountInfo` leading to fake or malformed accounts

Each example is intentionally minimal and focused on **one core mistake**.

---

## 📖 Deep-Dive Security Guide

In addition to code examples, this repository includes a written deep-dive:

```text
deep-dive/solana-security-guide.md
```

This guide explains:

* Why most Solana exploits are logic bugs
* How Solana’s account model impacts security
* Common mental traps developers fall into
* How to reason about program safety before deployment

---

## ▶️ How to Read or Use This Repository

You do **not** need to deploy or run these programs.

Recommended approach:

1. Start with an example folder
2. Read the `README.md` for context
3. Study `vulnerable.rs`
4. Compare it with `secure.rs`
5. Read the inline comments carefully

The goal is understanding **why** the fix works not memorizing patterns blindly.

---

## ⚠️ Disclaimer

All vulnerable code in this repository is **intentional** and for **educational purposes only**.
Do **not** use vulnerable patterns in production programs.

---

## 🤝 Contribution & Scope

This project is intentionally small and focused.
Clarity, correctness, and explanation matter more than scale.

---

### Final Note

If you can understand every example in this repository, you will already be ahead of many Solana developers in terms of security awareness.
