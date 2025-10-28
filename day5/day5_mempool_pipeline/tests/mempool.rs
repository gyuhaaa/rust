// 멤풀 파이프라인의 핵심 기능들을 검증하는 통합 테스트
// 임계값 필터링, 트랜잭션 필터링, 계정별 그룹화, 통계 계산 기능을 종합적으로 테스트

use day5_mempool_pipeline::*;

// 테스트용 Fixture Data 생성 함수
fn create_test_transactions() -> Vec<PendingTx> {
    vec![
        PendingTx {
            id: "tx1".to_string(),
            account: "account1".to_string(),
            fee_micro_lamports: 1000,
            payload_size: 500,
            status: TxStatus::Pending,
        },
        PendingTx {
            id: "tx2".to_string(),
            account: "account1".to_string(),
            fee_micro_lamports: 2000,
            payload_size: 300,
            status: TxStatus::Simulated { compute_units: 100 },
        },
        PendingTx {
            id: "tx3".to_string(),
            account: "account2".to_string(),
            fee_micro_lamports: 500, // 낮은 수수료
            payload_size: 800,       // 큰 페이로드
            status: TxStatus::Pending,
        },
        PendingTx {
            id: "tx4".to_string(),
            account: "account2".to_string(),
            fee_micro_lamports: 1500,
            payload_size: 200,
            status: TxStatus::Rejected {
                reason: "insufficient funds".to_string(),
            },
        },
        PendingTx {
            id: "tx5".to_string(),
            account: "account3".to_string(),
            fee_micro_lamports: 3000,
            payload_size: 100,
            status: TxStatus::Pending,
        },
    ]
}

#[test]
fn test_threshold_filter_all_conditions() {
    // 임계값 필터가 수수료, 페이로드, 시뮬레이션 실패 조건을 모두 적용하는지 확인

    let txs = create_test_transactions();

    // 필터 설정: 최소 수수료 1000, 최대 페이로드 600, 시뮬레이션 실패 거부
    let filter = ThresholdFilter {
        min_fee: 1000,
        max_payload: 600,
        reject_simulation_failures: true,
    };

    let filtered = filter_transactions(&txs, &filter);

    // 예상 결과: tx1, tx2, tx5만 통과 (tx3은 수수료 낮음, tx4는 거절됨)
    assert_eq!(filtered.len(), 3);

    let ids: Vec<&String> = filtered.iter().map(|tx| &tx.id).collect();
    assert!(ids.contains(&&"tx1".to_string()));
    assert!(ids.contains(&&"tx2".to_string()));
    assert!(ids.contains(&&"tx5".to_string()));
    assert!(!ids.contains(&&"tx3".to_string())); // 수수료 낮음
    assert!(!ids.contains(&&"tx4".to_string())); // 거절됨
}

#[test]
fn test_threshold_filter_without_rejecting_simulation_failures() {
    // 시뮬레이션 실패 거부를 비활성화했을 때의 동작 확인

    let txs = create_test_transactions();

    // 필터 설정: 시뮬레이션 실패 거부 비활성화
    let filter = ThresholdFilter {
        min_fee: 1000,
        max_payload: 600,
        reject_simulation_failures: false, // 거부하지 않음
    };

    let filtered = filter_transactions(&txs, &filter);

    // 예상 결과: tx1, tx2, tx4, tx5 통과 (tx4도 허용됨)
    assert_eq!(filtered.len(), 4);

    let ids: Vec<&String> = filtered.iter().map(|tx| &tx.id).collect();
    assert!(ids.contains(&&"tx1".to_string()));
    assert!(ids.contains(&&"tx2".to_string()));
    assert!(ids.contains(&&"tx4".to_string())); // 이제 허용됨
    assert!(ids.contains(&&"tx5".to_string()));
    assert!(!ids.contains(&&"tx3".to_string())); // 여전히 수수료 낮음
}

#[test]
fn test_filter_transactions_lifetime_and_references() {
    // filter_transactions가 참조를 반환하며 원본 벡터와 수명이 연동되는지 검증

    let txs = create_test_transactions();
    let filter = ThresholdFilter {
        min_fee: 1000,
        max_payload: 600,
        reject_simulation_failures: true,
    };

    // 필터링된 트랜잭션들 가져오기
    let filtered = filter_transactions(&txs, &filter);

    // 반환된 참조들이 원본 데이터와 동일한지 확인
    assert_eq!(filtered.len(), 3);

    // 각 필터링된 트랜잭션의 필드 값이 원본과 일치하는지 확인
    for filtered_tx in &filtered {
        let original_tx = txs.iter().find(|tx| tx.id == filtered_tx.id).unwrap();

        // 참조를 통해 필드 값 비교
        assert_eq!(filtered_tx.id, original_tx.id);
        assert_eq!(filtered_tx.account, original_tx.account);
        assert_eq!(
            filtered_tx.fee_micro_lamports,
            original_tx.fee_micro_lamports
        );
        assert_eq!(filtered_tx.payload_size, original_tx.payload_size);

        // 상태 비교 (match 사용)
        match (&filtered_tx.status, &original_tx.status) {
            (TxStatus::Pending, TxStatus::Pending) => {}
            (
                TxStatus::Simulated { compute_units: a },
                TxStatus::Simulated { compute_units: b },
            ) => {
                assert_eq!(a, b);
            }
            (TxStatus::Rejected { reason: a }, TxStatus::Rejected { reason: b }) => {
                assert_eq!(a, b);
            }
            _ => panic!("상태가 일치하지 않음"),
        }
    }
}

