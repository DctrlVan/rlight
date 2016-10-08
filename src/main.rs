extern crate bitcoin;
extern crate secp256k1;
extern crate rand;

mod channel;

use rand::Rng;
use secp256k1::key::{PublicKey, SecretKey};
use secp256k1::Secp256k1;

use bitcoin::network::constants::Network;
use bitcoin::blockdata::script::*;
use bitcoin::blockdata::opcodes::All;
use bitcoin::util::address::{Privkey};
use bitcoin::util::address::Address;
use bitcoin::util::hash::Hash160;
use bitcoin::util::base58::ToBase58;
use bitcoin::blockdata::transaction::*;

use bitcoin::util::hash::Sha256dHash;

use channel::*;

struct Node {
    address : Address,
    secret: SecretKey,
    private: Privkey,
}

fn createAddress()-> Address{
    let scp = Secp256k1::new();
    let mut rng = rand::thread_rng();
    let secret = SecretKey::new( &scp, &mut rng );
    let private = Privkey::from_key(Network::Testnet, secret, false );
    let address = private
        .to_address(&scp)
        .unwrap();
    address
}

impl Node {
    fn generate()-> Self {
        let scp = Secp256k1::new();
        let mut rng = rand::thread_rng();
        let secret = SecretKey::new( &scp, &mut rng );
        let private = Privkey::from_key(Network::Testnet, secret, false );
        let address = private
            .to_address(&scp)
            .unwrap();

        Node {
            address:address,
            secret: secret,
            private: private
        }
    }

    fn create_public_key(&self)-> PublicKey{
        let scp = Secp256k1::new();
        PublicKey::from_secret_key(&scp, &self.secret).unwrap()
    }

    fn open_channel(&self, min_depth :u32)-> OpenChannel {
        let mut rng = rand::thread_rng();
        OpenChannel {
            revocation_hash:rng.gen(),
            next_revocation_hash: rng.gen(),
            commit_key: self.create_public_key(),
            final_key: self.create_public_key(),
            min_depth: min_depth,
            delay: Locktime::Blocks(0),
            anch: AnchorOffer::WILL_CREATE_ANCHOR,
            initial_fee_rate: 0,
        }
    }

}

fn create_anchor_tx(key1: Address,key2: Address, prev_txin: TxIn, amount: u64) -> Transaction {

    let (hash1, hash2) = (key1.base58_layout(), key2.base58_layout());

    let script_out = Builder::new()
        .push_opcode(All::OP_PUSHNUM_2)
        .push_slice(&hash1)
        .push_slice(&hash2)
        .push_opcode(All::OP_PUSHNUM_2)
        .push_opcode(All::OP_CHECKMULTISIG)
        .into_script();

    let txout = TxOut {
        value: amount,
        script_pubkey: script_out
    };

    Transaction {
        version: 1,
        lock_time: 0,
        input: vec![prev_txin],
        output:vec![txout],
        witness:vec![vec![vec![0]]]
    }



}


fn main() {
    let node = Node::generate();
    println!("{:?}", node.open_channel(0) );

    let prev_hash = Sha256dHash::from_hex("812e898507ad95c9b89890e39c4b8a158ee8293ed462397de3a574739cb71937");

    let tx_in = TxIn {
        prev_hash: prev_hash.unwrap(),
        prev_index: 0,
        script_sig: Script::new(),
        sequence: 0xFFFFFFFF,
    };

    let addr1:Address = createAddress();
    let addr2:Address = createAddress();
    println!("{:?},{:?}", addr1, addr2 );

    println!("{:?}" , create_anchor_tx(addr1, addr2,tx_in,1000));

}
