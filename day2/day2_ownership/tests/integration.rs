use day2_ownership::*;

#[test]
fn test_blockchain_node_state_management() {
    // 블록체인 노드 상태 관리 통합 테스트
    // 소유권(ownership), 빌림(borrowing), 수명(lifetime)의 통합 동작 검증

    // 1. 노드 상태 초기화
    let mut node_balance = 1000i64;
    let block_heights = vec![100, 101, 102, 103, 104, 105];

    // 2. 불변 참조를 통한 안전한 상태 읽기
    let balance_info = annotate_borrowing("노드 잔액 조회");
    assert!(!balance_info.is_empty(), "잔액 정보가 반환되어야 합니다");
    assert!(
        balance_info.contains("immutable reference"),
        "불변 참조 정보가 포함되어야 합니다"
    );

    // annotate_borrowing 리턴값 상세 확인
    println!("=== 통합 테스트 - annotate_borrowing 리턴값 ===");
    println!("{}", balance_info);
    println!("리턴값 타입: {}", std::any::type_name_of_val(&balance_info));
    assert!(balance_info.len() > 100, "상세한 정보가 포함되어야 합니다");

    // 3. 가변 참조를 통한 안전한 상태 수정
    let transaction_result = mutate_wallet(&mut node_balance, 500);
    assert!(transaction_result.is_ok(), "트랜잭션이 성공해야 합니다");
    assert_eq!(node_balance, 1500, "잔액이 올바르게 업데이트되어야 합니다");

    // mutate_wallet 리턴값 확인
    println!("=== 통합 테스트 - mutate_wallet 성공 리턴값 ===");
    println!("리턴값: {:?}", transaction_result);
    println!(
        "리턴값 타입: {}",
        std::any::type_name_of_val(&transaction_result)
    );
    assert_eq!(
        transaction_result,
        Ok(()),
        "성공 시 Ok(())를 반환해야 합니다"
    );

    // 4. 슬라이스를 통한 효율적인 블록 데이터 처리
    let (recent_blocks, total_blocks) = summarize_slice(&block_heights);
    assert_eq!(
        recent_blocks,
        &[100, 101, 102],
        "최근 3개 블록이 반환되어야 합니다"
    );
    assert_eq!(total_blocks, 6, "전체 블록 수가 올바르게 반환되어야 합니다");

    // summarize_slice 리턴값 확인
    println!("=== 통합 테스트 - summarize_slice 리턴값 ===");
    println!("recent_blocks: {:?}", recent_blocks);
    println!("total_blocks: {}", total_blocks);
    let slice_result = (recent_blocks, total_blocks);
    println!("리턴값 타입: {}", std::any::type_name_of_val(&slice_result));
    assert_eq!(recent_blocks.len(), 3, "최근 블록 수가 3개여야 합니다");
    assert_eq!(total_blocks, 6, "전체 블록 수가 6개여야 합니다");

    // 블록체인 노드에서 소유권 시스템이 메모리 안전성과 성능을 동시에 보장
}

#[test]
fn test_concurrent_safe_operations() {
    // 동시성 안전 작업 통합 테스트
    // 여러 스레드가 동시에 접근해도 안전한 블록체인 노드 동작 검증

    let mut shared_balance = 2000i64;
    let shared_blocks = vec![200, 201, 202, 203, 204];

    // 불변 참조는 동시에 여러 개 존재해도 안전
    let balance_ref1 = &shared_balance;
    let balance_ref2 = &shared_balance;
    let balance_ref3 = &shared_balance;

    // 모든 불변 참조가 같은 값을 참조
    assert_eq!(*balance_ref1, 2000);
    assert_eq!(*balance_ref2, 2000);
    assert_eq!(*balance_ref3, 2000);

    // 가변 참조는 한 번에 하나만 허용 (단일 스코프 제한)
    {
        let result = mutate_wallet(&mut shared_balance, -500);
        assert!(result.is_ok(), "가변 참조를 통한 수정이 성공해야 합니다");
        assert_eq!(shared_balance, 1500, "잔액이 올바르게 수정되어야 합니다");
    } // 가변 참조 스코프 종료

    // 슬라이스 참조는 복사 없이 효율적으로 공유
    let (block_slice, _) = summarize_slice(&shared_blocks);
    assert_eq!(
        block_slice,
        &[200, 201, 202],
        "블록 슬라이스가 올바르게 반환되어야 합니다"
    );

    // 러스트의 소유권 시스템이 블록체인 노드의 동시성 안전성을 보장
}

