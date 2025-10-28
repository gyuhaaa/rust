use std::collections::BTreeMap;

// 2. **도메인 모델 정의하기 (`src/lib.rs`)**
//    - `PendingTx` 구조체를 선언하고 아래 필드를 추가합니다.
//      - `pub id: String` — 트랜잭션 해시를 문자열로 표현합니다.
//      - `pub account: String` — 트랜잭션을 보낸 계정(공개키)입니다.
//      - `pub fee_micro_lamports: u64` — 지불한 수수료(1 lamport = 10^-6 SOL).
//      - `pub payload_size: u32` — 직렬화된 트랜잭션 크기(Byte 단위).
//      - `pub status: TxStatus` — 아래에서 정의할 상태 열거형입니다.
//    - `TxStatus` 열거형을 정의하고 `Pending`, `Simulated { compute_units: u64 }`, `Rejected { reason: String }` 변형을 선언합니다.
//    - 각 구조체/열거형 위에는 "이 데이터가 왜 필요한지" 어린이도 이해할 수 있게 한국어+영어로 짧은 주석을 작성하세요.

// 트랜잭션이 진행중인지, 시뮬레이션 결과가 나왔는지, 거절되었는지 상태를 나타내기 위한 열거형
#[derive(Clone)]
pub enum TxStatus {
    Pending,
    Simulated { compute_units: u64 },
    Rejected { reason: String },
}

// 트랜잭션 정보를 담기 위한 구조체
#[derive(Clone)]
pub struct PendingTx {
    pub id: String,
    pub account: String,
    pub fee_micro_lamports: u64,
    pub payload_size: u32,
    pub status: TxStatus,
}

// 3. **필터 추상화 설계 (`src/lib.rs`)**
//    - `pub trait MempoolFilter`를 선언하고 `fn allow(&self, tx: &PendingTx) -> bool` 메서드를 정의합니다.
//    - `ThresholdFilter` 구조체를 만들어 아래 필드를 추가합니다.
//      - `pub min_fee: u64` — 허용할 최소 수수료.
//      - `pub max_payload: u32` — 허용할 최대 페이로드 크기.
//      - `pub reject_simulation_failures: bool` — 시뮬레이션 실패(`TxStatus::Rejected`)를 거부할지 여부.
//    - `MempoolFilter`를 `ThresholdFilter`가 구현하도록 하고, 각 조건을 만족해야 `true`를 반환하도록 작성하세요. 조건 체크 순서를 주석으로 설명해 주세요.
pub trait MempoolFilter {
    fn allow(&self, tx: &PendingTx) -> bool;
}

pub struct ThresholdFilter {
    pub min_fee: u64,
    pub max_payload: u32,
    pub reject_simulation_failures: bool,
}

impl MempoolFilter for ThresholdFilter {
    fn allow(&self, tx: &PendingTx) -> bool {
        // 1단계: 수수료 조건 확인
        if tx.fee_micro_lamports < self.min_fee {
            // log::info!("수수료 조건 불만족: {}", tx.id);
            return false;
        }
        // 2단계: 페이로드 크기 조건 확인
        if tx.payload_size > self.max_payload {
            // log::info!("페이로드 크기 조건 불만족: {}", tx.id);
            return false;
        }
        // 3단계: 시뮬레이션 실패 조건 확인
        if self.reject_simulation_failures {
            if let TxStatus::Rejected { .. } = tx.status {
                // log::info!("시뮬레이션 실패 조건 불만족: {}", tx.id);
                return false;
            }
        }
        true
    }
}

// 4. **핵심 기능 구현 (`src/lib.rs`)**
//    아래 함수들을 구현하고 함수 위에 "무엇을 반환하며 왜 필요한지" 서술형 주석을 추가하세요.
//    1. `pub fn filter_transactions<'a, F: MempoolFilter>(txs: &'a [PendingTx], filter: &F) -> Vec<&'a PendingTx>`
//       - 주어진 필터를 통과한 트랜잭션 참조만 모아 반환합니다.
//       - 라이프타임 매개변수(`'a`)를 명시적으로 선언해 참조가 안전하게 유지되도록 하세요.
//    2. `pub fn group_by_account(txs: &[PendingTx]) -> BTreeMap<String, Vec<PendingTx>>`
//       - 계정별로 트랜잭션을 묶어 사전식 정렬이 보장되는 `BTreeMap`에 저장합니다.
//       - 각 계정의 벡터는 원본 트랜잭션을 복사해 새로운 벡터로 관리합니다.
//    3. `pub struct AccountStats { pub total_fee: u64, pub total_bytes: u32, pub pending: usize }`를 선언하고,
//       `pub fn compute_account_stats(grouped: &BTreeMap<String, Vec<PendingTx>>) -> BTreeMap<String, AccountStats>` 함수를 구현합니다.
//       - 각 계정의 총 수수료, 총 페이로드 크기, `TxStatus::Pending` 개수를 계산합니다.
//       - 루프 내부에서 `match`를 활용해 상태별로 분기하세요.

// 주어진 필터 조건을 만족하는 트랜잭션들만 선별하여 반환하는 함수
// 원본 데이터를 복사하지 않고 참조만 반환하여 메모리 효율적이며,
// 라이프타임을 통해 참조의 안전성을 보장
pub fn filter_transactions<'a, F: MempoolFilter>(
    txs: &'a [PendingTx],
    filter: &F,
) -> Vec<&'a PendingTx> {
    txs.iter().filter(|tx| filter.allow(tx)).collect()
}

// 트랜잭션들을 계정별로 그룹화하여 BTreeMap으로 반환하는 함수
// 계정 주소를 키로 사용하여 사전식 정렬이 보장되며,
// 각 계정의 트랜잭션들을 별도 벡터로 관리하여 후속 통계 계산에 활용
pub fn group_by_account(txs: &[PendingTx]) -> BTreeMap<String, Vec<PendingTx>> {
    let mut grouped = BTreeMap::new();
    for tx in txs {
        grouped
            .entry(tx.account.clone())
            .or_insert(Vec::new())
            .push((*tx).clone());
    }
    grouped
}

pub struct AccountStats {
    pub total_fee: u64,
    pub total_bytes: u32,
    pub pending: usize,
}

// 계정별 트랜잭션 그룹을 분석하여 통계 정보를 계산하는 함수
// 각 계정의 총 수수료, 총 바이트 크기, Pending 상태 개수를 집계하여
// AccountStats 구조체로 반환하며, BTreeMap을 통해 사전식 정렬 보장
pub fn compute_account_stats(
    grouped: &BTreeMap<String, Vec<PendingTx>>,
) -> BTreeMap<String, AccountStats> {
    let mut stats = BTreeMap::new();
    for (account, txs) in grouped {
        let total_fee = txs.iter().map(|tx| tx.fee_micro_lamports).sum();
        let total_bytes = txs.iter().map(|tx| tx.payload_size).sum();
        let pending = txs
            .iter()
            .filter(|tx| match tx.status {
                TxStatus::Pending => true,
                _ => false,
            })
            .count();
        stats.insert(
            account.clone(),
            AccountStats {
                total_fee,
                total_bytes,
                pending,
            },
        );
    }
    stats
}
