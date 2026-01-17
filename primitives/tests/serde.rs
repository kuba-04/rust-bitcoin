// SPDX-License-Identifier: CC0-1.0

//! Test the `serde` implementations for types in `primitives`.

#![cfg(feature = "alloc")]
#![cfg(feature = "serde")]

use bitcoin_primitives::{BlockHash, Txid};
use internals::serde::{Deserialize, Serialize};

/// Txid tests: 1) round-trips; 2) format verifications
fn tx_id() -> Txid {
    "e567952fb6cc33857f392efa3a46c995a28f69cca4bb1b37e0204dab1ec7a389".parse::<Txid>().unwrap()
}

#[test]
fn tx_id_serde_human_readable_roundtrips() {
    let tc = tx_id();
    let ser = serde_json::to_string(&tc).unwrap();
    let got = serde_json::from_str::<Txid>(&ser).unwrap();
    assert_eq!(got, tc);
}

#[test]
fn tx_id_serde_non_human_readable_roundtrips() {
    let tc = tx_id();
    let ser = bincode::serialize(&tc).unwrap();
    let got = bincode::deserialize::<Txid>(&ser).unwrap();
    assert_eq!(got, tc);
}

#[test]
fn tx_id_serde_deserialize_human_readable() {
    let ser = "\"e567952fb6cc33857f392efa3a46c995a28f69cca4bb1b37e0204dab1ec7a389\"";
    let got = serde_json::from_str::<Txid>(ser).unwrap();
    let want = tx_id();
    assert_eq!(got, want);
}

#[test]
fn tx_id_serde_deserialize_non_human_readable() {
    let want = tx_id();
    let expected_bytes = bincode::serialize(&want).unwrap();
    let got = bincode::deserialize::<Txid>(&expected_bytes).unwrap();
    assert_eq!(got, want);
}

/// BlockHash tests: 1) round-trips; 2) format verifications
fn block_hash() -> BlockHash {
    "000000000019d6689c085ae165831e934ff763ae46a2a6c172b3f1b60a8ce26f".parse::<BlockHash>().unwrap()
}

#[test]
fn block_hash_serde_human_readable_roundtrips() {
    let tc = block_hash();
    let ser = serde_json::to_string(&tc).unwrap();
    let got = serde_json::from_str::<BlockHash>(&ser).unwrap();
    assert_eq!(got, tc);
}

#[test]
fn block_hash_serde_non_human_readable_roundtrips() {
    let tc = block_hash();
    let ser = bincode::serialize(&tc).unwrap();
    let got = bincode::deserialize::<BlockHash>(&ser).unwrap();
    assert_eq!(got, tc);
}

#[test]
fn block_hash_deserialize_human_readable() {
    let ser = "\"000000000019d6689c085ae165831e934ff763ae46a2a6c172b3f1b60a8ce26f\"";
    let got = serde_json::from_str::<BlockHash>(ser).unwrap();
    let want = block_hash();
    assert_eq!(got, want);
}

#[test]
fn block_hash_deserialize_non_human_readable() {
    let want = block_hash();
    let expected_bytes = bincode::serialize(&want).unwrap();
    let got = bincode::deserialize::<BlockHash>(&expected_bytes).unwrap();
    assert_eq!(got, want);
}