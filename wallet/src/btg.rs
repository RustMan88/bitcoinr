use bitcoin::{
    util::{
        address::{Address,Payload},
        bip143,
        key::{PrivateKey,PublicKey}
    },
    blockdata::transaction::{OutPoint, Transaction, TxIn, TxOut,SigHashType},
    blockdata::script::{Script, Builder},
    network::constants::Network,
    consensus::{self},
};
use bitcoin_hashes::{self, sha256d, Hash};
use bitcoin_hashes::hex::FromHex;

use secp256k1::{Secp256k1, Message};
use hex;
use rand::thread_rng;
use std::{
    sync::{Arc, RwLock},
    collections::HashMap,
    str::FromStr,
};
use super::{Error,TxInputReq};
use crate::TxOutputReq;

#[derive(Debug)]
pub struct Account {
    pub public_key: PublicKey,
    pub private_key: PrivateKey,
    pub address: Address,
}

/*pub fn generate_p2pkh(num :u32) -> Result<Vec<Account>,Error> {
    let s = Secp256k1::new();
    let mut acts = vec![];

    for i in 0..num {
        let key = s.generate_keypair(&mut thread_rng());
        acts.push(Account{
            private: key.0.to_string(),
            public:key.1.to_string(),
            address:Address::p2pkh(&key.1,Network::BitcoinGold)
        })
    }
    let public_key = PublicKey {
    compressed: true,
     key: s.generate_keypair(&mut thread_rng()).1,
    };

    let address = Address::p2pkh(&public_key, Network::Bitcoin);


    let secp = Secp256k1::new();
    let pk = Address::p2pkh(&sk.public_key(&secp), sk.network);
}
*/
fn get_outpoint(input : &TxInputReq) -> Result<OutPoint,Error>{
    Ok(OutPoint{
        txid: sha256d::Hash::from_hex(&input.txid).map_err(|_|Error::TxidParseError)?,
        vout: input.index
    })
}

pub fn create_rawtx(vins:&Vec<TxInputReq>,vouts:&Vec<TxOutputReq>)->Result<Transaction,Error> {

    //检查入数量是否大于等于出数量
    let total_out = vouts.iter().fold(0, |acc, output| acc + output.value);
    let total_in = vins.iter().fold(0, |acc, input| acc + input.credit);

    if total_in < total_out {
        return Err(Error::NotEnoughAmount);
    }

    let mut tx = Transaction {
        version:   1,
        lock_time: 0,
        input:     Vec::new(),
        output:    Vec::new(),
    };

    for i in 0..vins.len(){
        let vin = &vins[i];
        let op = get_outpoint(vin)?;
        let input = TxIn{
            previous_output: op,
            script_sig:      Script::new(),
            sequence:        0xFFFFFFFF,
            witness:         Vec::new(),
        };
        tx.input.push(input);
    }

    for j in 0..vouts.len(){
        let vout = &vouts[j];
        let addr: Address = Address::from_str(&vout.address).map_err(|_|Error::AddressParseError)?;
        // dest output
        let output = TxOut{
            value: vout.value,
            script_pubkey: addr.script_pubkey(),
        };
        tx.output.push(output);
    }

    println!("create_rawtx-> tx:{:?}",tx);
    return Ok(tx)
}

pub fn sign_rawtx( tmp :&Transaction,accounts:Vec<Account>) -> Result<String, Error> {

    let mut tx = tmp.clone();

    let ctx = Secp256k1::new();

    for i in 0..tx.input.len() {
      //  let mut vin = &tx.input[i];
        let account   = &accounts[i];

        match account.address.payload {
            Payload::PubkeyHash(_) =>{
                let pk_script = account.address.script_pubkey();
                println!("sign_rawtx-> pk_script:{:?}",pk_script);
                let sign_data = tx.signature_hash(i, &pk_script, SigHashType::Forkid.as_u32()).into_inner();
                let mut serialized_sig = account.private_key.sign(&Message::from_slice(&sign_data).map_err(|_|Error::AddressParseError)?,&ctx);
//                let signature = ctx.sign(
//                    &Message::from_slice(&sign_data).map_err(|_|Error::AddressParseError)?,
//                    &account.private_key.key,
//                );
//                let mut serialized_sig = signature.serialize_der();
                serialized_sig.push(0x1);

                let script = Builder::new()
                    .push_slice(serialized_sig.as_slice())
                    .push_slice(&account.public_key.key.serialize())
                    .into_script();

                tx.input[i].script_sig = script;
            },
            _ => return Err(Error::NotSupportedAddressFormError)
        }
    }
    println!("sign_rawtx-> tx:{:?}",tx);
   Ok(consensus::encode::serialize_hex(&tx))
}

#[cfg(test)]
mod tests {
    #[test]
    fn sign_tx_test() {
        assert_eq!(2 + 2, 4);
    }
}

