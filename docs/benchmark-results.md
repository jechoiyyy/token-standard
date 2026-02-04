# Benchmark Results - Day 2

**Date**: 2025-02-05
**Environment**:

- OS: Linux
- CPU: AMD Ryzen 7 Pro 4750U
- Rust: cargo 1.92.0

---

## Results Summary

| Operation                 | Time (ns) | Notes                    |
| ------------------------- | --------- | ------------------------ |
| balance_of (existing)     | 22.2 ns   | HashMap lookup + copy    |
| balance_of (non-existing) | 17.8 ns   | HashMap lookup + default |
| transfer (success)        | 173.8 ns  | 2x HashMap insert        |
| transfer (insufficient)   | 80.5 ns   | Early return             |

---

## Detailed Analysis

### 1. balance_of Performance

#### Existing Address

- Time: 22.2 ns
- Operations:
  1. HashMap::get() - O(1)
  2. Option::copied() - copy u64
  3. Return value

#### Non-existing Address

- Time: 17.8 ns
- Operations:
  1. HashMap::get() → None
  2. unwrap_or(0)
  3. Return 0

**Comparison**:
[존재하는 주소가 더 빠른가? 느린가? 왜?]
존재하는 주소가 더 느림, copied와 unwrap_or 과정에서 로직 실행 양이 다르기 때문.

**Q116**: 이 차이가 실제 블록체인에서 중요할까요?

- 네트워크 지연시간을 최소화하기 위해, 이런 미세한 연산차이조차도 최적화하거나, 가스비 산정에 반영
- 한 블록에 수천 개의 트랜잭션이 들어가는 블록체인 환경에서는 이 차이가 누적되어 블록 전파 속도에 영향을 주게 됨.

힌트:

- 수백만 개의 주소
- 대부분은 잔액이 없음 (0)
- 어떤 케이스가 더 자주 발생?

---

### 2. transfer Performance

#### Success Case

- Time: 173.8 ns
- Operations:
  1. Self-transfer 체크
  2. Zero amount 체크
  3. balance_of (from)
  4. Insufficient balance 체크
  5. checked_add (to)
  6. HashMap::insert × 2

#### Insufficient Balance Case

- Time: 80.5 ns
- Operations:
  1. Self-transfer 체크
  2. Zero amount 체크
  3. balance_of (from)
  4. Insufficient balance 체크
  5. **Early return!** (나머지 생략)

**Comparison**:
[성공 케이스가 얼마나 더 느린가?]
2배 가까이 느림.

**Q117**: Early return이 왜 중요한가?

- Early return이 없다면, DoS 공격의 경우 실패할 트랜잭션임에도 불구하고 무거운 연상을 수행하느라 CPU 자원이 낭비됨. 블록체인에서 계산 리소스는 곧 돈임.

블록체인 관점:

- 잘못된 트랜잭션 (잔액 부족)이 많음
- 빨리 실패하면 리소스 절약
- Gas 효율성

---

### 3. Bottleneck 분석

**가장 느린 Operation은?**
[balance_of? transfer? 왜?]
transfer

**Q118**: transfer가 balance_of보다 몇 배 느린가요?

- 성공한 transfer을 기준으로 약 8배 이상 느림.

계산:

```
transfer time / balance_of time = ???x
```

**왜 이렇게 차이가 날까요?**

- balance_of는 조회 1번으로 끝나고, transfer는 조회 2번, 쓰기 2번, 산술 연산 등 연산 수가 많음
- balance_of는 값을 읽기만 하지만, transfer에서는 insert 쓰기 작업이 있어 무거움

힌트:

- balance_of: HashMap lookup 1번
- transfer:
  - balance_of 2번 (from, to)
  - HashMap insert 2번
  - checked_add 1번
  - 여러 if 체크

---

### 4. 최적화 아이디어

#### Idea 1: Address 타입 변경

현재: `String` (24 bytes heap allocation)
대안: `[u8; 32]` (stack allocation)

**예상 효과**: 10-20% 개선?

#### Idea 2: Balance 타입 변경

현재: `u64`
대안: `u128` (더 큰 범위)

**Trade-off**: 범위 ↑, 속도 ↓

#### Idea 3: HashMap vs BTreeMap

현재: `HashMap`
대안: `BTreeMap`

**언제 BTreeMap이 나을까?**

- 순서가 필요할 때
- 범위 쿼리 (예: "잔액 > 1000인 주소들")

---

### 5. 비하베스트 채용 요건 연결

채용 공고:

> "블록체인 코어 모듈 설계, 개발, 및 **성능 분석**"

이 벤치마크가 보여주는 것:

- ✅ 성능 측정 능력
- ✅ Bottleneck 파악
- ✅ 최적화 아이디어 제시
- ✅ Trade-off 분석

**Q119**: 면접에서 이 벤치마크 결과를 어떻게 설명하시겠습니까?
토큰 컨트랙트의 핵심 로직을 Rust로 구현하고 Criterion을 통해 나노초 단위의 성능 벤치마크를 수행했습니다. 분석 결과 transfer로직은 약 174ns가 소요되며, 이는 단순 잔액 조회보다 7.8배 더 무거운 작업임을 확인했습니다. 주요 병목은 HashMap insert과정에서의 상태 쓰기 오버헤드였습니다.
Early Return 기법을 적용해. 실패 케이스의 실행 시간을 50%이상 단축시켰고, 이는 실제 환경에서 공격을 방어하고 가스 비용을 합리적으로 산정하는데 필수적인 포인트라고 생각합니다.

---

## Next Steps

### Short-term

- [ ] HashMap vs BTreeMap 벤치마크
- [ ] Address 타입별 벤치마크 (String vs [u8;32])
- [ ] approve/transferFrom 추가 시 성능 측정

### Long-term

- [ ] Commonware Storage와 통합 시 성능
- [ ] 실제 블록체인 네트워크에서 TPS 측정
