use warp::Filter;
use crate::handlers;
use list::linked_list::*;
use api::server::*;

pub fn routes(list_blocks: DoubleLinkedList<Block>) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    get_blocks_in_parallel(list_blocks.clone())
    .or(get_blocks_in_backward(list_blocks.clone()))
    // get_blocks_in_forward();
}

pub fn get_blocks_in_parallel( list_blocks: DoubleLinkedList<Block>) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("blocks" / u32)
    .and(warp::get())
    .and_then(move |end: u32| {
        let blocks = list_blocks.clone();
        return async move {
            handlers::get_blocks_in_parallel(blocks.clone(), end)
                .await
                .map_err(|e|e)
        }
    })
}

pub fn get_blocks_in_backward( list_blocks: DoubleLinkedList<Block>) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("blocks" / u32)
    .and(warp::get())
    .and_then(move |end: u32| {
        let blocks = list_blocks.clone();
        return async move {
            handlers::get_blocks_in_backward(blocks.clone(), end)
                .await
                // .map(|result| warp::reply::json(&result))
                .map_err(|e|e)
                // .map_err(|e| warp::reject::custom(e))
        }
    })
}