use async_trait::async_trait;
use core::ops::Range;
use list::linked_list::*;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Fields required by the consensus to validate the block.
///
/// For simplicity, it is a dummy structure. We don't need to
// verify the block header validity.
#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct ConsensusFields;

/// Fields of the transaction that cause some state transition of the blockchain.
///
/// For simplicity, it is a dummy structure. We don't need to
// implement state transition.
#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct TransactionFields;

/// The identifier of the transaction in the database and the
// network.
pub type TransactionId = [u8; 32];

/// The header of the block that describes the final state of the blockchain at `block_height`.
#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize, PartialEq)]
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

/// The error that describe failed state transition.
#[derive(Debug, Clone)]
pub struct StateTransitionError;

impl fmt::Display for StateTransitionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "State transition error")
    }
}

/// The transaction causes a state transition on the blockchain.
#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize, PartialEq)]
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

/// The block that contains transactions and the header of the blockchain state.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct Block {
    pub header: BlockHeader,
    pub transactions: Vec<Transaction>,
}

/// The error that describes failure on the server side.
#[derive(Debug, Serialize)]
pub struct ServerError;

impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Server error")
    }
}

/// The API is supported by the server.
#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait ServerAPI {
    /// Return the list of headers for provided
    // `block_height_range`.
    ///
    /// The endpoint guarantees that the ordering of the header is
    // right and headers are connected to each other.
    async fn block_headers(
        &self,
        block_height_range: Range<u32>,
    ) -> Result<Vec<BlockHeader>, ServerError>;

    /// Return the list of transactions per each block height from
    // `block_height_range`.
    ///
    /// Each element is a `Vec<Transaction>` that belongs to the
    // corresponding block height.
    /// The endpoint guarantees that the ordering of transactions
    // is right according to the `block_height_range` .
    async fn block_transactions(
        &self,
        block_height_range: Range<u32>,
    ) -> Result<Vec<Vec<Transaction>>, ServerError>;
}

///
/// Double linked list on blocks to support the data to request
///
pub type BlockList = DoubleLinkedList<Block>;

///
/// Implements BlockList for the serverAPI trait
///
#[cfg_attr(test, mockall::automock)]
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
        for i in 0..h_end {
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
