
extern crate crypto;
extern crate secp256k1;
extern crate bitcoin;
extern crate rand;
extern crate hex;
extern crate bitcoin_bech32;
extern crate bitcoin_hashes;
extern crate byteorder;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate base64;


pub mod btg;

pub use self::btg::Account;
pub use bitcoin::{
    util::{
        address::{Address, Payload},
        key::{PrivateKey, PublicKey}
    },
    blockdata::transaction::{OutPoint, Transaction, TxIn, TxOut},
    network::constants::Network,
};

#[derive(Debug)]
pub enum Error {
    GreateRawTxError,
    NotFoundKeyError,
    SignRawTxError,
    NotSupportedAddressFormError,
    TxidParseError,
    AddressParseError,
    PrivKeyParseError,
    NotEnoughAmount,
    PrepareRawTxError,
    NotFoundAesKeyError,
    AesDecryptError,
    SerdeJsonError,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TxInputReq {
    pub txid: String,
    pub index: u32,
    pub address: String,
    pub credit: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TxOutputReq {
    pub address: String,
    pub value: u64,
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
