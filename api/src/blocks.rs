use crate::server::*;
use async_trait::async_trait;
use core::ops::Range;
use futures::future::{BoxFuture, FutureExt};
use std::sync::Arc;

///
///
/// Trait to implement double linked list on Block type
#[async_trait]
pub trait Blocks {
    fn get_block_header_at(&mut self, height: u32) -> Option<Block>;

    async fn build_block_transactions(
        &self,
        block_header: BlockHeader,
        height: u32,
    ) -> Result<Block, StateTransitionError>;

    async fn build_blocks_parallel(
        self: Arc<Self>,
        block_height_range: Range<u32>,
    ) -> Result<Vec<Block>, ServerError>;

    // #[async_recursion(?Send)]
    fn build_blocks_backward(
        &self,
        blocks: Vec<Block>,
        block_height_range: Range<u32>,
    ) -> BoxFuture<'_, Result<Vec<Block>, ServerError>>;

    // #[async_recursion(?Send)]
    fn build_blocks_forward(
        &self,
        blocks: Vec<Block>,
        block_height_range: Range<u32>,
    ) -> BoxFuture<'_, Result<Vec<Block>, ServerError>>;
}

#[async_trait]
impl Blocks for BlockList {
    /// Get the block at some height
    ///
    /// (Not used)
    fn get_block_header_at(&mut self, height: u32) -> Option<Block> {
        for block in self.iter() {
            if block.header.block_height == height {
                return Some(block);
            }
        }
        return None;
    }

    /// Build the build independently X does not depend on X - 1
    ///
    /// Spawn an async thread for each block height
    async fn build_blocks_parallel(
        self: Arc<Self>,
        block_height_range: Range<u32>,
    ) -> Result<Vec<Block>, ServerError> {
        let mut blocks = vec![];
        let res_blocks: Vec<_> = block_height_range
            .into_iter()
            .map(|i| {
                let s = self.clone();
                return tokio::spawn(async move {
                    let block_header = s.block_headers(i..i + 1).await;
                    let header = block_header
                        .map_err(|e| e.to_string())
                        .expect("header returned from the server, check if your range is correct"); // needs better error handling
                    let res_block: Vec<_> = header
                        .clone()
                        .into_iter()
                        .map(|bh| {
                            return s.build_block_transactions(bh, i);
                        })
                        .collect();
                    let mut new_block = futures::future::join_all(res_block).await;
                    return new_block.pop().unwrap();
                });
            })
            .collect();
        let results = futures::future::join_all(res_blocks).await;
        for res in results {
            if let Ok(block) = res {
                blocks.push(block.clone().unwrap())
            }
        }
        Ok(blocks)
    }

    /// Request the transactions for a given block height
    ///
    /// Build the block from the returned transactions from the server and the given block header
    async fn build_block_transactions(
        &self,
        block_header: BlockHeader,
        height: u32,
    ) -> Result<Block, StateTransitionError> {
        if !block_header.verify() {
            return Err(StateTransitionError);
        }
        let block_transactions = self.block_transactions(height..height + 1).await;
        let transactions = block_transactions
            .map_err(|e| e.to_string())
            .expect("transactions returned from the server; check if your range is correct");
        let mut txns: Vec<_> = transactions
            .into_iter()
            .map(|txs| match validate_block_transactions(txs.clone()) {
                Ok(()) => {
                    return Ok(Block {
                        header: block_header,
                        transactions: txs,
                    });
                }
                Err(_) => return Err(StateTransitionError),
            })
            .collect();
        if let Some(block) = txns.pop() {
            return block;
        }
        return Err(StateTransitionError);
    }

    /// Build the block where X depends on X - 1
    ///
    /// Recursive backward
    fn build_blocks_backward(
        &self,
        blocks: Vec<Block>,
        block_height_range: Range<u32>,
    ) -> BoxFuture<'_, Result<Vec<Block>, ServerError>> {
        async move {
            if block_height_range.start == block_height_range.end {
                return Ok(blocks);
            }
            let mut iter = self.iter();
            if iter.next_back().is_none() {
                return Ok(blocks);
            }
            // println!("build_blocks_backward {:#?}", block_height_range);
            let previous_block = self
                .build_blocks_backward(blocks, block_height_range.start..block_height_range.end - 1)
                .await;
            if previous_block.as_ref().ok().is_some() {
                iter.next();
                let block_header = self
                    .block_headers(block_height_range.end - 1..block_height_range.end)
                    .await;
                let header = block_header.map_err(|e| e.to_string()).unwrap();
                let res2: Vec<_> = header
                    .clone()
                    .into_iter()
                    .map(|bh| {
                        return self.build_block_transactions(bh, block_height_range.end - 1);
                    })
                    .collect();
                let mut res = futures::future::join_all(res2).await;
                match res.remove(0) {
                    Ok(b) => {
                        let mut previous_blocks = previous_block.unwrap();
                        previous_blocks.push(b);
                        return Ok(previous_blocks);
                    }
                    Err(_) => return Err(ServerError),
                }
            }
            return Err(ServerError);
        }
        .boxed()
    }

    // #[async_recursion(?Send)]
    /// Build the block where X depends on X - 1
    ///
    /// Recursive forward
    ///
    /// Recursive tail terminal but not to consider
    fn build_blocks_forward(
        &self,
        mut blocks: Vec<Block>,
        block_height_range: Range<u32>,
    ) -> BoxFuture<'_, Result<Vec<Block>, ServerError>> {
        async move {
            if block_height_range.start == block_height_range.end {
                return Ok(blocks);
            }
            let mut iter = self.iter();
            let block_header = self
                .block_headers(block_height_range.end - 1..block_height_range.end)
                .await;
            // println!("build_blocks_backward {:#?}", block_height_range);
            let header = block_header.map_err(|e| e.to_string()).unwrap(); // check the return
            let res2: Vec<_> = header
                .clone()
                .into_iter()
                .map(|bh| {
                    return self.build_block_transactions(bh, block_height_range.end - 1);
                })
                .collect();
            let mut res = futures::future::join_all(res2).await;
            let d = res.remove(0);
            if d.as_ref().ok().is_some() {
                if iter.next().is_none() {
                    return Ok(blocks);
                }
                let new_block = d.unwrap();
                blocks.push(new_block);
                return self
                    .build_blocks_forward(
                        blocks,
                        block_height_range.start..block_height_range.end - 1,
                    )
                    .await;
            }
            return Err(ServerError);
        }
        .boxed()
    }
}

fn validate_block_transactions(transactions: Vec<Transaction>) -> Result<(), StateTransitionError> {
    for transaction in transactions.iter() {
        let validate = transaction.execute();
        match validate {
            Ok(()) => continue,
            Err(_e) => return Err(StateTransitionError {}),
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    // #[test]
    #[tokio::test]
    // mock the server
    async fn server_api() {
        let mut server = MockBlockList::new();
        let _ = server.expect_block_headers().returning(move |_| {
            let header = BlockHeader::default();
            Ok(vec![header])
        });
        let _ = server.expect_block_transactions().returning(move |_| {
            let txns = vec![Transaction::default()];
            Ok(vec![txns])
        });

        let mut list = BlockList::new();
        let txns = vec![Transaction::default()];
        let b = Block {
            header: BlockHeader::default(),
            transactions: txns,
        };
        list.insert_at_tail(b.clone());

        let block = list
            .build_block_transactions(BlockHeader::default(), 0)
            .await;
        assert_eq!(block.clone().unwrap(), b.clone());
        assert_eq!(block.unwrap().header.block_height, 0);
    }
}
