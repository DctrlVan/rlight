extern crate secp256k1;

use secp256k1::key::{PublicKey, SecretKey};

#[derive(Debug)]
pub enum AnchorOffer {
    WILL_CREATE_ANCHOR,
    WONT_CREATE_ANCHOR,
}

#[derive(Debug)]
pub enum Locktime {
    Seconds(u32),
    Blocks(u32)
}

#[derive(Debug)]
pub struct OpenChannel {
    pub revocation_hash: [u8; 32],
    pub next_revocation_hash: [u8; 32],
    pub commit_key: PublicKey,
    pub final_key: PublicKey,
    pub anch: AnchorOffer,
    pub min_depth: u32,
    pub delay: Locktime,
    pub initial_fee_rate: u32
}
