use async_trait::async_trait;
use core::ops::Range;
use std::sync::{Arc, Mutex};

// Rc::RefCell can also be used (non thread safe though)
pub type Link<T> = Arc<Mutex<Node<T>>>;

#[derive(Debug)]
pub struct Node<T> {
    item: T,
    previous: Option<Link<T>>,
    next: Option<Link<T>>,
}

pub fn process_node<T>(node: Link<T>) -> T 
where T: Copy
{
    let guard = node.lock().unwrap();
    let value = guard.item;
    value
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
    // next_back: Option<&'a Mutex<Node<T>>>,
    // next: MutexGuard<'a, Node<T>>,
    // next_back: MutexGuard<'a, Node<T>>,
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

    // pub fn iter<'a>(&'a self) -> DoubleLinkedListIter<'a, T> {
    //     DoubleLinkedListIter {
    //         next: self.head.as_deref(),
    //         next_back: self.tail.as_deref(),
    //     }
    // }

    pub fn iter<'a>(&'a self) -> DoubleLinkedListIter<T> {
        DoubleLinkedListIter {
            next: self.head.clone(),
            // next_back: self.tail.as_deref(),
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

impl<'a, T> Iterator for DoubleLinkedListIter<T> 
where T: Copy
{
    type Item = T;
    // type Item = Iter<'_, &'a Mutex<Node<T>>>;
    // fn next(&mut self) -> Option<Self::Item> {
    //     self.next.map(|node| {
    //         // let n = node.clone();
    //         // let n2 = &node.lock().unwrap().item;
    //         // let n3 = node.lock().unwrap().next.as_deref().iter();
    //         let guard = node.lock().unwrap();
    //         self.next = guard.next.as_deref();
    //         &guard.item
    //         // n.lock().unwrap().next.as_deref().clone().unwrap()
    //         // node.lock().unwrap().next.as_deref().unwrap()
    //     })
    // }
    fn next(&mut self) -> Option<Self::Item> {
        //   let guard = self.next.next.as_deref().unwrap().lock().unwrap();
          self.next.take().map(|node| {
            let binding = node.lock().unwrap().next.clone().unwrap();
            // self.next = Some(Arc::clone(&binding));
            self.next = Some(binding.clone());
            // process_node(Arc::clone(&binding))
            process_node(binding.clone())    
          })
   }
  
}

// for iter
// impl<'a, T> Iterator for DoubleLinkedList<'a, T> {
//     type Item = &'a T;
//     fn next(&'a mut self) -> Option<Self::Item> {
//         self.head.map(|node| {
//             self.head = node.lock().unwrap().next.map(|node| & node);
//             & node.item
//         })
//     }
// }

// impl<'a, T> DoubleEndedIterator for DoubleLinkedList<'a, T> {
//     fn next_back(&mut self) -> Option<Self::Item> {
//         self.pop_tail()
//     }
// }
////////////////////////////
///
///
pub struct ConsensusFields;

pub struct TransactionFields;

pub type TransactionId = [u8; 32];

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

#[async_trait]
pub trait ServerAPI {
    async fn block_headers(
        &self,
        block_height_range: Range<u32>,
    ) -> Result<Vec<BlockHeader>, ServerError>;

    async fn block_transactions(
        &self,
        block_height_range: Range<u32>,
    ) -> Result<Vec<Vec<Transaction>>, ServerError>;
}

// type BlockList = DoubleLinkedList<Block>;

// impl ServerAPI for BlockList {

//     async fn block_headers(
//         &self,
//         block_height_range: Range<u32>,
//         ) -> Result<Vec<BlockHeader>, ServerError> {
//            let headers = block_height_range
//            .map(BlockHeader::from)
//            .map(|blockHeader| self. )

//            Ok(headers)
//         }

fn main() {
    // https://rtoch.com/posts/rust-doubly-linked-list/
    // add test with cfg_test and asserts
    // test drop works fine

    println!("Hello, world!");
    let mut list = DoubleLinkedList::<i32>::new();
    for i in 0..=10 {
        list.insert_at_head(i);
        // list.insert_at_tail(i + 2);
    }
    // println!("{:#?}", list.pop_head());
    // println!("{:#?}", list.pop_tail());
    // println!("{:#?}", list.pop_head());
    // println!("{:#?}", list.pop_tail());
    // println!("{:#?}", list.pop_tail());
    // println!("{:#?}", list.pop_tail());
    // println!("{:#?}", list.pop_tail());
    // println!("{:#?}", list.pop_tail());
    // println!("{:#?}", list.pop_tail());
    print!("iter ");
    for i in list.iter() {
        println!("{}", i);
        // break;
    }
    // print!("into iter");
    // println!("{:#?}", list.next());
    // println!("{:#?}", list.next());
    // println!("{:#?}", list.next_back());
    // // println!("{:#?}", list.rev());
    // for i in list.into_iter() {
    //     println!("iter test: {:#?}", i);
    // }
}