#[test]
fn test_group_by_account() {
    // group_by_account가 계정별로 트랜잭션을 정확히 묶는지 확인

    let txs = create_test_transactions();
    let grouped = group_by_account(&txs);

    // 계정별로 그룹화 확인
    assert_eq!(grouped.len(), 3); // account1, account2, account3

    // account1: tx1, tx2
    let account1_txs = grouped.get("account1").unwrap();
    assert_eq!(account1_txs.len(), 2);
    let account1_ids: Vec<&String> = account1_txs.iter().map(|tx| &tx.id).collect();
    assert!(account1_ids.contains(&&"tx1".to_string()));
    assert!(account1_ids.contains(&&"tx2".to_string()));

    // account2: tx3, tx4
    let account2_txs = grouped.get("account2").unwrap();
    assert_eq!(account2_txs.len(), 2);
    let account2_ids: Vec<&String> = account2_txs.iter().map(|tx| &tx.id).collect();
    assert!(account2_ids.contains(&&"tx3".to_string()));
    assert!(account2_ids.contains(&&"tx4".to_string()));

    // account3: tx5
    let account3_txs = grouped.get("account3").unwrap();
    assert_eq!(account3_txs.len(), 1);
    assert_eq!(account3_txs[0].id, "tx5");
}

#[test]
fn test_compute_account_stats() {
    // compute_account_stats가 계정별 통계를 정확히 계산하는지 확인

    let txs = create_test_transactions();
    let grouped = group_by_account(&txs);
    let stats = compute_account_stats(&grouped);

    // account1 통계 확인
    let account1_stats = stats.get("account1").unwrap();
    assert_eq!(account1_stats.total_fee, 3000); // 1000 + 2000
    assert_eq!(account1_stats.total_bytes, 800); // 500 + 300
    assert_eq!(account1_stats.pending, 1); // tx1만 Pending

    // account2 통계 확인
    let account2_stats = stats.get("account2").unwrap();
    assert_eq!(account2_stats.total_fee, 2000); // 500 + 1500
    assert_eq!(account2_stats.total_bytes, 1000); // 800 + 200
    assert_eq!(account2_stats.pending, 1); // tx3만 Pending

    // account3 통계 확인
    let account3_stats = stats.get("account3").unwrap();
    assert_eq!(account3_stats.total_fee, 3000); // 3000
    assert_eq!(account3_stats.total_bytes, 100); // 100
    assert_eq!(account3_stats.pending, 1); // tx5만 Pending
}

#[test]
fn test_compute_account_stats_pending_count() {
    // Pending 상태 개수 계산이 정확한지 확인

    let txs = vec![
        PendingTx {
            id: "tx1".to_string(),
            account: "test_account".to_string(),
            fee_micro_lamports: 1000,
            payload_size: 100,
            status: TxStatus::Pending,
        },
        PendingTx {
            id: "tx2".to_string(),
            account: "test_account".to_string(),
            fee_micro_lamports: 2000,
            payload_size: 200,
            status: TxStatus::Simulated { compute_units: 50 },
        },
        PendingTx {
            id: "tx3".to_string(),
            account: "test_account".to_string(),
            fee_micro_lamports: 3000,
            payload_size: 300,
            status: TxStatus::Rejected {
                reason: "error".to_string(),
            },
        },
        PendingTx {
            id: "tx4".to_string(),
            account: "test_account".to_string(),
            fee_micro_lamports: 4000,
            payload_size: 400,
            status: TxStatus::Pending,
        },
    ];

    let grouped = group_by_account(&txs);
    let stats = compute_account_stats(&grouped);

    let account_stats = stats.get("test_account").unwrap();
    assert_eq!(account_stats.pending, 2); // tx1, tx4만 Pending
    assert_eq!(account_stats.total_fee, 10000); // 모든 수수료 합계
    assert_eq!(account_stats.total_bytes, 1000); // 모든 바이트 합계
}
