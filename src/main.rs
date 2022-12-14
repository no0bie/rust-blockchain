use serde::Serialize;
use sha2::{Sha256, Digest};

use chrono::prelude;

const DIFFICULTY: &str = "0x00";

#[derive(Serialize, Debug)]
struct Block{
    height: u64,
    timestamp: i64,
    hash: String,
    previous_hash: String,
    data: String,
    nonce: u128,
}

struct Chain{
    blocks: Vec<Block>,
}

impl Block {
    fn hex_digest(&self) -> String{
        let group_data = format!("{}{}{}{}{}", self.height, self.timestamp, self.previous_hash, self.data, self.nonce);
        let sha_digest = Sha256::digest(group_data);
        hex(hex::encode(sha_digest))
    }
}

impl Chain {
    fn start() -> Self{
        let genesis: Block = Block { 
            height: 0, 
            timestamp: 1670978178, 
            hash: String::from("0x00ce2356a31b00df2d948334440ca89d9a9c4ea456be2e8925e763073e68ec16"), 
            previous_hash: String::from("0x0074686520626567696e6e696e67206f66206120727573747920636861696e21"), 
            data: String::from("A very rustychain"),
            nonce: 146,
        };
    
        Self {
            blocks: vec![genesis]
        }
    }

    fn block_valid(&self, block: &Block) -> bool{
        let previous_block = self.blocks.last().expect("No blocks in chain"); 

        println!("{}", block.hash);
        println!("{}", block.hex_digest());

        if previous_block.hash.cmp(&block.previous_hash).is_ne(){
            println!("Block does not have a correct previous hash");
            return false;
        }
        else if !block.hash.starts_with(&DIFFICULTY){
            println!("Block does not have the correct difficulty");
            return false
        }

        else if block.hash.cmp(&block.hex_digest()).is_ne(){
            println!("Block does not have the correct hash");
            return false
        }

        else if block.height.cmp(&(previous_block.height + 1)).is_ne(){
            println!("Block height is not what is supposed to be");
            return false;
        }
        true
    }

    fn add_block(&mut self, block: Block) -> String{
        if self.block_valid(&block) {
            self.blocks.push(block);
            return String::from("Block added");
        }

        String::from("Block is not valid")
    }
}

fn hex(hex: String) -> String{
    format!("0x{hex}")
}

fn main() {
    let mut chain: Chain = Chain::start();
    
    println!("{:?}", chain.blocks);

}
