use api::server::*;
use hex::FromHex;

use requests::handlers;
use requests::routes;

mod requests;

#[tokio::main]
async fn main() {
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
    let routes = routes::routes(list_block);

    println!("Server started at http://localhost:8000");
    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}
