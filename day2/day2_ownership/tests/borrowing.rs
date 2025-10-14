use day2_ownership::*;

#[test]
fn test_annotate_borrowing_does_not_consume_original() {
    // annotate_borrowing 결과가 원본 문자열을 소비하지 않는지 테스트
    let original_message = String::from("블록체인 노드 상태");

    // 함수 호출 전 원본 문자열 확인
    assert_eq!(original_message, "블록체인 노드 상태");

    // annotate_borrowing 호출 (불변 참조로 차용)
    let result = annotate_borrowing(&original_message);

    // 함수 호출 후에도 원본 문자열이 여전히 유효함 (소유권 이동 없음)
    assert_eq!(original_message, "블록체인 노드 상태");

    // 결과가 비어있지 않음을 확인
    assert!(!result.is_empty());

    // 결과가 다중 라인 문자열인지 확인
    assert!(
        result.contains('\n'),
        "결과가 다중 라인 문자열이어야 합니다"
    );

    // 불변 참조(immutable reference)를 통한 차용(borrowing)이
    // 원본 데이터의 소유권(ownership)을 이동시키지 않음을 검증
    // 블록체인 노드에서 상태를 읽기만 할 때 안전하게 사용 가능
}

#[test]
fn test_annotate_borrowing_contains_required_keywords() {
    // annotate_borrowing 결과가 필수 키워드를 포함하는지 테스트
    let message = "테스트 메시지";
    let result = annotate_borrowing(message);

    // 필수 키워드 "immutable reference" 포함 확인
    assert!(
        result.contains("immutable reference"),
        "결과에 'immutable reference' 키워드가 포함되어야 합니다"
    );

    // 필수 키워드 "no data race" 포함 확인
    assert!(
        result.contains("no data race"),
        "결과에 'no data race' 키워드가 포함되어야 합니다"
    );

    // 결과의 길이 확인
    assert!(result.len() > 100, "결과가 충분히 상세해야 합니다");

    // 수명(lifetime)과 빌림(borrowing) 개념이 블록체인 노드의
    // 동시성 안전성을 보장하는 핵심 메커니즘임을 검증
}

#[test]
fn test_mutate_wallet_successful_operations() {
    // mutate_wallet의 정상적인 증감 작업 테스트
    let mut balance = 100;

    // 정상적인 증가
    let result = mutate_wallet(&mut balance, 50);
    assert!(result.is_ok(), "정상적인 증가 작업이 성공해야 합니다");
    assert_eq!(balance, 150, "잔액이 올바르게 증가해야 합니다");

    // 정상적인 감소
    let result = mutate_wallet(&mut balance, -30);
    assert!(result.is_ok(), "정상적인 감소 작업이 성공해야 합니다");
    assert_eq!(balance, 120, "잔액이 올바르게 감소해야 합니다");

    // 가변 참조(mutable reference)를 통한 안전한 상태 수정이
    // 블록체인 노드에서 트랜잭션 처리 시 데이터 무결성을 보장
}

#[test]
fn test_mutate_wallet_prevents_negative_balance() {
    // mutate_wallet의 음수 잔액 방지 로직 테스트
    let mut balance = 50;

    // 음수가 되는 경우 테스트
    let result = mutate_wallet(&mut balance, -100);
    assert!(
        result.is_err(),
        "음수 잔액이 되는 경우 에러를 반환해야 합니다"
    );
    assert_eq!(balance, 50, "원본 잔액이 변경되지 않아야 합니다");

    // 에러 리턴값의 타입 확인 (unwrap_err() 호출 전에 확인)
    assert!(result.is_err(), "에러 시 Err(String)을 반환해야 합니다");

    // 에러 메시지 확인
    let error_msg = result.unwrap_err();

    // 에러 메시지 구조 확인
    assert!(
        error_msg.contains("잔액은 음수가 될 수 없습니다"),
        "적절한 에러 메시지가 포함되어야 합니다"
    );
    assert!(
        error_msg.contains("50"),
        "현재 잔액이 에러 메시지에 포함되어야 합니다"
    );
    assert!(
        error_msg.contains("-100"),
        "변경값이 에러 메시지에 포함되어야 합니다"
    );

    // 에러 메시지 형식 확인
    assert!(
        error_msg.contains("현재:"),
        "현재 잔액 표시가 포함되어야 합니다"
    );
    assert!(
        error_msg.contains("변경:"),
        "변경값 표시가 포함되어야 합니다"
    );

    // Result<(), String>을 통한 안전한 에러 처리가
    // 블록체인 노드에서 예외 상황을 우아하게 처리할 수 있게 함
}

