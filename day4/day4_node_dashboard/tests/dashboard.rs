use day4_node_dashboard::{NodePeer, count_uninitialized, fastest_peer, summarize_slots};

#[test]
// count_uninitialized가 초기화되지 않은 피어 수를 정확하게 센다.
fn test_count_uninitialized() {
    let peers = vec![
        NodePeer {
            name: "Alice".to_string(),
            last_slot: Some(100),
            latency_ms: 50,
        },
        NodePeer {
            name: "Bob".to_string(),
            last_slot: None,
            latency_ms: 30,
        },
        NodePeer {
            name: "Charlie".to_string(),
            last_slot: Some(200),
            latency_ms: 80,
        },
        NodePeer {
            name: "David".to_string(),
            last_slot: None,
            latency_ms: 40,
        },
    ];

    assert_eq!(count_uninitialized(&peers), 2);
}

#[test]
// fastest_peer가 최소 지연 시간을 가진 피어를 찾아 준다.
fn test_fastest_peer_finds_minimum_latency() {
    let peers = vec![
        NodePeer {
            name: "Alice".to_string(),
            last_slot: Some(100),
            latency_ms: 50,
        },
        NodePeer {
            name: "Bob".to_string(),
            last_slot: None,
            latency_ms: 30,
        },
        NodePeer {
            name: "Charlie".to_string(),
            last_slot: Some(200),
            latency_ms: 80,
        },
    ];

    let fastest = fastest_peer(&peers);
    assert!(fastest.is_some());
    assert_eq!(fastest.unwrap().name, "Bob");
    assert_eq!(fastest.unwrap().latency_ms, 30);
}

#[test]
// 피어가 없을 때 None을 돌려준다.
fn test_fastest_peer_empty_list() {
    let peers: Vec<NodePeer> = vec![];

    let fastest = fastest_peer(&peers);
    assert!(fastest.is_none());
}

#[test]
// summarize_slots가 슬롯이 있는 피어와 없는 피어를 구분해 예상 문장을 만든다.
fn test_summarize_slots() {
    let peers = vec![
        NodePeer {
            name: "Alice".to_string(),
            last_slot: Some(100),
            latency_ms: 50,
        },
        NodePeer {
            name: "Bob".to_string(),
            last_slot: None,
            latency_ms: 30,
        },
        NodePeer {
            name: "Charlie".to_string(),
            last_slot: Some(200),
            latency_ms: 80,
        },
    ];

    let summaries = summarize_slots(&peers);

    // 벡터 길이 확인
    assert_eq!(summaries.len(), 3);

    // 각 항목 문자열 확인
    assert_eq!(summaries[0], "Alice synced up to slot 100");
    assert_eq!(summaries[1], "Bob awaiting first block");
    assert_eq!(summaries[2], "Charlie synced up to slot 200");
}
