# Day 3 Hint

## [힌트]
- `match` 표현식은 모든 경우를 반드시 다뤄야 하므로, `_` 패턴을 사용하면 빠르게 기본 동작을 정의할 수 있습니다.
- `Option`과 `Result`는 `unwrap` 대신 안전한 기본값이나 메시지를 선택할 때 유용합니다. 실무에서는 `match`나 `if let`으로 분기하는 습관을 들이세요.
- 테스트에서 경계값을 강조하면, 나중에 복잡한 합의 로직을 작성할 때도 버그를 미리 차단할 수 있습니다.

## [참고자료]
- [The Rust Programming Language - Enums and Pattern Matching](https://doc.rust-lang.org/book/ch06-00-enums.html)
- [Rust by Example - Option and Result](https://doc.rust-lang.org/rust-by-example/std/option.html)
- [Solana Validator Monitoring Guide](https://docs.solana.com/validator/monitoring) - 어떤 상태를 관찰하는지 참고해보세요.

## [참고 키워드]
- `match expression`
- `Option<T>`
- `Result<T, E>`
- Latency monitoring