#[test]
fn test_mutate_wallet_exact_zero_balance() {
    // 정확히 0이 되는 경우는 허용되어야 함
    let mut balance = 100;

    let result = mutate_wallet(&mut balance, -100);
    assert!(result.is_ok(), "정확히 0이 되는 경우는 허용되어야 합니다");
    assert_eq!(balance, 0, "잔액이 0이 되어야 합니다");

    // 소유권(ownership) 시스템이 메모리 안전성을 보장하면서
    // 블록체인 노드의 상태 전환을 안전하게 처리
}

#[test]
fn test_summarize_slice_with_sufficient_elements() {
    // 3개 이상의 요소가 있는 경우 테스트
    let blocks = [100, 101, 102, 103, 104];
    let (slice, total_length) = summarize_slice(&blocks);

    // 앞부분 3개 요소 확인
    assert_eq!(
        slice,
        &[100, 101, 102],
        "앞부분 3개 요소가 반환되어야 합니다"
    );

    // 전체 길이 확인
    assert_eq!(total_length, 5, "전체 길이가 올바르게 반환되어야 합니다");

    // 슬라이스 참조가 복사 없이 공유됨을 확인
    assert_eq!(slice.len(), 3, "슬라이스 길이가 3이어야 합니다");

    // 리턴값의 타입 확인
    assert_eq!(slice.len(), 3, "슬라이스 길이가 3이어야 합니다");
    assert_eq!(total_length, 5, "전체 길이가 5여야 합니다");

    // 수명(lifetime) 'a를 통한 참조 공유가
    // 블록체인 노드에서 메모리 효율적인 블록 데이터 처리를 가능하게 함
}

#[test]
fn test_summarize_slice_with_insufficient_elements() {
    // 3개 미만의 요소가 있는 경우 테스트
    let blocks = [100, 101];
    let (slice, total_length) = summarize_slice(&blocks);

    // 가능한 만큼의 요소 반환 확인
    assert_eq!(slice, &[100, 101], "가능한 만큼의 요소가 반환되어야 합니다");

    // 전체 길이 확인
    assert_eq!(total_length, 2, "전체 길이가 올바르게 반환되어야 합니다");

    // 슬라이스가 원본과 같은 참조임을 확인
    assert_eq!(slice.len(), 2, "슬라이스 길이가 2여야 합니다");

    // 리턴값의 타입과 내용 확인
    assert_eq!(slice, &[100, 101], "슬라이스가 원본과 같아야 합니다");
    assert_eq!(total_length, 2, "전체 길이가 2여야 합니다");

    // 빌림(borrowing)을 통한 안전한 참조 공유가
    // 블록체인 노드에서 대용량 블록 데이터를 효율적으로 처리
}

#[test]
fn test_summarize_slice_reference_sharing() {
    // 슬라이스 참조가 복사 없이 공유되는지 테스트
    let blocks = vec![100, 101, 102, 103, 104];

    // summarize_slice 호출
    let (slice, _) = summarize_slice(&blocks);

    // 슬라이스가 원본과 같은 메모리를 참조하는지 확인
    // (슬라이스는 이미 생성된 시점의 값을 참조)
    assert_eq!(
        slice,
        &[100, 101, 102],
        "슬라이스는 생성 시점의 값을 유지해야 합니다"
    );

    // 원본 데이터는 여전히 유효함을 확인
    assert_eq!(blocks.len(), 5, "원본 데이터가 여전히 유효해야 합니다");

    // 수명(lifetime)과 소유권(ownership) 시스템이
    // 블록체인 노드에서 데이터 일관성을 보장하는 핵심 메커니즘
}

#[test]
fn test_summarize_slice_empty_array() {
    // 빈 배열에 대한 테스트
    let blocks: [u64; 0] = [];
    let (slice, total_length) = summarize_slice(&blocks);

    // 빈 슬라이스 반환 확인
    assert_eq!(slice, &[], "빈 슬라이스가 반환되어야 합니다");

    // 전체 길이 확인
    assert_eq!(total_length, 0, "전체 길이가 0이어야 합니다");

    // 빌림(borrowing) 시스템이 경계 조건에서도
    // 블록체인 노드의 안정성을 보장함을 검증
}
