# Day 3: `match`와 `Option`으로 노드 상태를 읽기 쉽게 표현하기

**난이도: EASY (아주 쉬워요!)**

## [전날 과제 요약]
- `day2_ownership` 라이브러리에서 불변/가변 참조와 수명 개념을 테스트로 검증하며, 안전한 데이터 공유 패턴을 학습했습니다.

## [전날 과제로 얻은 역량]
- 소유권과 빌림 규칙을 기반으로 함수 설계를 하고, 블록체인 노드 상태 변경 시 발생할 수 있는 데이터 경합을 예방하는 방법을 익혔습니다.

## [오늘 과제 목표]
- `match` 표현식으로 여러 경우의 수를 간결하게 표현한다.
- `Option`과 `Result` 타입을 사용해 값이 없거나 실패한 상황을 안전하게 처리한다.
- 테스트 주도 접근으로 간단한 상태 보고 함수를 설계한다.

## [오늘 과제 설명]
1. `cargo new day3_match --lib` 명령으로 새로운 라이브러리 프로젝트를 생성하세요.
2. `src/lib.rs`에 다음 함수를 구현하세요. 모든 함수에는 블록체인 검증 노드 운영과 연관된 한/영 병기 주석을 한 줄 이상 추가합니다.
   - `pub fn describe_slot(slot: Option<u64>) -> String`: 슬롯 번호가 `Some`이면 "Processing slot <번호>" 형식으로, `None`이면 "No slot scheduled"를 반환합니다.
   - `pub fn guard_height(height: Result<u64, String>) -> u64`: 성공(`Ok`)이면 값을 그대로 반환하고, 실패(`Err`)이면 기본 높이 `0`을 반환합니다.
   - `pub fn classify_latency(ms: u64) -> &'static str`: 지연 시간이 0~99ms이면 "Fast", 100~299ms이면 "Normal", 그 이상이면 "Slow"를 반환합니다.
3. `tests/report.rs` 파일을 생성하고 아래 사항을 검증하세요.
   - `describe_slot`이 `Some`과 `None` 입력 모두에서 기대 문자열을 생성하는지 테스트합니다.
   - `guard_height`가 `Result`의 두 가지 경우를 안전하게 처리하는지 확인합니다.
   - `classify_latency`가 경계값(99ms, 100ms, 299ms, 300ms)을 올바르게 분류하는지 테스트합니다.
4. 모든 테스트 코드에도 `match`, `Option`, `Result`가 왜 블록체인 상태 보고에 유용한지 주석을 남겨 주세요.
5. `cargo fmt`와 `cargo test`를 실행하여 포맷과 테스트가 통과함을 확인하세요.
6. README 마지막에는 학습자가 오늘 느낀 점을 적을 수 있는 학습 일지 공간을 남겨 주세요.

## [이해를 돕기 위한 예시]
```rust
fn map_message(code: Option<u8>) -> &'static str {
    match code {
        Some(0) => "All good",       // 메시지 코드가 있을 때는 바로 해석 가능
        Some(_) => "Needs review",   // 다른 값은 확인이 필요함을 알려줌
        None => "No data",           // 값이 없으면 안전하게 기본 메시지를 사용
    }
}
```
- 위 예시는 `match`와 `Option`을 사용해 누락된 값을 안전하게 처리하는 기본 패턴을 보여 줍니다.
- 과제에서 구현할 함수들도 동일한 사고방식을 유지하며, 노드 상태를 사람과 기계 모두에게 이해하기 쉬운 메시지로 바꾸게 됩니다.

---

### 학습자 학습 일지 메모
- (학습자가 작성할 공간)
