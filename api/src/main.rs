use hex::FromHex;
use std::sync::Arc;
use api::server::{TransactionFields, Block, BlockHeader, ConsensusFields, Transaction, BlockList};
use api::blocks::Blocks;
use list::linked_list::*;

#[tokio::main]
async fn main() {

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

    // let arclist = Arc::new(list_block);
    // let blocks_parallel = arclist.build_blocks_parallel(0..4).await;
    // println!("block parallel {:#?}", blocks_parallel);

    // println!("BUILD BLOCKS BACKWARD");
    // let arclist = Arc::new(list_block);
    // let blcks: Vec<Block> = vec![];
    // let blocks_backward = arclist.build_blocks_backward(blcks, 0..5).await;
    // println!("block parallel backward {:#?}", blocks_backward);

    println!("BUILD BLOCKS FORWARD");
    let arclist = Arc::new(list_block);
    let blcks: Vec<Block> = vec![];
    let blocks_forward = arclist.build_blocks_forward(blcks, 0..5).await;
    println!("block parallel forward {:#?}", blocks_forward);


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
