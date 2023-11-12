# double_list_headers

## double linked list

The double linked list together with its iterators (Iterator and DoubleEndedIterator) are implemented in the module `./list/linked_list`. It is implemented on a generic type `T` using Arc and Mutex to make it thread safe.


## Blocks build

The implementation of the API Server is in `api/server.rs`.

A double linked list on the type `Block` will support the data to be requested by the server.

The API to verify and execute the blocks are in the `./api/blocks` module. There are two types:

* In parallel where each block can be verified independently from each other (`build_blocks_parallel` function)

* Recursively backward where block `X` depends on block `X - 1` verification/execution (`build_blocks_backward` function)

* Recursively forward (terminal) (using the backward is better)

Running `cargo run` (or `cargo run --release`) in the `./api` module (for `./api/src/main.rs`) calls the above functions on an instanciated block list. A binary could also be used.

`cargo bench` could also be used to run these tests and times could be given.


## warp server

A warp server is built in the root of the project with endpoints calling the different block build functions but not finished yet (fixing tokio runtime issue). It could be run with `cargo run` at the root of the repo.





## The task
 
  The task is to fetch all [`Block`]s for the range `0..100_000`,
 but:
 - Before the requesting transactions, we need to call
   [`BlockHeader::verify`]
  and fetch transactions only if it returns `true`.
  - Before combining [`BlockHeader`] and [`Vec<Transaction>`]
   into [`Block`], we need to
 iterate over each transaction and call
  [`Transaction::execute`]. If all results are [`Result::Ok`],
 then we can create a [`Block`].
  
   The goal of the task is to request data as fast as possible(in
  parallel). Blocks can be executed
  and verified independently. It means verification or execution
   of the block
   at height `X` can be done without block at height `X - 1`.

   An additional optional task: The same goal as before, but
   verification/execution of
   the block header/block at height `X` requires verification/
   execution of
   the block header/block at height `X - 1`.

