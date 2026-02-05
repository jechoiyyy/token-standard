# Token Standard - Design Decisions

## 1. Data Structures

### Address Type
**Decision**: Use `String` initially
**Rationale**: 
- Simple for learning phase
- Easy to test and debug
- Human-readable in test output
- Can optimize later to `[u8; 32]` for production

**Trade-offs**:
- Pro: Easy to read, debug, no need for hex encoding
- Con: More memory (24 bytes vs 32 bytes for [u8;32]), slower comparison
- Con: String cloning cost in HashMap operations

**Future considerations**:
- Real blockchain addresses are fixed-size (e.g., 20 bytes for Ethereum)
- `[u8; 32]` or custom Address type for production

### Balance Type
**Decision**: Use `u64`
**Rationale**:
- Sufficient range: 0 to 18,446,744,073,709,551,615
- Good balance between size and capacity
- Standard integer type, well-optimized

**Trade-offs**:
- Pro: Fast arithmetic, Copy trait, no heap allocation
- Con: May overflow (handled with checked_add)
- Con: Smaller than ERC-20's uint256

**Overflow handling**: Use `checked_add()` to detect overflow and return error

### Storage: HashMap
**Decision**: Use `HashMap<Address, Balance>` for balances
**Rationale**:
- O(1) average case for lookups
- Flexible size (grows as needed)

**Trade-offs**:
- Pro: Fast lookups, industry standard
- Con: No ordering (BTreeMap would give sorted addresses)
- Con: Hash function overhead

## 2. Ownership Strategy

### new Function
**Signature**: `fn new(creator: Address, initial_supply: Balance) -> Self`
**Rationale**:
- `creator`: Takes ownership (Address), stored in HashMap
- Returns `Self` (TokenState owns all data)

### total_supply Function
**Signature**: `fn total_supply(&self) -> Balance`
**Rationale**:
- `&self`: Read-only access, no modification
- Returns `Balance`: Copy is cheap (u64 is Copy)
- Note: Removed `get_` prefix to follow Rust conventions

### balance_of Function
**Signature**: `fn balance_of(&self, address: &Address) -> Balance`
**Rationale**:
- `&self`: Read-only, no state modification
- `&Address`: Avoid cloning, just read
- Return `Balance`: Copy is cheap (u64 is Copy)
- Returns `0` for non-existing addresses (blockchain convention)

### transfer Function
**Signature**: `fn transfer(&mut self, from: &Address, to: &Address, amount: Balance) -> Result<(), TokenError>`
**Rationale**:
- `&mut self`: Modifies internal state (balances HashMap)
- `&Address` for from/to: No ownership needed, just read
- `amount: Balance`: Copy trait, no ownership concerns
- Returns `Result<(), TokenError>`: Success returns unit `()`, errors are explicit

**Error handling strategy**:
- Early validation (self-transfer, zero amount)
- Balance checks before state modification
- Overflow protection with `checked_add()`
- Atomic-like behavior (all checks before any modifications)

## 3. Error Handling

### TokenError Enum
**Design**: Custom enum with detailed variants

**Variants**:
1. `InsufficientBalance { required, available }`: 
   - Contains both values for debugging
   - Helps users understand exactly what went wrong

2. `SelfTransfer`: 
   - Prevents no-op transfers
   - Clear error message

3. `ZeroAmount`:
   - Prevents meaningless transactions
   - Gas optimization consideration

4. `BalanceOverFlow`:
   - Critical safety check
   - Prevents arithmetic overflow

**Derives**: `Debug` for printing, `PartialEq` for testing

## 4. Testing Strategy

### Test Coverage
- Unit tests for each function
- Happy path: `test_transfer_success`
- Error cases: All TokenError variants
- Edge cases: Overflow, non-existing addresses

### Test Helpers
- `mint_for_test()`: Creates arbitrary balances for testing
- Marked with `#[cfg(test)]` to prevent production use
- Security: Test-only code doesn't compile in release builds

### AAA Pattern
All tests follow Arrange-Act-Assert pattern for clarity

## 5. Future Enhancements

### Short-term (Week 1-2)
- [ ] Add `approve()` and `transfer_from()` for ERC-20 compatibility
- [ ] Add events/logs for transfers
- [ ] Benchmarking for performance analysis

### Medium-term (Week 3-4)
- [ ] Optimize Address type to `[u8; 32]`
- [ ] Add more sophisticated error types
- [ ] Documentation (rustdoc comments)

### Long-term
- [ ] Integration with real blockchain (Solana/Cosmos)
- [ ] Gas optimization
- [ ] Formal verification considerations

