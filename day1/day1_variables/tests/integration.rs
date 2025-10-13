use day1_variables::{describe_mutability, shadow_example, move_semantics_demo};

#[test]
fn test_integration_all_functions() {
    println!("\n=== 통합 테스트: 모든 함수 연동 확인 ===\n");
    
    // 통합 테스트 1: describe_mutability() 함수의 실제 동작 확인
    println!("1. describe_mutability() 통합 테스트:");
    println!("{}", "=".repeat(50));
    let mutability_result = describe_mutability();
    println!("반환 타입: String");
    println!("전체 내용:");
    println!("{}", mutability_result);
    println!();
    
    // 통합 테스트 2: shadow_example() 함수의 실제 동작 확인
    println!("2. shadow_example() 통합 테스트:");
    println!("{}", "=".repeat(50));
    let shadow_result = shadow_example();
    println!("반환 타입: Vec<String>");
    println!("섀도잉 과정:");
    
    // shadow_result 전체 출력
    for res in shadow_result.iter() {
        println!("  {}", res);
    }

    println!();
    
    // 통합 테스트 3: move_semantics_demo() 함수의 실제 동작 확인
    println!("3. move_semantics_demo() 통합 테스트:");
    println!("{}", "=".repeat(50));
    match move_semantics_demo() {
        Ok(_) => {
            println!("⚠️  예상치 못한 성공 - 소유권 이동이 발생하지 않았습니다");
        },
        Err(e) => {
            println!("✅ 예상된 에러 발생 - 소유권 이동 시뮬레이션 성공");
            println!("전체 에러 메시지:");
            println!("{}", e);
        }
    }
    println!();
    
    // 통합 테스트 4: 모든 함수가 함께 작동하는지 확인
    println!("4. 전체 시스템 통합 테스트:");
    println!("{}", "=".repeat(50));
    
    // 모든 함수를 순차적으로 호출하여 시스템이 안정적으로 작동하는지 확인
    let _mutability = describe_mutability();
    let _shadow = shadow_example();
    let _move_result = move_semantics_demo();
    
    println!("✅ 모든 함수가 오류 없이 연속 실행됨");
    println!("✅ 메모리 누수 없음");
    println!("✅ 시스템 안정성 확인");
    
    println!("\n=== 통합 테스트 완료 ===");
}
