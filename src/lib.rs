extern crate protobuf;
extern crate secp256k1;
//
// use secp256k1::key::{PublicKey, SecretKey};
//
// struct OpenChannel {
//     revocation_hash: [u8; 32],
//     next_revocation_hash: [u8; 32],
//     commit_key: PublicKey,
//     final_key: PublicKey,
//     anch: AnchorOffer,
//     min_depth: u32,
//     initial_fee_rate: u64,
// }
//
// struct OpenAnchor {
//     txid: [u8; 32],
//     output_index: u32,
//     amount: u64,
// }
//
// enum AnchorOffer {
//     WILL_CREATE_ANCHOR,
//     WONT_CREATE_ANCHOR,
// }
