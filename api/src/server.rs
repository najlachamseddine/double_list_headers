use async_trait::async_trait;
use list::linked_list::*;
use core::ops::Range;
use std::fmt;

#[derive(Debug, Default, Clone, Copy)]
pub struct ConsensusFields;

#[derive(Debug, Default, Clone, Copy)]
pub struct TransactionFields;

pub type TransactionId = [u8; 32];

#[derive(Debug, Default, Clone, Copy)]
pub struct BlockHeader {
    pub block_height: u32,
    pub consensus_fields: ConsensusFields,
}

impl BlockHeader {
    /// The function that verifies the block header validity.
    pub fn verify(&self) -> bool {
        true
    }
}

#[derive(Debug)]
pub struct StateTransitionError;

impl fmt::Display for StateTransitionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "State transition error")
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Transaction {
    pub tx_id: TransactionId,
    pub transaction_fields: TransactionFields,
}

impl Transaction {
    /// The function executes transaction and performance state
    // transition.
    pub fn execute(self) -> Result<(), StateTransitionError> {
        let a = 0;
        if a == 0 {
            return Ok(());
        }
        return Err(StateTransitionError);
    }
}

#[derive(Debug, Clone, Default)]
pub struct Block {
    pub header: BlockHeader,
    pub transactions: Vec<Transaction>,
}


#[derive(Debug)]
pub struct ServerError;

impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Server error")
    }
}

#[async_trait]
pub trait ServerAPI {
    // add async later
    async fn block_headers(
        &self,
        block_height_range: Range<u32>,
    ) -> Result<Vec<BlockHeader>, ServerError>;

    async fn block_transactions(
        &self,
        block_height_range: Range<u32>,
    ) -> Result<Vec<Vec<Transaction>>, ServerError>;
}

pub type BlockList = DoubleLinkedList<Block>;

#[async_trait]
impl ServerAPI for BlockList {
    // add async later
    async fn block_headers(
        &self,
        block_height_range: Range<u32>,
    ) -> Result<Vec<BlockHeader>, ServerError> {
        let mut headers = vec![];
        let h_start = block_height_range.start;
        let h_end = block_height_range.end;
        let mut iter = self.iter();
        for i in 0..=h_end {
            let block = iter.next();
            if block.is_none() {
                return Ok(headers);
            }
            if block.clone().unwrap().header.block_height != i {
                return Err(ServerError);
            }
            if i < h_start {
                continue;
            }
            headers.push(block.unwrap().header);
        }
        Ok(headers)
    }

    async fn block_transactions(
        &self,
        block_height_range: Range<u32>,
    ) -> Result<Vec<Vec<Transaction>>, ServerError> {
        let mut transactions = vec![];
        let h_start = block_height_range.start;
        let h_end = block_height_range.end;
        let mut iter = self.iter();
        for i in 0..h_end {
            let block = iter.next();
            if block.is_none() {
                return Ok(transactions);
            }
            if block.clone().unwrap().header.block_height != i {
                return Err(ServerError);
            }
            if i < h_start {
                continue;
            }
            transactions.push(block.unwrap().transactions)
        }
        Ok(transactions)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