#[test]
fn test_memory_efficient_data_processing() {
    // 메모리 효율적인 데이터 처리 통합 테스트
    // 대용량 블록체인 데이터를 효율적으로 처리하는 검증

    // 대용량 블록 높이 배열 생성
    let large_block_heights: Vec<u64> = (1000..2000).collect();
    let mut node_balance = 5000i64;

    // 불변 참조를 통한 안전한 데이터 읽기 (복사 없음)
    let info = annotate_borrowing("대용량 블록 데이터 처리");
    assert!(
        info.contains("메모리 효율성"),
        "메모리 효율성 정보가 포함되어야 합니다"
    );

    // 슬라이스를 통한 효율적인 데이터 접근 (복사 없음)
    let (recent_blocks, total_count) = summarize_slice(&large_block_heights);
    assert_eq!(recent_blocks.len(), 3, "최근 3개 블록이 반환되어야 합니다");
    assert_eq!(
        total_count, 1000,
        "전체 블록 수가 올바르게 반환되어야 합니다"
    );

    // 가변 참조를 통한 안전한 상태 업데이트
    let update_result = mutate_wallet(&mut node_balance, -1000);
    assert!(update_result.is_ok(), "상태 업데이트가 성공해야 합니다");
    assert_eq!(node_balance, 4000, "잔액이 올바르게 업데이트되어야 합니다");

    // 빌림(borrowing) 시스템이 대용량 블록체인 데이터를
    // 메모리 효율적으로 처리할 수 있게 함을 검증
}

#[test]
fn test_error_handling_and_recovery() {
    // 에러 처리 및 복구 통합 테스트
    // 블록체인 노드에서 예외 상황 처리 검증

    let mut balance = 100i64;

    println!("=== 통합 테스트 - mutate_wallet 에러 처리 시나리오 ===");

    // 정상적인 트랜잭션
    let success_result = mutate_wallet(&mut balance, 50);
    assert!(success_result.is_ok(), "정상 트랜잭션이 성공해야 합니다");
    assert_eq!(balance, 150, "잔액이 올바르게 업데이트되어야 합니다");

    println!("정상 트랜잭션 리턴값: {:?}", success_result);

    // 음수 잔액 시도 (에러 처리)
    let error_result = mutate_wallet(&mut balance, -200);
    assert!(error_result.is_err(), "음수 잔액 시도는 실패해야 합니다");
    assert_eq!(balance, 150, "원본 잔액이 변경되지 않아야 합니다");

    // 에러 메시지 검증
    let error_msg = error_result.unwrap_err();
    println!("에러 리턴값: {}", error_msg);
    println!(
        "에러 리턴값 타입: {}",
        std::any::type_name_of_val(&error_msg)
    );
    assert!(
        error_msg.contains("잔액은 음수가 될 수 없습니다"),
        "적절한 에러 메시지가 포함되어야 합니다"
    );

    // 정확히 0이 되는 경우는 허용
    let zero_result = mutate_wallet(&mut balance, -150);
    assert!(
        zero_result.is_ok(),
        "정확히 0이 되는 경우는 허용되어야 합니다"
    );
    assert_eq!(balance, 0, "잔액이 0이 되어야 합니다");

    println!("0이 되는 경우 리턴값: {:?}", zero_result);

    // Result<(), String>을 통한 안전한 에러 처리가
    // 블록체인 노드의 견고성을 보장함을 검증
}

#[test]
fn test_lifetime_and_ownership_integration() {
    // 수명과 소유권의 통합 동작 테스트
    // 블록체인 노드에서 메모리 안전성과 성능의 균형 검증

    // 1. 소유권 이동 없이 데이터 공유
    let original_blocks = vec![300, 301, 302, 303, 304];
    let original_balance = 3000i64;

    // 2. 불변 참조를 통한 안전한 데이터 접근
    let info = annotate_borrowing("통합 테스트");
    assert!(!info.is_empty(), "정보가 반환되어야 합니다");

    // 3. 슬라이스 참조를 통한 효율적인 데이터 처리
    let (slice, len) = summarize_slice(&original_blocks);
    assert_eq!(
        slice,
        &[300, 301, 302],
        "슬라이스가 올바르게 반환되어야 합니다"
    );
    assert_eq!(len, 5, "전체 길이가 올바르게 반환되어야 합니다");

    // 4. 원본 데이터가 여전히 유효함을 확인 (소유권 이동 없음)
    assert_eq!(
        original_blocks.len(),
        5,
        "원본 데이터가 여전히 유효해야 합니다"
    );
    assert_eq!(original_balance, 3000, "원본 잔액이 여전히 유효해야 합니다");

    // 5. 가변 참조를 통한 안전한 상태 수정
    let mut balance = original_balance;
    let result = mutate_wallet(&mut balance, -500);
    assert!(result.is_ok(), "상태 수정이 성공해야 합니다");
    assert_eq!(balance, 2500, "잔액이 올바르게 수정되어야 합니다");

    // 수명(lifetime)과 소유권(ownership) 시스템이
    // 블록체인 노드의 메모리 안전성과 성능을 동시에 보장하는
    // 러스트의 핵심 철학을 검증
}
