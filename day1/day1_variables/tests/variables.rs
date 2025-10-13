use day1_variables::{describe_mutability, shadow_example, move_semantics_demo};

#[test]
fn test_describe_mutability() {
    let result = describe_mutability();
    
    // README 요구사항 1: 문자열이 반환되는지 확인
    assert!(!result.is_empty());
    
    // README 요구사항 2: 불변 변수와 가변 변수 설명이 포함되어 있는지 확인
    assert!(result.contains("불변 변수") && result.contains("immutable"));
    assert!(result.contains("가변 변수") && result.contains("mutable"));
    
    // README 요구사항 3: 예시 코드 조각이 포함되어 있는지 확인
    assert!(result.contains("let x = 5"));
    assert!(result.contains("let mut y"));
}

#[test]
fn test_shadow_example() {
    let result = shadow_example();
    
    // README 요구사항 1: 벡터가 반환되는지 확인
    assert!(!result.is_empty());
    
    // README 요구사항 2: 같은 이름의 변수를 섀도잉하여 타입이 바뀌는 과정을 보여주는지 확인
    assert!(result.len() >= 3); // 최소 3단계의 섀도잉 과정
    
    // README 요구사항 3: 각 단계의 값이 담겨 있는지 확인
    for step in &result {
        assert!(!step.is_empty()); // 각 단계에 값이 있는지 확인
    }
    
    // README 요구사항 4: 타입이 바뀌는 과정을 보여주는지 확인
    let has_type_change = result.iter().any(|step| step.contains("타입:"));
    assert!(has_type_change, "타입 변경 과정이 표시되어야 합니다");
    
    // README 요구사항 5: 섀도잉 관련 설명이 포함되어 있는지 확인
    let has_shadowing_explanation = result.iter().any(|step| 
        step.contains("섀도잉") || step.contains("shadowing")
    );
    assert!(has_shadowing_explanation, "섀도잉 설명이 포함되어야 합니다");
}

#[test]
fn test_move_semantics_demo() {
    let result = move_semantics_demo();
    
    // README 요구사항 1: Result<(), String> 형태로 표현되는지 확인
    assert!(result.is_err(), "함수는 Err를 반환해야 합니다");
    
    // README 요구사항 2: 실패 메시지를 Err로 반환하는지 확인
    let error_msg = result.unwrap_err();
    assert!(!error_msg.is_empty(), "에러 메시지가 비어있지 않아야 합니다");
    
    // README 요구사항 3: 소유권 이동 관련 설명이 포함되어 있는지 확인
    assert!(error_msg.contains("소유권") || error_msg.contains("ownership"), 
        "소유권(ownership) 관련 설명이 포함되어야 합니다");
    
    // README 요구사항 4: 한글/영어 병기 형태로 작성되어 있는지 확인
    let has_bilingual = error_msg.contains("소유권") && error_msg.contains("ownership");
    assert!(has_bilingual, "한글/영어 병기 형태로 작성되어야 합니다");
}
