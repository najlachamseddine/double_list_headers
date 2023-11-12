
use list::linked_list::*;
use api::server::*;
use api::blocks::*;
use std::sync::Arc;

pub async fn get_blocks_in_parallel(list_blocks: DoubleLinkedList<Block>, end_range:u32) -> Result<impl warp::Reply, warp::Rejection> {

    let arclist = Arc::new(list_blocks);
    let blocks_parallel = tokio::join!(arclist.build_blocks_parallel(0..end_range));
    Ok(warp::reply::json(&blocks_parallel))
   } 

   pub async fn get_blocks_in_backward(list_blocks: DoubleLinkedList<Block>, end_range:u32) -> Result<impl warp::Reply, warp::Rejection> {

    let blocks: Vec<Block> = vec![];
    let arclist = Arc::new(list_blocks);
    let blocks_parallel = arclist.build_blocks_backward(blocks.clone(), 0..end_range).await;
    Ok(warp::reply::json(&blocks_parallel))
   }

//    pub async fn get_blocks_in_forward(list_blocks: DoubleLinkedList<Block>, end_range:u32) -> Result<impl warp::Reply, warp::Rejection> {

//     let blocks: Vec<Block> = vec![];
//     let arclist = Arc::new(list_blocks);
//     let blocks_parallel = arclist.build_blocks_parallel(0..end_range).await;
//     println!("block parallel {:#?}", blocks_parallel);

//     Ok(warp::reply::json(&blocks))
//    }