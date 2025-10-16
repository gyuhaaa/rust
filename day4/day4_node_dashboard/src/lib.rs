// - 아래 함수를 순서대로 구현합니다. 각 함수 위에도 주석을 남겨 "이 함수가 어떤 문제를 해결하는지" 설명합니다.

// - `NodePeer` 구조체를 만들고 아래 필드를 정의합니다.
// - `pub name: String` — 피어의 이름입니다.
// - `pub last_slot: Option<u64>` — 마지막으로 보고된 슬롯 번호입니다. 아직 블록을 못 받은 피어라면 `None`입니다.
// - `pub latency_ms: u64` — 왕복 지연 시간입니다.

// NodePeer 구조체는 우리가 피어(노드)들이 어떻게 작동하는지 이해하고, 가장 빠른 피어(노드)를 찾아내는 데 도와줍니다.
// It helps us understand how the peers are functioning and find the fastest one.
#[derive(Debug)]
pub struct NodePeer {
    pub name: String,
    pub last_slot: Option<u64>,
    pub latency_ms: u64,
}

// 1. `pub fn count_uninitialized(peers: &[NodePeer]) -> usize`
//    - `last_slot`이 `None`인 피어가 몇 개인지 세어 반환합니다.
//    - `for` 반복문과 `if let`을 활용해 보세요.
// 이 함수는 아직 블록을 받지 못한 피어들의 수를 세어서 네트워크 동기화 상태를 파악합니다.
pub fn count_uninitialized(peers: &[NodePeer]) -> usize {
    let mut count = 0;
    for peer in peers {
        if let None = peer.last_slot {
            count += 1;
        }
    }
    count
}

// 2. `pub fn fastest_peer<'a>(peers: &'a [NodePeer]) -> Option<&'a NodePeer>`
//    - 지연 시간이 가장 낮은 피어에 대한 참조를 반환합니다.
//    - 피어가 하나도 없다면 `None`을 반환합니다.
//    - 최소값을 갱신할 때는 `match`나 `if let`으로 현재까지 찾은 피어를 비교해 주세요.
// 이 함수는 여러 피어 중에서 가장 빠른 응답 속도를 가진 피어를 찾아서 최적의 통신 파트너를 선택합니다.
pub fn fastest_peer<'a>(peers: &'a [NodePeer]) -> Option<&'a NodePeer> {
    let mut fastest_peer = None;
    let mut min_latency = u64::MAX;
    for peer in peers {
        if let Some(_current_fastest) = fastest_peer {
            if peer.latency_ms < min_latency {
                min_latency = peer.latency_ms;
                fastest_peer = Some(peer);
            }
        } else {
            min_latency = peer.latency_ms;
            fastest_peer = Some(peer);
        }
    }
    fastest_peer
}

// 3. `pub fn summarize_slots(peers: &[NodePeer]) -> Vec<String>`
//    - 각 피어를 순회하며 아래 규칙에 맞는 설명 문장을 만들어 `Vec<String>`으로 모읍니다.
//      - 슬롯을 알고 있는 경우: `format!("{} synced up to slot {}", name, slot)`
//      - 슬롯을 모르는 경우: `format!("{} awaiting first block", name)`
//    - 이 함수는 "여러 값 -> 보고용 문자열" 패턴을 반복하며 익숙해지는 것이 목적입니다.
// 이 함수는 각 피어의 동기화 상태를 사람이 읽기 쉬운 형태로 요약하여 네트워크 상태를 한눈에 파악할 수 있게 합니다.
pub fn summarize_slots(peers: &[NodePeer]) -> Vec<String> {
    let mut summaries = Vec::new();
    for peer in peers {
        if let Some(slot) = peer.last_slot {
            summaries.push(format!("{} synced up to slot {}", peer.name, slot));
        } else {
            summaries.push(format!("{} awaiting first block", peer.name));
        }
    }
    summaries
}
