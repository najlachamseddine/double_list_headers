use async_trait::async_trait;
use core::ops::Range;
use futures::executor::block_on;
use hex::FromHex;
use std::fmt;
use std::sync::{Arc, Mutex};
use tokio::runtime::Handle;
use async_recursion::async_recursion;

// Rc::RefCell can also be used (non thread safe though)
pub type Link<T> = Arc<Mutex<Node<T>>>;

#[derive(Debug)]
pub struct Node<T> {
    item: T,
    previous: Option<Link<T>>,
    next: Option<Link<T>>,
}

impl<T> Node<T> {
    fn new(item: T) -> Self {
        Self {
            item,
            previous: None,
            next: None,
        }
    }
}

#[derive(Debug)]
pub struct DoubleLinkedList<T> {
    head: Option<Link<T>>,
    tail: Option<Link<T>>,
    size: usize,
}

pub struct DoubleLinkedListIter<T> {
    next: Option<Link<T>>,
    next_back: Option<Link<T>>,
}

impl<T> DoubleLinkedList<T> {
    pub fn new() -> Self {
        DoubleLinkedList {
            head: None,
            tail: None,
            size: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn insert_at_head(&mut self, data: T) {
        let new_node = Arc::new(Mutex::new(Node::new(data)));
        match self.head.take() {
            None => {
                self.head = Some(Arc::clone(&new_node));
                self.tail = Some(new_node);
                self.size = 1;
            }
            Some(h) => {
                h.lock().unwrap().previous = Some(Arc::clone(&new_node));
                new_node.lock().unwrap().next = Some(h);
                self.head = Some(new_node);
                self.size += 1;
            }
        }
    }

    pub fn insert_at_tail(&mut self, data: T) {
        let new_node = Arc::new(Mutex::new(Node::new(data)));
        match self.tail.take() {
            None => {
                self.head = Some(Arc::clone(&new_node));
                self.tail = Some(new_node);
                self.size = 1;
            }
            Some(t) => {
                t.lock().unwrap().next = Some(Arc::clone(&new_node));
                new_node.lock().unwrap().previous = Some(t);
                self.tail = Some(new_node);
                self.size += 1;
            }
        }
    }

    pub fn pop_head(&mut self) -> Option<T> {
        self.head.take().map(|h| {
            self.size -= 1;
            match h.lock().unwrap().next.take() {
                None => {
                    self.tail.take();
                }
                Some(h_next) => {
                    h_next.lock().unwrap().previous = None;
                    self.head = Some(h_next);
                }
            }
            Arc::try_unwrap(h).ok().unwrap().into_inner().unwrap().item
        })
    }

    pub fn pop_tail(&mut self) -> Option<T> {
        self.tail.take().map(|t| {
            self.size -= 1;
            match t.lock().unwrap().previous.take() {
                None => {
                    self.head.take();
                }
                Some(h_previous) => {
                    h_previous.lock().unwrap().next = None;
                    self.tail = Some(h_previous);
                }
            }
            Arc::try_unwrap(t).ok().unwrap().into_inner().unwrap().item
        })
    }

    pub fn iter<'a>(&'a self) -> DoubleLinkedListIter<T> {
        DoubleLinkedListIter {
            next: self.head.clone(),
            next_back: self.tail.clone(),
        }
    }
}

impl<T> Drop for DoubleLinkedList<T> {
    fn drop(&mut self) {
        while let Some(node) = self.head.take() {
            let _ = node.lock().unwrap().previous.take();
            self.head = node.lock().unwrap().next.take();
        }
        self.tail.take();
    }
}

impl<T> Iterator for DoubleLinkedList<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.pop_head()
    }
}

impl<T> DoubleEndedIterator for DoubleLinkedList<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.pop_tail()
    }
}

impl<T> Iterator for DoubleLinkedListIter<T>
where
    T: Clone + Default,
{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            let guard = node.lock().unwrap();
            self.next = guard.next.clone();
            guard.item.clone()
        })
    }
}

impl<T> DoubleEndedIterator for DoubleLinkedListIter<T>
where
    T: Clone + Default,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.next_back.take().map(|node| {
            let guard = node.lock().unwrap();
            self.next_back = guard.previous.clone();
            guard.item.clone()
        })
    }
}

////////////////////////////
///
///
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

type BlockList = DoubleLinkedList<Block>;

impl BlockList {
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

    #[async_recursion(?Send)]
    async fn build_blocks_backward(
        &self,
        blocks: Vec<Block>,
        block_height_range: Range<u32>,
    ) -> Result<Vec<Block>, ServerError> {
        let mut iter = self.iter();
        if iter.next_back().is_none() {
            return Ok(blocks);
        }
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
    }

