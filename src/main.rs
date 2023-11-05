use async_trait::async_trait;
use core::ops::Range;
use std::sync::{Arc, Mutex};
use std::fmt;

// Rc::RefCell can also be used (non thread safe though)
pub type Link<T> = Arc<Mutex<Node<T>>>;

#[derive(Debug)]
pub struct Node<T> {
    item: T,
    previous: Option<Link<T>>,
    next: Option<Link<T>>,
}

pub fn process_node<T>(node: Option<Link<T>>) -> Option<T>
where
    T: Copy,
{
    match node {
        Some(n) => {
            let guard = n.lock().unwrap();
            let value = guard.item;
            Some(value)
        }
        None => None,
    }
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

// impl<T> Iterator for DoubleLinkedList<T> {
//     type Item = T;
//     fn next(&mut self) -> Option<Self::Item> {
//         self.pop_head()
//     }
// }

// impl<T> DoubleEndedIterator for DoubleLinkedList<T> {
//     fn next_back(&mut self) -> Option<Self::Item> {
//         self.pop_tail()
//     }
// }

impl<T> Iterator for DoubleLinkedListIter<T>
where
    T: Copy + Default,
{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            let guard = node.lock().unwrap();
            // use if let Some(guard.next.clone() = n and remove the match None)
            match guard.next.clone() {
                Some(n) => {
                    self.next = Some(n.clone());
                    process_node(Some(n.clone())).unwrap()
                }
                None => {
                    self.next = None;
                    T::default()
                }
            }
        })
    }
}

impl<T> DoubleEndedIterator for DoubleLinkedListIter<T>
where
    T: Copy + Default,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.next_back.take().map(|node| {
            let guard = node.lock().unwrap();
            match guard.previous.clone() {
                Some(n) => {
                    self.next_back = Some(n.clone());
                    process_node(Some(n.clone())).unwrap()
                }
                None => {
                    self.next_back = None;
                    T::default()
                }
            }
        })
    }
}

////////////////////////////
///
///
#[derive(Debug,Default, Clone, Copy)]
pub struct ConsensusFields;

#[derive(Debug,Default, Clone, Copy)]
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


pub struct StateTransitionError;

#[derive(Debug,Default, Clone, Copy)]
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

pub struct Block {
    pub header: BlockHeader,
    pub transactions: Vec<Transaction>,
}

pub struct ServerError;

impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Server error")
    }
}

#[async_trait]
pub trait ServerAPI {
    async fn block_headers(
        &mut self,
        block_height_range: Range<u32>,
    ) -> Result<Vec<BlockHeader>, ServerError>;

    async fn block_transactions(
        &self,
        block_height_range: Range<u32>,
    ) -> Result<Vec<Vec<Transaction>>, ServerError>;
}




fn main() {
    // https://rtoch.com/posts/rust-doubly-linked-list/
    // add test with cfg_test and asserts
    // test drop works fine

    println!("Hello, world!");
    let mut list = DoubleLinkedList::<i32>::new();
    for i in 0..=10 {
        list.insert_at_head(i);
        list.insert_at_tail(i + 2);
    }

    // let mut list_block = BlockHeaderList::new();
    // let block_header0 = BlockHeader { block_height: 0, consensus_fields: ConsensusFields{}};
    // // let block1 = Block { header: block_header0, transactions: Vec::new() } ;

    // let block_header1 = BlockHeader { block_height: 1, consensus_fields: ConsensusFields{}};
    // // let block2 = Block { header: block_header1, transactions: Vec::new() } ;

    // let block_header2 = BlockHeader { block_height: 2, consensus_fields: ConsensusFields{}};
    // // let block3 = Block { header: block_header2, transactions: Vec::new() } ;

    // let block_header3 = BlockHeader { block_height: 3, consensus_fields: ConsensusFields{}};
    // // let block3 = Block { header: block_header3, transactions: Vec::new() } ;

    // let block_header4 = BlockHeader { block_height: 4, consensus_fields: ConsensusFields{}};
    // // let block4 = Block { header: block_header4, transactions: Vec::new() } ;

    // let block_header5 = BlockHeader { block_height: 5, consensus_fields: ConsensusFields{}};
    // // let block5 = Block { header: block_header5, transactions: Vec::new() } ;

    // list_block.insert_at_head(block_header0);
    // list_block.insert_at_head(block_header1);
    // list_block.insert_at_head(block_header2);
    // list_block.insert_at_head(block_header3);
    // list_block.insert_at_head(block_header4);
    // list_block.insert_at_head(block_header5);
    

//      for j in list_block.iter() {
//         println!("{:#?}", j);
//         // break;
//     }

//    let block_at = list_block.get_block_header_at(3);
// //    println!("BLOCK AT {:#?}", block_at.lock().unwrap().item);
//    println!("BLOCK AT {:#?}", block_at.lock().unwrap().item);

//    let verify_block_headers = list_block.verify_block_header_list(0..2);
//    println!("verify list headers {:#?}", verify_block_headers);

    // println!("{:#?}", list.pop_head());
    // println!("{:#?}", list.pop_tail());
    // println!("{:#?}", list.pop_head());
    // println!("{:#?}", list.pop_tail());
    // println!("{:#?}", list.pop_tail());
    // println!("{:#?}", list.pop_tail());
    // println!("{:#?}", list.pop_tail());
    // println!("{:#?}", list.pop_tail());
    // println!("{:#?}", list.pop_tail());
    // print!("iter ");
    // for i in list.iter() {
    //     println!("{}", i);
    //     // break;
    // }
    // for j in list.iter() {
    //     println!("{}", j);
    //     // break;
    // }

    // for k in list.iter().rev() {
    //     println!("{}", k);
    //     // break;
    // }
    // print!("into iter");
    // println!("{:#?}", list.next());
    // println!("{:#?}", list.next());
    // println!("{:#?}", list.next_back());
    // // println!("{:#?}", list.rev());
    // for i in list.into_iter() {
    //     println!("iter test: {:#?}", i);
    // }
}
