use std::collections::HashMap;
use libp2p::{core::PublicKey, PeerId};

use crate::message::*;
use libp2p::*;
pub struct kvdb{
    db:HashMap<u64,store_data>
}
impl kvdb {
    pub fn new()->Self{
        Self{
            db:HashMap::new()
        }
    }
}
pub struct store_data{
    id:PeerId,
    //pk_and_sk:(PublicKey,PrivateKey),
    parse_secret:u64,

}
pub struct PrivateKey{

}