    // #[async_recursion(?Send)]
    // async fn build_blocks_forward(
    //     &self,
    //     blocks: Vec<Option<Block>>,
    //     block_height_range: Range<u32>,
    // ) -> Result<Vec<Option<Block>>, ServerError> {
    //     let mut iter = self.iter();
    //     let block_header = self
    //         .block_headers(block_height_range.end - 1..block_height_range.end)
    //         .await;
    //     let header = block_header.map_err(|e| ServerError).unwrap(); // check the return
    //     let res2: Vec<_> = header
    //         .clone()
    //         .into_iter()
    //         .map(|bh| {
    //             return self.build_block_transactions(bh, block_height_range.end - 1);
    //         })
    //         .collect();
    //     let mut res = futures::future::join_all(res2).await;
    //     let d = res.remove(0);
    //     if d.ok().is_some() {
    //         if iter.next().is_none() {
    //             return Ok(blocks);
    //         }
    //         return self.build_blocks_forward(blocks.push(), block_height_range.start..block_height_range.end - 1).await;
    //     }
    //     return Err(ServerError)
    // }
}

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

#[tokio::main]
async fn main() {
    // https://rtoch.com/posts/rust-doubly-linked-list/
    // add test with cfg_test and asserts
    // test drop works fine

    println!("Hello, world!");
    let mut list = DoubleLinkedList::<i32>::new();
    for i in 0..10 {
        print!("print the index {:#?}", i);
        // list.insert_at_head(i);
        list.insert_at_tail(i);
    }

    let mut list_block = BlockList::new();
    let block_header0 = BlockHeader {
        block_height: 0,
        consensus_fields: ConsensusFields {},
    };

    let transaction0 = Transaction {
        tx_id: <[u8; 32]>::from_hex(
            "1e6f77206973207468652074696d6520666f7220616c6c20676f6f64206d656e",
        )
        .expect("invalid tx_id"),
        transaction_fields: TransactionFields {},
    };
    let transaction1 = Transaction {
        tx_id: <[u8; 32]>::from_hex(
            "2eef77206973207468652074696d6520666f7220616c6c20676f6f64206d656e",
        )
        .expect("invalid tx_id"),
        transaction_fields: TransactionFields {},
    };
    let transaction2 = Transaction {
        tx_id: <[u8; 32]>::from_hex(
            "3eaf77206973207468652074696d6520666f7220616c6c20676f6f64206d656e",
        )
        .expect("invalid tx_id"),
        transaction_fields: TransactionFields {},
    };
    let transaction3 = Transaction {
        tx_id: <[u8; 32]>::from_hex(
            "4eff77206973207468652074696d6520666f7220616c6c20676f6f64206d656e",
        )
        .expect("invalid tx_id"),
        transaction_fields: TransactionFields {},
    };
    let mut transactions = Vec::new();
    transactions.push(transaction0);
    transactions.push(transaction1);
    transactions.push(transaction2);
    transactions.push(transaction3);

    let block0 = Block {
        header: block_header0,
        transactions: transactions.clone(),
    };

    let block_header1 = BlockHeader {
        block_height: 1,
        consensus_fields: ConsensusFields {},
    };
    let block1 = Block {
        header: block_header1,
        transactions: transactions.clone(),
    };

    let block_header2 = BlockHeader {
        block_height: 2,
        consensus_fields: ConsensusFields {},
    };
    let block2 = Block {
        header: block_header2,
        transactions: transactions.clone(),
    };

    let block_header3 = BlockHeader {
        block_height: 3,
        consensus_fields: ConsensusFields {},
    };
    let block3 = Block {
        header: block_header3,
        transactions: transactions.clone(),
    };

    let block_header4 = BlockHeader {
        block_height: 4,
        consensus_fields: ConsensusFields {},
    };
    let block4 = Block {
        header: block_header4,
        transactions: transactions.clone(),
    };

    let block_header5 = BlockHeader {
        block_height: 5,
        consensus_fields: ConsensusFields {},
    };
    let block5 = Block {
        header: block_header5,
        transactions: transactions.clone(),
    };

    list_block.insert_at_head(block5);
    list_block.insert_at_head(block4);
    list_block.insert_at_head(block3);
    list_block.insert_at_head(block2);
    list_block.insert_at_head(block1);
    list_block.insert_at_head(block0.clone());

    //  for j in list_block.iter() {
    //     println!("{:#?}", j);
    //     // break;
    // }

    // let block_at = list_block.get_block_header_at(2);
    // println!("BLOCK AT {:#?}", block_at);

    //    let verify_block_headers = list_block.verify_block_header_list(0..4);
    //    println!("verify list headers {:#?}", verify_block_headers);

    // let headers = list_block.block_headers(1..6).await;
    // println!("block headers {:#?}", headers);

    // let valid = validate_block_transactions(block0.clone().transactions);
    // println!("validate transactions {:#?}", valid);

    // let new_block = list_block.build_block_transactions(block_header3, 3).await;
    // println!("NEW BLOCK {:#?}", new_block );

    let arclist = Arc::new(list_block);
    let blocks_parallel = arclist.build_blocks_parallel(1..4).await;
    println!("block parallel {:#?}", blocks_parallel);

    // for i in list.iter().rev() {
    //     println!("{}", i);
    //     // break;
    // }
    for j in list.iter() {
        println!("{}", j);
        // break;
    }

    // for k in list.iter().rev() {
    //     println!("{}", k);
    //     // break;
    // }
}
