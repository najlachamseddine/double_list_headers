use api::blocks::Blocks;
use api::server::{Block, BlockHeader, BlockList, ConsensusFields, Transaction, TransactionFields};
use hex::FromHex;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    
    let mut list_block = BlockList::new();

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

    for i in 0..100000 {
        let block_header = BlockHeader {
            block_height: i,
            consensus_fields: ConsensusFields {},
        };
        let block = Block {
            header: block_header,
            transactions: transactions.clone(),
        };
        list_block.insert_at_tail(block);
    }

    //
    // Generates blocks in parallel calling build_blocks_parallel
    //
    println!("---- Builds blocks in parallel");
    let arclist = Arc::new(list_block);
    let blocks_parallel = arclist.clone().build_blocks_parallel(0..1000).await;
    // println!("block parallel {:#?}", blocks_parallel);
    assert_eq!(
        blocks_parallel.expect("blocks list in parallel").len(),
        1000
    );
    println!("---- End build blocks in parallel");

    //
    // Generates blocks recursively backward  (for X depends on X -1 )
    // (Might need to increase the local stack size)
    // 
    println!("---- Builds blocks backward");
    // let arclist = Arc::new(list_block);
    let blcks: Vec<Block> = vec![];
    let blocks_backward = arclist.clone().build_blocks_backward(blcks, 0..1000).await;
    // println!("block parallel backward {:#?}", blocks_backward);
    assert_eq!(blocks_backward.expect("blocks list backward").len(), 1000);
    println!("---- End build blocks backward");

    //
    // Generates blocks recursively forward
    // (Might need to increase thr local stack size)
    //
    println!("---- Builds blocks forward");
    // let arclist = Arc::new(list_block);
    let blcks: Vec<Block> = vec![];
    let blocks_forward = arclist.clone().build_blocks_forward(blcks, 0..5000).await;
    // println!("block parallel forward {:#?}", blocks_forward);
    assert_eq!(blocks_forward.expect("blocks list forward").len(), 5000);
    println!("---- End build blocks forward");



    // let headers = list_block.block_headers(1..6).await;
    // println!("block headers {:#?}", headers);

    // let valid = validate_block_transactions(block0.clone().transactions);
    // println!("validate transactions {:#?}", valid);

    // let new_block = list_block.build_block_transactions(block_header3, 3).await;
    // println!("NEW BLOCK {:#?}", new_block );
}
