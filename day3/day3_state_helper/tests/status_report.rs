use day3_state_helper::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // Some과 None 케이스 모두에서 올바른 메시지를 반환하는지 확인합니다
    fn test_describe_sync_height() {
        // Some 케이스 테스트 - 값이 있을 때
        assert_eq!(describe_sync_height(Some(100)), "Current height: 100");
        assert_eq!(describe_sync_height(Some(0)), "Current height: 0");
        assert_eq!(describe_sync_height(Some(999999)), "Current height: 999999");

        // None 케이스 테스트 - 값이 없을 때
        assert_eq!(describe_sync_height(None), "No blocks yet");
    }

    #[test]
    // 성공과 실패 케이스를 안전하게 처리하는지 확인합니다
    fn test_fallback_slot() {
        // 성공 케이스 테스트 - Ok 값이 그대로 반환되는지
        assert_eq!(fallback_slot(Ok(42)), 42);
        assert_eq!(fallback_slot(Ok(0)), 0);
        assert_eq!(fallback_slot(Ok(100)), 100);

        // 실패 케이스 테스트 - Err일 때 기본값 0이 반환되는지
        assert_eq!(fallback_slot(Err("Network error".to_string())), 0);
        assert_eq!(fallback_slot(Err("Timeout".to_string())), 0);
        assert_eq!(fallback_slot(Err("".to_string())), 0);
    }

    #[test]
    // 경계값들을 올바르게 분류하는지 확인합니다
    fn test_classify_peer_speed() {
        // Instant 범위 테스트 (0~150ms)
        assert_eq!(classify_peer_speed(0), "Instant");
        assert_eq!(classify_peer_speed(75), "Instant");
        assert_eq!(classify_peer_speed(150), "Instant");

        // Acceptable 범위 테스트 (151~400ms)
        assert_eq!(classify_peer_speed(151), "Acceptable");
        assert_eq!(classify_peer_speed(275), "Acceptable");
        assert_eq!(classify_peer_speed(400), "Acceptable");

        // Lagging 범위 테스트 (401ms 이상)
        assert_eq!(classify_peer_speed(401), "Lagging");
        assert_eq!(classify_peer_speed(500), "Lagging");
        assert_eq!(classify_peer_speed(1000), "Lagging");
    }
}
