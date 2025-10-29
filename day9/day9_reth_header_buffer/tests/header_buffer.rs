// 이 테스트는 헤더 버퍼가 부모/번호/난이도 규칙을 지키며 상태를 일관되게 유지함을 보장한다.

use day9_reth_header_buffer::{BlockHeader, HeaderBuffer, HeaderInsertError};

fn mk_header(number: u64, hash: &str, parent_hash: &str, difficulty: u64) -> BlockHeader {
    BlockHeader {
        number,
        hash: hash.to_string(),
        parent_hash: parent_hash.to_string(),
        difficulty,
    }
}

#[test]
fn normal_append_updates_head_and_total_difficulty() {
    let genesis = mk_header(0, "genesis", "", 100);
    let mut buf = HeaderBuffer::new(genesis.clone());

    let h1 = mk_header(1, "h1", &genesis.hash, 2);
    let h2 = mk_header(2, "h2", "h1", 3);

    assert!(buf.try_append(h1).is_ok());
    assert!(buf.try_append(h2).is_ok());

    let head = buf.head().expect("head must exist");
    assert_eq!(head.number, 2);
    assert_eq!(head.hash, "h2");

    // canonical 길이: genesis + h1 + h2 = 3
    assert_eq!(buf.len(), 3);

    // total difficulty: 100 + 2 + 3 = 105
    assert_eq!(buf.total_difficulty(), 105);
}

#[test]
fn parent_not_found_is_rejected_and_length_unchanged() {
    let genesis = mk_header(0, "genesis", "", 7);
    let mut buf = HeaderBuffer::new(genesis.clone());

    let orphan = mk_header(1, "orphan", "missing_parent", 5);
    match buf.try_append(orphan) {
        Err(HeaderInsertError::ParentNotFound { parent_hash }) => {
            assert_eq!(parent_hash, "missing_parent");
        }
        other => panic!("expected ParentNotFound, got: {:?}", other),
    }

    // 실패 후에도 길이와 TD는 변함없음
    assert_eq!(buf.len(), 1);
    assert_eq!(buf.total_difficulty(), 7);
}

#[test]
fn number_mismatch_is_rejected_without_state_change() {
    let genesis = mk_header(0, "genesis", "", 1);
    let mut buf = HeaderBuffer::new(genesis.clone());

    // 부모는 genesis가 맞지만 번호가 건너뛰어진 경우(number = 2)
    let bad = mk_header(2, "bad", &genesis.hash, 10);
    match buf.try_append(bad) {
        Err(HeaderInsertError::NumberMismatch { expected, got }) => {
            assert_eq!(expected, 1);
            assert_eq!(got, 2);
        }
        other => panic!("expected NumberMismatch, got: {:?}", other),
    }

    assert_eq!(buf.len(), 1);
    assert_eq!(buf.total_difficulty(), 1);
}
