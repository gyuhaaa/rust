// 블록체인 동기화 높이를 사용자 친화적인 메시지로 변환합니다 / Convert blockchain sync height to user-friendly message
pub fn describe_sync_height(height: Option<u64>) -> String {
    match height {
        Some(value) => format!("Current height: {value}"),
        None => "No blocks yet".to_string(),
    }
}

// 슬롯 조회 실패 시 안전한 기본값을 제공합니다 / Provide safe default value when slot lookup fails
pub fn fallback_slot(slot: Result<u64, String>) -> u64 {
    match slot {
        Ok(value) => value,
        Err(_) => 0,
    }
}

// 피어의 응답 속도를 카테고리로 분류합니다 / Classify peer response speed into categories
pub fn classify_peer_speed(ms: u64) -> &'static str {
    match ms {
        0..=150 => "Instant",
        151..=400 => "Acceptable",
        401.. => "Lagging",
    }
}
