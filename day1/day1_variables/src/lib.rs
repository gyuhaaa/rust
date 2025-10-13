// - `describe_mutability()` 함수는 불변 변수와 가변 변수의 차이를 설명하는 문자열을 반환합니다. 문자열 안에는 예시 코드 조각을 포함해야 합니다.
// - `shadow_example()` 함수는 같은 이름의 변수를 섀도잉하여 타입이 바뀌는 과정을 보여 주는 벡터를 반환합니다. 벡터에는 각 단계의 값이 담겨 있어야 합니다.
// - `move_semantics_demo()` 함수는 소유권이 이동한 이후 원래 변수를 사용할 수 없는 상황을 문자열 설명과 함께 `Result<(), String>` 형태로 표현해야 합니다. `Ok(())`를 반환하는 대신 실패 메시지를 `Err`로 반환하고, 테스트에서 그 메시지를 검증하세요.

// 불변 변수와 가변 변수의 차이를 설명하는 문자열을 반환
pub fn describe_mutability() -> String {
    let result = r#"
        let x = 5;
        x = 6; // 컴파일 오류

        let mut y = 10;
        y = 11; // 재할당

        x는 불변(immutable) 변수, y는 가변(mutable) 변수입니다.
        불변 변수는 값을 변경할 수 없으며, 가변 변수는 값을 변경할 수 있습니다.
    "#.to_string();

    // println!("{}", result);
    return result;
}

// 같은 이름의 변수를 섀도잉하여 타입이 바뀌는 과정을 보여 주는 벡터를 반환
pub fn shadow_example() -> Vec<String> {
    let mut result = Vec::new();

    // 불변 변수 (섀도잉 가능)
    let immutable_value = "gyuseon Min";
    result.push(format!("1. 불변 변수: 값: {}, 타입: {}", immutable_value, std::any::type_name_of_val(&immutable_value)));

    let immutable_value = immutable_value.to_string();
    result.push(format!("2. 뷸변 변수: 값: {}, 타입: {}", immutable_value, std::any::type_name_of_val(&immutable_value)));

    let immutable_value = immutable_value.len();
    result.push(format!("3. 불변 변수: 값: {}, 타입: {}", immutable_value, std::any::type_name_of_val(&immutable_value)));

    // 가변 변수 (섀도잉 가능)
    let mut mutable_value = "gyuseon Min";
    result.push(format!("4. 가변 변수: 값: {}, 타입: {}", mutable_value, std::any::type_name_of_val(&mutable_value)));

    mutable_value = "gyuseon";
    result.push(format!("5. 가변 변수: 값: {}, 타입: {}", mutable_value, std::any::type_name_of_val(&mutable_value)));

    let mutable_value = mutable_value.to_string();
    result.push(format!("6. 가변 변수: 값: {}, 타입: {}", mutable_value, std::any::type_name_of_val(&mutable_value)));

    let mutable_value = mutable_value.len();
    result.push(format!("7. 가변 변수: 값: {}, 타입: {}", mutable_value, std::any::type_name_of_val(&mutable_value)));

    // 상수 (섀도잉 불가능)
    const CONSTANT_VALUE: usize = 12;
    result.push(format!("8. 상수: 값: {}, 타입: {}", CONSTANT_VALUE, std::any::type_name_of_val(&CONSTANT_VALUE)));

    // let CONSTANT_VALUE = CONSTANT_VALUE.to_string();
    // result.push(format!("{}", CONSTANT_VALUE));

    // let CONSTANT_VALUE = CONSTANT_VALUE.len();
    // result.push(format!("{}", CONSTANT_VALUE));

    result.push(format!("변수(variable)는 섀도잉 가능하지만, 상수(constant)는 섀도잉 불가능합니다."));
    return result;
}

// 소유권이 이동한 이후 원래 변수를 사용할 수 없는 상황을 문자열 설명과 함께 `Result<(), String>` 형태로 표현해야 합니다. 
// `Ok(())`를 반환하는 대신 실패 메시지를 `Err`로 반환하고, 테스트에서 그 메시지를 검증하세요.
pub fn move_semantics_demo() -> Result<(), String> {
    let before = String::from("gyuseon");
    let after = before; // 소유권이 before에서 after로 이동
    println!("{}", after);

    Err(format!("소유권(ownership)이 before에서 after로 이동한 후 before 변수를 사용할 수 없습니다. \
    이는 러스트의 메모리 안전성을 위해 설계된 소유권 이동 규칙에 따른 것입니다."))
}

pub fn main() {
    describe_mutability();
    shadow_example();
    match move_semantics_demo() {
        Ok(_) => println!("테스트 통과"),
        Err(e) => println!("테스트 실패: {}", e),
    }
}