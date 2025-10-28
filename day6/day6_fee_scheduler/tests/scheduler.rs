// 이 테스트들은 수수료 우선순위 스케줄러의 핵심 동작을 검증합니다.
// - 유효성 검사 에러 처리
// - 우선순위(점수) 및 동점 시 ID 역순 정렬
// - 1,000개 랜덤 유사 데이터로 빠르게 넣고 빼기(작은 벤치마크)

use day6_fee_scheduler::{MempoolEntry, PriorityScheduler, SchedulerError, TxClass};

fn make_entry(id: &str, fee: u64, cu: u32, class: TxClass) -> MempoolEntry {
    MempoolEntry {
        id: id.to_string(),
        fee_micro_lamports: fee,
        compute_units: cu,
        class,
    }
}

#[test]
fn push_validates_and_errors() {
    let mut sched = PriorityScheduler::new();

    // fee == 0 -> FeeTooLow
    let e = make_entry("a", 0, 10_000, TxClass::Standard);
    let err = sched.push(e).unwrap_err();
    matches!(err, SchedulerError::FeeTooLow);

    // compute_units > 200_000 -> ComputeUnitsOutOfRange
    let e = make_entry("b", 1, 200_001, TxClass::Standard);
    let err = sched.push(e).unwrap_err();
    matches!(err, SchedulerError::ComputeUnitsOutOfRange);
}

#[test]
fn pop_orders_by_score_then_id_desc() {
    let mut sched = PriorityScheduler::new();

    // 점수 = fee*1000 + (200_000 - cu)
    // 동점 유도: 동일 fee, 동일 cu, id만 다름
    let e1 = make_entry("abc", 10, 100_000, TxClass::Standard); // score 동일
    let e2 = make_entry("xyz", 10, 100_000, TxClass::Standard); // score 동일, id 더 큼

    // 더 높은 점수 하나 추가
    let e3 = make_entry("mid", 11, 100_000, TxClass::Standard); // score 더 큼

    sched.push(e1).unwrap();
    sched.push(e2).unwrap();
    sched.push(e3).unwrap();

    // 가장 먼저 e3(pop)
    let first = sched.pop().unwrap();
    assert_eq!(first.id, "mid");

    // 그 다음 동점 둘: id 역순 -> "xyz" 먼저, 그 다음 "abc"
    let second = sched.pop().unwrap();
    assert_eq!(second.id, "xyz");
    let third = sched.pop().unwrap();
    assert_eq!(third.id, "abc");

    assert!(sched.is_empty());
}

#[test]
fn tiny_benchmark_push_pop_1000() {
    let mut sched = PriorityScheduler::new();

    // 1,000개 삽입
    for i in 0..1000u32 {
        let fee = (i as u64 % 50) + 1; // 1..=50
        let cu = 50_000 + (i % 10_000); // 50_000..=59_999
        let id = format!("id_{:04}", i);
        let entry = make_entry(&id, fee, cu, TxClass::LowPriority);
        sched.push(entry).unwrap();
    }

    assert_eq!(sched.len(), 1000);

    // 모두 pop하여 비워짐 확인
    let mut count = 0;
    while let Some(_e) = sched.pop() {
        count += 1;
    }
    assert_eq!(count, 1000);
    assert!(sched.is_empty());
}


