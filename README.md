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

### ERC-20 Allowance Pattern ← 새 섹션 추가!
- `approve(owner, spender, amount)` - Authorize spender to use tokens
- `allowance(owner, spender)` - Query approved amount
- `transfer_from(spender, from, to, amount)` - Transfer on behalf of owner

### Error Handling
- `InsufficientBalance` - Not enough tokens to transfer
- `SelfTransfer` - Cannot transfer to self
- `ZeroAmount` - Cannot transfer zero tokens
- `BalanceOverFlow` - Arithmetic overflow protection

## Testing
```bash
cargo test
```

Current test coverage: **16 tests, 100% pass rate** ← 업데이트

### Test Categories

**Balance Operations (3 tests)**
- Creating tokens
- Querying existing/non-existing addresses

**Direct Transfers (5 tests)**
- Success case
- Insufficient balance
- Self-transfer prevention
- Zero amount prevention
- Overflow protection

**Allowance Management (4 tests)**
- Approve success
- Self-approval prevention
- Zero amount approval (revocation)
- Overwriting allowances

**Delegated Transfers (4 tests)**
- transfer_from success
- Insufficient allowance
- Insufficient balance (with allowance)
- Allowance updates after transfer

## Learning Notes

### Day 1 (2025-02-05) ← 날짜 수정
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
- Error types with data provide better debugging
- Test-only code should be isolated with `#[cfg(test)]`

**Time spent:** ~4 hours

---

### Day 2 (2025-02-06) ← 새로 추가
**Topics covered:**
- Commonware blockchain framework architecture
- Framework comparison (Commonware vs Cosmos SDK vs Substrate)
- Performance benchmarking with Criterion
- `black_box` and compiler optimization
- `iter_batched` for stateful benchmarks

**Key insights:**
- Commonware's modular "anti-framework" philosophy
- B-Harvest's focus on low-level blockchain R&D
- Early return pattern saves 54% execution time
- Benchmark-driven development reveals bottlenecks

**Time spent:** ~4 hours

---

### Day 3 (2025-02-07) ← 새로 추가
**Topics covered:**
- ERC-20 allowance pattern
- Tuple keys in HashMap
- transfer_from delegation mechanism
- Test-driven development (TDD) approach

**Key insights:**
- Allowance enables DeFi protocols (DEX, lending, staking)
- `(owner, spender)` tuple key design trade-offs
- TDD catches bugs during implementation (not after!)
- approve() surprisingly has minimal error cases

**Bugs found & fixed:**
- `test_balance_of_non_existing_address` assertion error
- `transfer_from` updating wrong allowance key

**Time spent:** ~2.5 hours

---

### Week 1 Summary
**Total time:** ~10.5 hours
**Lines of code:** ~350
**Tests written:** 16 (100% pass)
**Documents created:** 4
- README.md
- docs/design.md
- docs/commonware-research.md
- docs/benchmark-results.md

**Achievement:** ✅ Fully functional ERC-20 compatible token

## Next Steps

### Week 2: AMM (Automated Market Maker)
- Constant product formula (x * y = k)
- Liquidity pools
- Swap mechanism
- Price impact calculation

### Week 3: Lending Protocol
- Collateral management
- Interest rate calculation
- Liquidation logic

### Week 4: Integration
- Commonware runtime integration
- Real blockchain deployment
- Cross-contract interactions

### Future Optimizations
- Address type: String → [u8; 32]
- HashMap benchmarking vs BTreeMap
- Gas optimization for Solana/Cosmos

## References

- [Rust Book Chapter 4: Ownership](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html)
- [ERC-20 Token Standard](https://eips.ethereum.org/EIPS/eip-20)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)