# Commonware Research Notes

**Date**: 2025-02-05 (Day 2)
**Goal**: Understand Commonware architecture and its relevance to token development

---

## Research Log

### Session 1: Initial Discovery (10:00-11:00)

#### What is Commonware?

Commonware은 특정 용도에 최적화된 블록체인을 구축할 때 반복되는 복잡한 인프라 작업(네트워킹, 합의, 암호화 등)을 대신 처리해주는 **공통 라이브러리 및 도구 모음(Common Library for DApp)**입니다. 개발자가 로우 레벨의 인프라 구축보다는 애플리케이션의 핵심 로직에 집중할 수 있게 돕는 프레임워크

#### Core Components

- P2P Networking: 복잡한 네트워크 토플로지와 노드 간 통신 관리.
- Cryptography: 현대적이고 안전한 암호화 기본 체계 제공.
- Consensus Engines: 성능이 뛰어나고 신뢰할 수 있는 합의 알고리즘 라이브러리.
- Storage & State Management: 블록체인 상태 저장 및 조회를 위한 최적화된 스토리지 솔루션

#### Key Questions

- Commonware은 특정 언어(Rust 등)에 의존적인가? 아니면 다양한 언어를 지원하는가? => Rust만
- 기존의 Cosmos SDK나 Substrate와 같은 프레임워크와 비교했을 때 Commonware만의 가장 큰 차별점(경량화 또는 모듈성)은 무엇인가?

```

## Commonware Architecture

### Traditional Stack (Monolithic)
```

┌─────────────────────────┐
│ Your Application │
└─────────────────────────┘
↓ (tightly coupled)
┌─────────────────────────┐
│ Entire Framework │
│ (All-or-nothing) │
│ - Consensus │
│ - Networking │
│ - Storage │
│ - Crypto │
└─────────────────────────┘

```

### Commonware Approach (Modular)
```

┌─────────────────────────┐
│ Your Application │
│ (Token Standard) │
└─────────────────────────┘
↓ (pick & choose)
┌─────────┐ ┌──────────┐ ┌─────────┐
│Consensus│ │Networking│ │ Crypto │ ← Use only what you need
└─────────┘ └──────────┘ └─────────┘
┌─────────┐ ┌──────────┐
│ Storage │ │ Runtime │
└─────────┘ └──────────┘

```

### Token Standard의 위치
```

┌─────────────────────────┐
│ Token Standard (우리) │ ← Application/Runtime layer
└─────────────────────────┘
↓ uses
┌─────────────────────────┐
│ Commonware Libraries │ ← Infrastructure layer
│ - Cryptography (서명) │
│ - Storage (상태 저장) │
└─────────────────────────┘

```

## Core Components

### 1. Networking
**Purpose**: P2P communication between nodes
**What it does**:
- Gossip protocol
- Peer discovery
- Message routing

**Token standard에서 필요한가?**
- 직접 사용: No
- 간접 영향: Yes (트랜잭션이 네트워크를 통해 전파됨)

### 2. Cryptography
**Purpose**: Digital signatures and hashing
**What it does**:
- Verify transaction signatures
- Generate/verify addresses
- Hash functions

**Token Standard에서 필요한가?**
- 직접 사용: Yes! (transfer시 서명 검증 필요)
- 예: "이 transfer를 진짜 alice가 보냈나?"

### 3. Consensus
**Purpose**: Agreement on transaction order
**What it does**:
- Block production
- Finality
- Leader election

**Token Standard에 필요한가?**
- 직접 사용: No (상위 레이어)
- 간접 영향: Yes (transfer 순서 결정)

### 4. Storage
**Purpose**: Persistent state management
**What it does**:
- Key-value store
- State tree (Merkle tree)
- Proof generation

**Token Standard에서 필요한가?**:
- 직접 사용: Yes! (HashMap → 실제 DB)
- 현재: `HashMap<Address, Balance>` (메모리)
- 실제: Commonware Storage (디스크)

### 5. Runtime
**Purpose**: Execute application logic
**What it does**:
- Transaction execution
- State transitions
- Gas metering?

**Token Standard에서 필요한가?**:
- 직접 사용: Yes! (transfer 로직 실행)
---
```

#### 1. Commonware vs Cosmos SDK

**Similarities**:

- 범용 체인이 아닌, 특정 목적에 맞는 애플리케이션 전용 블록체인(App-chain)구축을 지향
- 기능별로 나누어진 모듈을 조합하여 체인을 구성하는 모듈형 구조

**Differences**:

- Language: Rust vs Golang
- Architecture: Cosmos SDK는 프레임워크 방식으로, 미리 정의된 틀(BaseApp, SDK모듈)안에 로직을 끼워 넣는 방식이라 초기 구축이 매우 빠름. Commonware는 Primitives방식으로, 틀을 강제하지 않고 네트워킹, 합의, 스토리지 등 기초 부품만 제공하여 개발자가 완전히 새로운 아키텍처를 설계할 수 있게 함.
- Use case: CosmosSDK는 검증된 모듈을 사용해 빠르게 상호운용 가능한 체인을 만들 때 유리, Commonware는 기존 프레임워크의 제약에서 벗어나 초고속 합의(300ms 등)나 특수한 데이터 구조가 필요한 혁신적인 프로젝트에 적합

#### 2. Commonware vs Substrate

**Similarities**:

- Both Rust
- 둘 다 블록체인의 로우 레벨(네트워킹, 합의 엔진 등)까지 직접 커스터마이징할 수 있는 깊은 제어권을 제공

**Differences**:

- Philosophy: Substrate는 편리한 커스텀이 핵심, Pallet이라는 강력한 도구를 통해 거버넌스, 스테이킹 등을 레고처럼 조립하고 Wasm기반의 포크리스 업그레이드를 중시. Commonware는 안티 프레임워크를 표방, Substrate보다 가볍고 결합도가 낮은 라이브러리 형태를 지향하여, 특정 생태계의 규칙에 얽매이지 않는 것을 강조
- Ecosystem:
  Substrate: Polkadot/Kusama생태계와 매우 밀접하게 연결되어 있으며, 파라체인 구조에 최적화 되어 있음.
  Commonware: 특정 네트워크에 종속되지 않는 독립적인 인프라 구축을 목표로 하며, 최근 Noble과 같은 체인들이 마이그레이션을 발표하며 독자적인 생태계를 구축 중