---

---

## 6. ERC-20 Allowance Pattern (Day 3)

### Design Challenge

**Problem**: How to enable DeFi protocols (DEX, lending) to move tokens on behalf of users?

**Naive approach** (doesn't work):
```rust
// DEX directly transfers from user?
token.transfer(&user, &other_user, amount);  // ❌ DEX can't call this!
```

**Solution**: Two-step allowance pattern
1. User approves DEX: `approve(dex, 100)`
2. DEX executes transfer: `transfer_from(user, recipient, 50)`

### Allowance Storage Design

**Decision**: Use `HashMap<(Address, Address), Balance>` with tuple keys

**Alternatives considered**:

#### Option A: Nested HashMap
```rust
allowances: HashMap<Address, HashMap<Address, Balance>>
// allowances[alice][bob] = 100
```
**Pros**: 
- Intuitive structure
- Easy to query "all of Alice's approvals"

**Cons**:
- Double lookup (slower)
- Memory waste when inner HashMap is empty
- More complex to manage

#### Option B: Tuple Key (✅ Chosen)
```rust
allowances: HashMap<(Address, Address), Balance>
// allowances[(alice, bob)] = 100
```
**Pros**:
- Single O(1) lookup
- Memory efficient
- Simple implementation

**Cons**:
- Harder to query "all approvals from Alice"
- But: This query is rarely needed in practice!

#### Option C: Custom Struct Key
```rust
#[derive(Hash, Eq, PartialEq)]
struct AllowanceKey {
    owner: Address,
    spender: Address,
}
```
**Pros**:
- Most explicit
- Easy to extend with metadata

**Cons**:
- Extra type definition
- No practical benefit over tuple

**Final Decision**: Option B (Tuple) for simplicity and performance.

---

### approve() Function Design

**Signature**:
```rust
pub fn approve(
    &mut self,
    owner: &Address,
    spender: &Address,
    amount: Balance
) -> Result<(), TokenError>
```

**Error Cases Analysis**:

| Scenario | Behavior | Rationale |
|----------|----------|-----------|
| `owner == spender` | ❌ Error: `SelfApproval` | Meaningless, likely a bug |
| `amount == 0` | ✅ Success | Valid: revokes approval |
| `amount > u64::MAX` | N/A | Type system prevents this |
| Owner doesn't exist | ✅ Success | approve() doesn't check balance |

**Surprising insight**: approve() almost never fails!

Unlike `transfer`, which validates balances, `approve` just records intent. 
The actual validation happens in `transfer_from`.

**Implementation**:
```rust
if owner == spender {
    return Err(TokenError::SelfApproval);
}
self.allowances.insert((owner.clone(), spender.clone()), amount);
Ok(())
```

---

### transfer_from() Function Design

**Signature**:
```rust
pub fn transfer_from(
    &mut self,
    spender: &Address,
    from: &Address,
    to: &Address,
    amount: Balance
) -> Result<(), TokenError>
```

**Check Order** (Critical for gas efficiency):
```
1. ✅ Self-transfer check (cheap)
2. ✅ Zero amount check (cheap)
3. ✅ Allowance check (HashMap lookup)
4. ✅ Balance check (HashMap lookup)
5. ✅ Overflow check (arithmetic)
6. ✅ Execute transfer (2x HashMap insert)
7. ✅ Update allowance (HashMap insert)
```

**Why this order?**
- **Early returns save computation**: Check cheap validations first
- **Allowance before balance**: If no approval, don't waste time checking balance
- **State changes last**: Only modify state after all validations pass

**Common Bug** (caught during implementation!):
```rust
// ❌ WRONG: Updates wrong allowance
self.allowances.insert((from, to), new_allowance);

// ✅ CORRECT: Updates (owner, spender)
self.allowances.insert((from, spender), new_allowance);
```

**Why this matters**:
- `to` is the recipient, not the spender!
- Caught by `test_transfer_from_updates_allowance`
- Demonstrates value of TDD

---

### Error Type: InsufficientAllowance
```rust
InsufficientAllowance {
    required: Balance,
    available: Balance,
}
```

**Design rationale**:
- Mirrors `InsufficientBalance` structure
- Provides debugging context
- Helps users understand why transaction failed

**Usage example**:
```rust
if current_allowance < amount {
    return Err(TokenError::InsufficientAllowance {
        required: amount,
        available: current_allowance,
    });
}
```

---

### Testing Strategy

**16 tests total** covering:

1. **Approve operations** (4 tests)
   - Success case
   - Self-approval prevention
   - Zero amount (revocation)
   - Overwriting existing approval

2. **transfer_from operations** (4 tests)
   - Success case with allowance update
   - Insufficient allowance
   - Insufficient balance (despite allowance)
   - Multiple transfers reducing allowance

**Key insight**: More tests for `transfer_from` than `approve` because 
`transfer_from` has more complex logic and failure modes.

---

### Performance Considerations

**Allowance lookup cost**: O(1) HashMap access
- Tuple key hashing: ~2x cost of single Address hash
- Still negligible (~20-40ns) compared to network latency

**Memory overhead**:
- Each allowance: (String, String, u64) ≈ 64 bytes
- For 1M allowances: ~64 MB
- Acceptable for in-memory, but needs optimization for blockchain

**Future optimization**:
```rust
// Current: String (24 bytes heap allocation)
pub type Address = String;

// Future: Fixed-size array (32 bytes on stack)
pub type Address = [u8; 32];

// Benefit: 
// - No heap allocation
// - Faster hashing
// - Lower memory usage
```

---

### Comparison: ERC-20 vs Our Implementation

| Feature | ERC-20 (Solidity) | Our Implementation (Rust) |
|---------|-------------------|---------------------------|
| **Signature verification** | Implicit (msg.sender) | Explicit (owner param) |
| **Allowance unlimited** | Yes (uint256) | No (u64 limit) |
| **Overflow protection** | Requires SafeMath | Built-in (checked_add) |
| **Return values** | bool + events | Result<(), Error> |
| **Gas optimization** | Critical | Not yet blockchain |

**Next steps for blockchain integration**:
1. Replace explicit `owner` param with `ctx.sender()`
2. Add event emission for approve/transfer_from
3. Integrate with Commonware runtime
4. Benchmark gas costs

---

## 7. Lessons Learned

### Week 1 Retrospective

**Technical Skills Acquired**:
- ✅ Rust ownership, borrowing, lifetimes
- ✅ HashMap operations and tuple keys
- ✅ Error handling with Result and custom enums
- ✅ Testing patterns (AAA, TDD)
- ✅ Performance benchmarking with Criterion
- ✅ Documentation (README, rustdoc)

**Blockchain Concepts Mastered**:
- ✅ Token standard fundamentals
- ✅ ERC-20 allowance pattern
- ✅ DeFi protocol integration patterns
- ✅ Overflow protection strategies
- ✅ Gas efficiency considerations

**Software Engineering Practices**:
- ✅ Git workflow and conventional commits
- ✅ Test-driven development
- ✅ Design documentation
- ✅ API design and trade-off analysis
- ✅ Code review and bug fixing

**Bugs Found & Fixed**:
1. `test_balance_of_non_existing_address`: Wrong expected value (1000 → 0)
2. `transfer_from` allowance update: Wrong key `(from, to)` → `(from, spender)`

**Time Investment**:
- Day 1: 4 hours (Core implementation)
- Day 2: 4 hours (Research + Benchmarking)
- Day 3: 2.5 hours (Allowance pattern)
- **Total**: 10.5 hours → Fully functional ERC-20 token

**Achievement**: 
From zero Rust knowledge to production-quality token in 3 days!

---

## 8. B-Harvest Job Requirements Mapping

### Requirements Covered

| Requirement | Evidence | Status |
|-------------|----------|--------|
| **Rust 실무 개발 경험** | 350+ lines, 16 tests | ✅ |
| **Git, GitHub 소스코드 관리** | 15+ commits, branching | ✅ |
| **Linux/Unix 사용 경험** | cargo, bash commands | ✅ |
| **영문 문서 학습 능력** | Rust Book, ERC-20 spec | ✅ |
| **새로운 기술 스택 탐구** | Commonware research | ✅ |
| **기술적 문제 해결 능력** | Bug fixes, design decisions | ✅ |
| **문서화 능력** | 4 docs, rustdoc | ✅ |
| **성능 분석** | Criterion benchmarks | ✅ |
| **오픈소스 기여 준비** | Clean code, tests, docs | 🚧 |

### Next Steps Toward B-Harvest

**Week 2**: AMM (Automated Market Maker)
- Learn constant product formula
- Implement liquidity pools
- Practice more complex Rust patterns

**Week 3**: Lending Protocol
- Collateral management
- Interest calculations
- Build on token + AMM knowledge

**Week 4**: Commonware Integration
- Connect token to real blockchain
- Study B-Harvest's actual work
- Prepare portfolio presentation

**Timeline**: Ready to apply in 3-4 weeks! 🚀