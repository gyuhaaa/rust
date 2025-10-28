use std::collections::BinaryHeap;
use std::cmp::Ordering;

pub enum TxClass {
    // 높은 수수료 트랜잭션을 먼저 꺼내는 큐를 구현하기 위해 존재합니다.
    // High priority transactions for urgent processing
    HighPriority,
    // 보통 수수료 트랜잭션을 먼저 꺼내는 큐를 구현하기 위해 존재합니다.
    // Standard priority transactions for normal processing
    Standard,
    // 낮은 수수료 트랜잭션을 먼저 꺼내는 큐를 구현하기 위해 존재합니다.
    // Low priority transactions for background processing
    LowPriority,
}

// 트랜잭션 정보를 담기 위한 구조체
pub struct MempoolEntry {
    pub id: String,
    pub fee_micro_lamports: u64,
    pub compute_units: u32,
    pub class: TxClass
}

#[derive(Debug, thiserror::Error)]
pub enum SchedulerError {
    #[error("fee must be greater than zero")]
    FeeTooLow,
    #[error("compute units must be within 200_000")]
    ComputeUnitsOutOfRange,
}

pub struct PriorityScheduler {
    heap: BinaryHeap<ScheduledTx>,
}

// `ScheduledTx`에 `Ord`, `PartialOrd`, `Eq`, `PartialEq`를 구현해 `score`가 높은 항목이 먼저 나오도록 하세요.
// 동점일 때는 `entry.id`의 사전식 역순(큰 값 우선)으로 정렬합니다. 구현 전략을 주석으로 남기세요.
// #[derive(Eq, PartialEq, Ord, PartialOrd)]
pub struct ScheduledTx {
    pub entry: MempoolEntry,
    pub score: u128,
}

impl PartialEq for ScheduledTx {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score && self.entry.id == other.entry.id
    }
}

impl Eq for ScheduledTx {}

// score 기준 정렬. 동점일 때는 entry.id 기준 역순 정렬
impl Ord for ScheduledTx {
    fn cmp(&self, other: &Self) -> Ordering {
        // score 기준 비교
        match self.score.cmp(&other.score) {
            // 동점일 때는 entry.id 기준 역순 정렬
            Ordering::Equal => {
                self.entry.id.cmp(&other.entry.id)
            }
            // 변수 바인딩(정의): Equal이 아닐 때 그 값을 other_order라는 새 변수에 할당(바인딩)
            other_order => other_order
        }
    }
}

impl PartialOrd for ScheduledTx {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}    

impl PriorityScheduler {
    // 새로운 스케줄러 인스턴스를 생성합니다.
    // Creates a new priority scheduler instance.
    pub fn new() -> Self {
        Self { heap: BinaryHeap::new() }
    }
    pub fn push(&mut self, entry: MempoolEntry) -> Result<(), SchedulerError> {
        // - `fee_micro_lamports == 0`이면 `SchedulerError::FeeTooLow`를 반환합니다.
        if entry.fee_micro_lamports == 0 {
            return Err(SchedulerError::FeeTooLow);
        }
        // - `compute_units > 200_000`이면 `SchedulerError::ComputeUnitsOutOfRange`를 반환합니다.
        if entry.compute_units > 200_000 {
            return Err(SchedulerError::ComputeUnitsOutOfRange);
        }
        // - `score`는 `fee_micro_lamports as u128 * 1_000 + (200_000 - compute_units as u128)`로 계산합니다.
        let score = entry.fee_micro_lamports as u128 * 1_000 + (200_000 - entry.compute_units as u128);
        let scheduled_tx = ScheduledTx {
            entry,
            score,
        };
        self.heap.push(scheduled_tx);
        Ok(())
    }
    // 가장 높은 우선순위의 엔트리를 꺼냅니다.
    // Pops and returns the highest-priority entry.
    pub fn pop(&mut self) -> Option<MempoolEntry> {
        self.heap.pop().map(|s| s.entry)
    }

    // 현재 큐에 담긴 엔트리 수를 반환합니다.
    // Returns the number of entries in the queue.
    pub fn len(&self) -> usize {
        self.heap.len()
    }

    // 큐가 비었는지 여부를 반환합니다.
    // Returns whether the queue is empty.
    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }
}