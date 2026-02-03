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