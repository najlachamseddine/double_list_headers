use api::blocks::*;
use api::server::*;
use list::linked_list::*;
use std::sync::Arc;
use tokio::runtime::Handle;

pub async fn get_blocks_in_parallel(
    list_blocks: DoubleLinkedList<Block>,
    end_range: u32,
) -> Result<impl warp::Reply, warp::Rejection> {
    let arclist = Arc::new(list_blocks);
    let handle = Handle::current();
    // let blocks_parallel = arclist.build_blocks_parallel(0..end_range).await;
    let handle = std::thread::spawn(move || {
     return handle.block_on(arclist.build_blocks_parallel(0..end_range)).unwrap();
    });
    let result = handle.join().unwrap();
    Ok(warp::reply::json(&result))
}

pub async fn get_blocks_in_backward(
    list_blocks: DoubleLinkedList<Block>,
    end_range: u32,
) -> Result<impl warp::Reply, warp::Rejection> {
    let blocks: Vec<Block> = vec![];
    let arclist = Arc::new(list_blocks);
    let blocks_parallel = arclist
        .build_blocks_backward(blocks.clone(), 0..end_range)
        .await;
    Ok(warp::reply::json(&blocks_parallel))
}

//    pub async fn get_blocks_in_forward(list_blocks: DoubleLinkedList<Block>, end_range:u32) -> Result<impl warp::Reply, warp::Rejection> {

//     let blocks: Vec<Block> = vec![];
//     let arclist = Arc::new(list_blocks);
//     let blocks_parallel = arclist.build_blocks_parallel(0..end_range).await;
//     println!("block parallel {:#?}", blocks_parallel);

//     Ok(warp::reply::json(&blocks))
//    }
