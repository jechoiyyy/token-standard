# Token Standard

ERC-20 inspired token standard built to learn Rust ownership.

## Goals

- Understand Rust ownership, borrowing, and lifetimes ✅
- Implement a production-quality token standard 🚧
- Practice blockchain development patterns 🚧

## Progress

- [x] Project initialization
- [x] Design phase (see `docs/design.md`)
- [x] Core implementation (`new`, `balance_of`, `transfer`)
- [x] Error handling with custom TokenError enum
- [x] Comprehensive test suite (7 tests, 100% pass)
- [ ] Extended features (approve, transfer_from)
- [ ] Performance benchmarking
- [ ] Documentation (rustdoc)

## Project Structure
```
token-standard/
├── src/
│   └── lib.rs          # Core implementation + tests
├── docs/
│   └── design.md       # Design decisions and rationale
├── Cargo.toml
└── README.md
```

## Quick Start
```bash
# Run tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Build
cargo build

# Check without building
cargo check
```

## Features Implemented

### Core Functions
- `new(creator, initial_supply)` - Create new token with initial supply
- `total_supply()` - Get total token supply
- `balance_of(address)` - Query balance of an address
- `transfer(from, to, amount)` - Transfer tokens between addresses

### Error Handling
- `InsufficientBalance` - Not enough tokens to transfer
- `SelfTransfer` - Cannot transfer to self
- `ZeroAmount` - Cannot transfer zero tokens
- `BalanceOverFlow` - Arithmetic overflow protection

## Testing
```bash
cargo test
```

Current test coverage: **7 tests, 100% pass rate**

Tests include:
- Happy path scenarios
- All error cases
- Edge cases (overflow, non-existing addresses)

## Learning Notes

### Day 1 (2025-02-04)
**Topics covered:**
- Git workflow and conventional commits
- Rust ownership and borrowing
- Result type and error handling with `?` operator
- HashMap operations
- Testing with `#[cfg(test)]`
- Integer overflow handling with `checked_add()`

**Key insights:**
- Ownership prevents data races at compile time
- `&mut` ensures exclusive access for modifications
- Error types with data (e.g., `InsufficientBalance { required, available }`) provide better debugging
- Test-only code should be isolated with `#[cfg(test)]`

**Time spent:** ~4 hours

## Next Steps

1. Add `approve()` and `transfer_from()` for allowance pattern
2. Add benchmarking to compare HashMap vs BTreeMap
3. Write rustdoc documentation
4. Research Commonware blockchain library integration

## References

- [Rust Book Chapter 4: Ownership](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html)
- [ERC-20 Token Standard](https://eips.ethereum.org/EIPS/eip-20)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)