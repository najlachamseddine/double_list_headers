use async_trait::async_trait;
use core::ops::Range;
use std::sync::Arc;
use async_recursion::async_recursion;
use std::future::Future;
use std::pin::Pin;
// use list::linked_list::*;
use crate::server::*;
use futures::future::{BoxFuture, FutureExt};

// type BlockList = DoubleLinkedList<Block>;

#[async_trait]
pub trait Blocks {

    fn get_block_header_at(&mut self, height: u32) -> Option<Block>;

    async fn build_block_transactions(
        &self,
        block_header: BlockHeader,
        height: u32,
    ) -> Result<Block, StateTransitionError>;

    async fn build_blocks_parallel(self: Arc<Self>, block_height_range: Range<u32>) -> Result<Vec<Block>, ServerError>;
    
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
    fn get_block_header_at(&mut self, height: u32) -> Option<Block> {
        for block in self.iter() {
            if block.header.block_height == height {
                return Some(block);
            }
        }
        return None;
    }

    async fn build_blocks_parallel(
        self: Arc<Self>,
        block_height_range: Range<u32>,
    ) -> Result<Vec<Block>, ServerError> {
        let mut blocks = vec![];
        let res1: Vec<_> = block_height_range
            .into_iter()
            .map(|i| {
                let s = self.clone();
                return tokio::spawn(async move {
                    let block_header = s.block_headers(i..i + 1).await;
                    let header = block_header.map_err(|e| ServerError).unwrap(); // check the return
                    let res2: Vec<_> = header
                        .clone()
                        .into_iter()
                        .map(|bh| {
                            return s.build_block_transactions(bh, i);
                        })
                        .collect();
                    let mut res = futures::future::join_all(res2).await;
                    return res.remove(0).unwrap();
                });
            })
            .collect();
        let res = futures::future::join_all(res1).await;
        for r in res {
            blocks.push(r.unwrap().clone())
        }
        Ok(blocks)
    }

    async fn build_block_transactions(
        &self,
        block_header: BlockHeader,
        height: u32,
    ) -> Result<Block, StateTransitionError> {
        if !block_header.verify() {
            return Err(StateTransitionError);
        }
        let block_transactions = self.block_transactions(height..height + 1).await;
        let transactions = block_transactions.map_err(|e| ServerError).unwrap();
        let mut x: Vec<_> = transactions
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
        if x.len() == 1 {
            return x.remove(0);
        }
        return Err(StateTransitionError);
    }

    // #[async_recursion(?Send)]
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
        println!("build_blocks_backward {:#?}", block_height_range);
        let previous_block = self
            .build_blocks_backward(blocks, block_height_range.start..block_height_range.end - 1)
            .await;
        if previous_block.as_ref().ok().is_some() {
            iter.next();
            let block_header = self
                .block_headers(block_height_range.end - 1..block_height_range.end)
                .await;
            let header = block_header.map_err(|e| ServerError).unwrap(); // check the return
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
                },
                Err(_) => return Err(ServerError),
            }
        }
        return Err(ServerError);
    }.boxed()
    }

    // #[async_recursion(?Send)]
    fn build_blocks_forward(
        &self,
        mut blocks: Vec<Block>,
        block_height_range: Range<u32>,
    ) -> BoxFuture<'_, Result<Vec<Block>, ServerError>> {
      async move{  
        if block_height_range.start == block_height_range.end {
            return Ok(blocks);
        }
        let mut iter = self.iter();
        let block_header = self
            .block_headers(block_height_range.end - 1..block_height_range.end)
            .await;
        println!("build_blocks_backward {:#?}", block_height_range);
        let header = block_header.map_err(|e| ServerError).unwrap(); // check the return
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
            return self.build_blocks_forward(blocks, block_height_range.start..block_height_range.end - 1).await;
        }
        return Err(ServerError)
    }.boxed()
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