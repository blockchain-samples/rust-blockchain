//! Blocks routines.

use std::net::{
    TcpStream,
    SocketAddr,
};
use std::time::Duration;
use std::io::Write;
use std::sync::{
    Arc,
    Mutex,
};

use bincode::serialize;

use block::Block;

use message::{
    Message,
    MessageLabel,
};

/// Displays the blockchain blocks.
///
/// Args:
///
/// `chain` - the chain to modify
pub fn list_blocks(chain: &Arc<Mutex<Vec<Block>>>) {

    let chain = chain.lock().unwrap();

    for block in chain.iter() {

        let content = block.get_content();
        println!("Hash: {}", block.get_current());
        println!("Timestamp: {}", content.get_timestamp());
        println!("Data: {} \n\n", content.get_data());
    }
}

/// Tries to send the given block to all the given peers. Skip peer if timeout.
///
/// Args:
///
/// `peers` - list of peers
/// `block` - the block object to send
pub fn broadcast_block(peers: &Vec<SocketAddr>, block: Block) {

    /* we voluntary halt the program if serialization and stream buffer write fails;
       in fact, if these problem happen, that means something is clearly wrong */

    let message = Message::new(
        vec![block],
        MessageLabel::SendBlock,
    );

    let bytes = serialize(&message).unwrap();

    for peer in peers.iter() {

        let address: String = peer.to_string();
        let address_part: Vec<&str> = address.split(':').collect();
        let address = address_part.get(0).unwrap();

        println!("Connecting to {}...", address);

        let mut stream = match TcpStream::connect_timeout(
            &peer,
            Duration::from_secs(5),
        ) {
            Ok(stream) => stream,
            Err(_) => {
                println!("Cannot connect to node {}.", address);
                continue;
            }
        };

        stream.write(&bytes).unwrap();

        println!("Block sent to {}.", address);
    }

    println!("Block creation broadcast terminated.");
}
