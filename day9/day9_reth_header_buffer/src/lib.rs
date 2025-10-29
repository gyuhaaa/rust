use std::collections::HashMap;

// BlockHeader는 Reth의 블록 헤더가 담는 핵심 정보를 단순화하여 모방합니다.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BlockHeader {
    // 블록 번호
    pub number: u64,
    // 블록 해시
    pub hash: String,
    // 부모 해시
    pub parent_hash: String,
    // 난이도
    pub difficulty: u64,
}

pub struct HeaderBuffer {
    // canonical: Reth의 CanonicalHeaders(정준 체인 테이블)과 유사한 메모리 시퀀스
    canonical: Vec<BlockHeader>,
    // index_by_hash: Reth DB의 해시→헤더 조회 인덱스와 유사(빠른 parent/hash lookup)
    index_by_hash: HashMap<String, usize>,
    // total_difficulty: Reth의 fork choice 지표(TD) 누적 값(가장 긴 체인 비교용)
    total_difficulty: u128,
}

impl HeaderBuffer {
    // 제네시스는 이전 블록이 없기 때문에 특별 취급된다.
    pub fn new(genesis: BlockHeader) -> Self {
        let mut index_by_hash = HashMap::new();
        // 제네시스는 체인의 기준점이므로 가장 먼저 인덱싱된다.
        index_by_hash.insert(genesis.hash.clone(), 0);
        let td = genesis.difficulty as u128;

        Self {
            canonical: vec![genesis],
            index_by_hash,
            // total difficulty는 제네시스의 난이도부터 포함해 누적한다.
            total_difficulty: td,
        }
    }
    pub fn head(&self) -> Option<&BlockHeader> {
        self.canonical.last()
    }
    pub fn total_difficulty(&self) -> u128 {
        self.total_difficulty
    }

    // canonical 길이를 노출해 외부 테스트에서 체인 길이 검증이 가능하도록 한다.
    pub fn len(&self) -> usize {
        self.canonical.len()
    }

    // clippy 권고: len()이 있으면 is_empty()도 제공해 API 일관성을 높인다.
    pub fn is_empty(&self) -> bool {
        self.canonical.is_empty()
    }
}

#[derive(Debug)]
pub enum HeaderInsertError {
    // 부모 해시가 없는 경우
    ParentNotFound { parent_hash: String },
    // 번호가 일치하지 않는 경우
    NumberMismatch { expected: u64, got: u64 },
    // 해시가 중복된 경우
    DuplicateHash { hash: String },
}

impl HeaderBuffer {
    pub fn try_append(&mut self, header: BlockHeader) -> Result<(), HeaderInsertError> {
        if self.index_by_hash.contains_key(&header.hash) {
            return Err(HeaderInsertError::DuplicateHash {
                hash: header.hash.clone(),
            });
        }
        if !self.index_by_hash.contains_key(&header.parent_hash) {
            return Err(HeaderInsertError::ParentNotFound {
                parent_hash: header.parent_hash.clone(),
            });
        }
        if header.number != self.head().unwrap().number + 1 {
            return Err(HeaderInsertError::NumberMismatch {
                expected: self.head().unwrap().number + 1,
                got: header.number,
            });
        }
        let new_index = self.canonical.len();
        // 새로운 헤더를 인덱싱하고 누적 난이도를 갱신한 뒤 canonical에 추가한다.
        self.index_by_hash.insert(header.hash.clone(), new_index);
        self.total_difficulty = self
            .total_difficulty
            .saturating_add(header.difficulty as u128);
        self.canonical.push(header);

        Ok(())
    }
}
