// - `pub fn annotate_borrowing(message: &str) -> String`: 입력 문자열을 차용만 하는 함수로,
// 불변 참조가 안전한 이유를 설명하는 다중 라인 문자열을 반환합니다.
// 반환 문자열에는 최소한 "immutable reference"와 "no data race"라는 표현을 포함하세요.
pub fn annotate_borrowing(message: &str) -> String {
    // 함수 내부에서 message를 어떻게 사용하든 불변 참조로만 사용 가능

    // ✅ 모두 불변 참조 사용 (읽기만)
    let length = message.len(); // 불변 참조로 길이 읽기
    let _first_char = message.chars().next(); // 불변 참조로 첫 글자 읽기
    let _upper = message.to_uppercase(); // 불변 참조로 새 String 생성
    let _contains = message.contains("hello"); // 불변 참조로 검색

    // ✅ 불변 참조를 다른 변수에 할당
    let _msg = message; // 같은 불변 참조 복사
    let _another_ref = &message; // 불변 참조의 불변 참조

    // ❌ 수정은 불가능 (컴파일 에러)
    // message.push('!');  // &str은 수정 불가
    // *message = "new";   // 불변 참조는 수정 불가

    format!(
        "불변 참조(immutable reference)는 동시에 여러 개 존재해도 안전합니다.
        
        안전한 이유:
        1. 읽기 전용 보장 - 데이터 변경이 불가능하여 일관성 유지
        2. 동시 접근 안전성 - 여러 스레드가 동시에 읽어도 문제없음
        3. 컴파일 타임 검사 - 런타임 오류를 컴파일 시점에 방지
        4. 메모리 효율성 - 복사 없이 참조만 사용하여 성능 최적화
        5. Data Race 방지 - 쓰기 연산이 없으므로 데이터 경합(no data race) 불가능
        * 데이터 경합(Data Race): 둘 이상의 프로세스나 스레드가 동일한 데이터에 접근하고 쓰는 상황으로, 결과가 예측할 수 없게 만듭니다.
        
        블록체인 노드에서 불변 참조를 사용하는 것은 안전성, 예측 가능성, 그리고 병렬 처리 성능을 극대화하는 데 결정적인 이점을 제공합니다.
        이는 러스트의 'Fearless Concurrency' 철학을 구현하는 핵심 메커니즘입니다.
        
        입력 메시지 길이: {}자
        ",
        length
    )
}

// - `pub fn mutate_wallet(balance: &mut i64, delta: i64)`: 가변 참조를 받아 잔액을 수정합니다.
// 함수 내부에서 가변 빌림이 단일 스코프에서만 허용됨을 주석과 로직으로 보여 주세요.
// 음수 잔액이 될 경우 `panic!` 대신 `Result<(), String>`을 사용하여 오류 메시지를 반환하도록 하세요.
pub fn mutate_wallet(balance: &mut i64, delta: i64) -> Result<(), String> {
    // * 스코프(scope): 변수가 유효한 범위

    // 블록체인 노드 상태 업데이트 이점:
    // - 가변 참조의 단일 스코프 제한으로 동시성 안전성 보장
    // - 트랜잭션 처리 시 데이터 경합(data race) 완전 방지
    // - 블록체인 노드에서 안전한 상태 업데이트 보장
    // - Result<(), String>을 통한 우아한 에러 처리로 노드 안정성 향상
    // - 컴파일 타임에 메모리 안전성 보장으로 런타임 오류 사전 차단

    // 음수 잔액 체크 (수정 전에 검증)
    if *balance + delta < 0 {
        return Err(format!(
            "잔액은 음수가 될 수 없습니다. 현재: {}, 변경: {}",
            *balance, delta
        ));
    }

    // 가변 빌림의 단일 스코프 제한을 보여주는 로직(중괄호 없이도 동작)
    {
        // balance는 함수 매개변수 - 함수 전체 스코프에서 유효
        *balance += delta;

        // ❌ 컴파일 에러 발생 (단일 스코프 제한)
        // let another_ref = &mut *balance;  // 동시에 두 개의 가변 참조 생성 불가
        // let second_ref = balance;          // 가변 참조 복사 불가

        // ✅ 불변 참조는 여러 개 가능
        // 이 변수들만 중괄호 스코프에서 생성/소멸
        let _read_only1 = &*balance; // 불변 참조 1
        let _read_only2 = &*balance; // 불변 참조 2
        let _read_only3 = &*balance; // 불변 참조 3
    } // 스코프 종료 (_read_only 변수들 소멸)

    Ok(())
}

// - `pub fn summarize_slice<'a>(blocks: &'a [u64]) -> (&'a [u64], usize)`:
// 블록 높이 목록을 슬라이스로 받아, 앞부분 3개의 요소만 가리키는 서브 슬라이스(3개 미만이면 가능한 만큼)와 총 요소 수를 함께 반환합니다.
pub fn summarize_slice<'a>(blocks: &'a [u64]) -> (&'a [u64], usize) {
    // 블록체인 노드 상태 업데이트 이점:
    // - 수명(lifetime) 'a를 통한 안전한 참조 공유로 메모리 효율성 극대화
    // - 슬라이스 참조를 통한 복사 없는 데이터 접근으로 대용량 블록체인 데이터 처리 성능 향상
    // - 불변 참조를 통한 안전한 읽기 전용 접근으로 동시성 문제 완전 해결
    // - 블록체인 노드에서 최근 블록 정보를 메모리 효율적으로 제공하여 네트워크 성능 최적화
    if blocks.len() < 3 {
        return (blocks, blocks.len());
    } else {
        let slice = &blocks[..3];
        let len = blocks.len();
        (slice, len)
    }
}
