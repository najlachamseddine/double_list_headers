use async_trait::async_trait;
use core::ops::Range;
use std::fmt;
use mockall::automock;
use waitgroup::WaitGroup;
// use async_std::task::spawn;
// use tokio::spawn;

/// Fields required by the consensus to validate the block.
///
/// For simplicity, it is a dummy structure. We don't need to
// verify the block header validity.
pub struct ConsensusFields;

/// Fields of the transaction that cause some state transition of the blockchain.
///
/// For simplicity, it is a dummy structure. We don't need to
// implement state transition.
pub struct TransactionFields;

/// The identifier of the transaction in the database and the
// network.
pub type TransactionId = [u8; 32];

/// The header of the block that describes the final state of the blockchain at `block_height`.
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
/// The error that describe failed state transition.
pub struct StateTransitionError;

/// The transaction causes a state transition on the blockchain.
pub struct Transaction {
    pub tx_id: TransactionId,
    pub transaction_fields: TransactionFields,
}

impl Transaction {
    /// The function executes transaction and performance state
    // transition.
    pub fn execute(self) -> Result<(), StateTransitionError> {
        Ok(())
    }
}
/// The block that contains transactions and the header of the blockchain state.
pub struct Block {
    pub header: BlockHeader,
    pub transactions: Vec<Transaction>,
}

/// The error that describes failure on the server side.
pub struct ServerError;

impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Server error")
    }
}


/// The API is supported by the server.
#[automock]
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
} // The task is to fetch all [`Block`]s for the range `0..100_000`,
  // but:
  // - Before the requesting transactions, we need to call
  // [`BlockHeader::verify`]
  // and fetch transactions only if it returns `true`.
  // - Before combining [`BlockHeader`] and [`Vec<Transaction>`]
  // into [`Block`], we need to
  // iterate over each transaction and call
  // [`Transaction::execute`]. If all results are [`Result::Ok`],
  // then we can create a [`Block`].
  //
  // The goal of the task is to request data as fast as possible(in
  // parallel). Blocks can be executed
  // and verified independently. It means verification or execution
  // of the block
  // at height `X` can be done without block at height `X - 1`.
  //
  // An additional optional task: The same goal as before, but
  // verification/execution of
  // the block header/block at height `X` requires verification/
  // execution of
  // the block header/block at height `X - 1`.


async fn validate_block_transactions(transactions: Vec<Transaction>) -> Result<(), StateTransitionError> {
    let wg = WaitGroup::new();
    let res: Vec<_> = transactions.into_iter().map(|transaction| {
        let w = wg.worker();
        tokio::spawn(async move {
            let _ = match transaction.execute(){
                Ok(()) => Ok(()),
                Err(e) => Err(StateTransitionError),
            };
            // w.done();
            drop(w);
        });
    }).collect();
    wg.wait().await;
    Ok(())
}

     // let results = futures::future::join_all(res).await;
            // for r in results {
            //     let k = r;
            //     let k2 = k?;
            // }


#[cfg(test)]
mod tests {
    // use super::*;

    // #[test]
    // fn test_server_api() -> i32 {
    // let mut mock = MockServerAPI::new();
    // let mut block_headers:Vec<BlockHeader> = Vec::new();
    // let mut transactions:Vec<Vec<Transaction>> = Vec::new();
    // for i in 0..100 {
    //     block_headers.push(BlockHeader { block_height: i, consensus_fields: ConsensusFields{}});
    //     for j in 0..100 {
    //         transactions.push(Transaction{ tx_id: , transaction_fields});
    //     }
    // }

    
    // mock.expect_block_headers().returns(

    // )
    // }

    // let _ = mock.expect_block_headers();

    // #[test]
    // fn it_works() {
    //     let result = add(2, 2);
    //     assert_eq!(result, 4);
    // }
}

