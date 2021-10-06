use crate::block::Block;

pub struct Blockchain {
    pub blocks: Vec<Block>,
    pub difficulty: u32,
    pub pool: Vec<Vec<u8>>,
}

impl Blockchain {
    pub fn new(difficulty: u32) -> Self {
        Blockchain {
            blocks: vec![Block::new("ORIGIN".as_bytes().to_vec(), 0, 0, None)],
            difficulty,
            pool: Vec::new(),
        }
    }

    pub fn add_to_pool(&mut self, data: Vec<u8>) {
        self.pool.push(data);
    }

    pub fn proof_of_work(&self, block: &Block) -> bool {
        let hash = block.hash();
        if block.hash != hash {
            log::error!("Wrong block hash - PoW cannot be verified!");
            return false;
        }
        if hash > u64::pow(2, 64 - self.difficulty) {
            log::error!("Hash value does not meet a requirement - PoW cannot be verified!");
            return false;
        }
        if block.phash.unwrap() != self.blocks.last().unwrap().hash {
            log::error!("Unknown previous hash value - PoW cannot be verified!");
            return false;
        }
        log::info!("PoW Verified!");
        true
    }

    pub fn add_to_chain(&mut self, block: Block) {
        if self.proof_of_work(&block) {
            self.blocks.push(block);
        }
    }

    pub fn mine(&mut self) {
        while !self.pool.is_empty() {
            let data = self.pool.pop().unwrap();
            let mut block = Block::new(data, 0, 0, Some(self.blocks.last().unwrap().hash));
            block.mine(self.difficulty);
            self.add_to_chain(block);
        }
    }
}
