// SPDX-License-Identifier: CC0-1.0

//! Test the `serde` implementations for types in `primitives`.

#![cfg(feature = "alloc")]
#![cfg(feature = "serde")]

use bitcoin_primitives::script::{ScriptHash, WScriptHash};
use bitcoin_primitives::{BlockHash, Ntxid, TxMerkleNode, Txid, WitnessCommitment, WitnessMerkleNode, Wtxid};

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

/// Ntxid tests: 1) round-trips; 2) format verifications

fn ntx_id() -> Ntxid {
    "e567952fb6cc33857f392efa3a46c995a28f69cca4bb1b37e0204dab1ec7a389".parse::<Ntxid>().unwrap()
}
#[test]
fn ntx_id_serde_human_readable_roundtrips() {
    let tc = ntx_id();
    let ser = serde_json::to_string(&tc).unwrap();
    let got = serde_json::from_str::<Ntxid>(&ser).unwrap();
    assert_eq!(got, tc);
}

#[test]
fn ntx_id_serde_non_human_readable_roundtrips() {
    let tc = ntx_id();
    let ser = bincode::serialize(&tc).unwrap();
    let got = bincode::deserialize::<Ntxid>(&ser).unwrap();
    assert_eq!(got, tc);
}

#[test]
fn ntx_id_serde_deserialize_human_readable() {
    let ser = "\"e567952fb6cc33857f392efa3a46c995a28f69cca4bb1b37e0204dab1ec7a389\"";
    let got = serde_json::from_str::<Ntxid>(ser).unwrap();
    let want = ntx_id();
    assert_eq!(got, want);
}

#[test]
fn ntx_id_serde_deserialize_non_human_readable() {
    let want = ntx_id();
    let expected_bytes = bincode::serialize(&want).unwrap();
    let got = bincode::deserialize::<Ntxid>(&expected_bytes).unwrap();
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

/// ScriptHash tests: 1) round-trips; 2) format verifications
fn script_hash() -> ScriptHash {
    "b472a266d0bd89c13706a4132ccfb16f7c3b9fcb".parse::<ScriptHash>().unwrap()
}

#[test]
fn script_hash_serde_human_readable_roundtrips() {
    let tc = script_hash();
    let ser = serde_json::to_string(&tc).unwrap();
    let got = serde_json::from_str::<ScriptHash>(&ser).unwrap();
    assert_eq!(got, tc);
}

#[test]
fn script_hash_serde_non_human_readable_roundtrips() {
    let tc = script_hash();
    let ser = bincode::serialize(&tc).unwrap();
    let got = bincode::deserialize::<ScriptHash>(&ser).unwrap();
    assert_eq!(got, tc);
}

#[test]
fn script_hash_deserialize_human_readable() {
    let ser = "\"b472a266d0bd89c13706a4132ccfb16f7c3b9fcb\"";
    let got = serde_json::from_str::<ScriptHash>(ser).unwrap();
    let want = script_hash();
    assert_eq!(got, want);
}

#[test]
fn script_hash_deserialize_non_human_readable() {
    let want = script_hash();
    let expected_bytes = bincode::serialize(&want).unwrap();
    let got = bincode::deserialize::<ScriptHash>(&expected_bytes).unwrap();
    assert_eq!(got, want);
}

/// TxMerkleNode tests: 1) round-trips; 2) format verifications
fn tx_merkle_node() -> TxMerkleNode {
    "56944c5d3f98413ef45cf54545538103cc9f298e0575820ad3591376e2e0f65d".parse::<TxMerkleNode>().unwrap()
}

#[test]
fn merkle_node_serde_human_readable_roundtrips() {
    let tc = tx_merkle_node();
    let ser = serde_json::to_string(&tc).unwrap();
    let got = serde_json::from_str::<TxMerkleNode>(&ser).unwrap();
    assert_eq!(got, tc);
}

#[test]
fn merkle_node_serde_non_human_readable_roundtrips() {
    let tc = tx_merkle_node();
    let ser = bincode::serialize(&tc).unwrap();
    let got = bincode::deserialize::<TxMerkleNode>(&ser).unwrap();
    assert_eq!(got, tc);
}

#[test]
fn merkle_node_deserialize_human_readable() {
    let ser = "\"56944c5d3f98413ef45cf54545538103cc9f298e0575820ad3591376e2e0f65d\"";
    let got = serde_json::from_str::<TxMerkleNode>(ser).unwrap();
    let want = tx_merkle_node();
    assert_eq!(got, want);
}

#[test]
fn merkle_node_deserialize_non_human_readable() {
    let want = tx_merkle_node();
    let expected_bytes = bincode::serialize(&want).unwrap();
    let got = bincode::deserialize::<TxMerkleNode>(&expected_bytes).unwrap();
    assert_eq!(got, want);
}

/// WitnessCommitment tests: 1) round-trips; 2) format verifications
fn witness_commitment() -> WitnessCommitment {
    "56944c5d3f98413ef45cf54545538103cc9f298e0575820ad3591376e2e0f65d".parse::<WitnessCommitment>().unwrap()
}

#[test]
fn witness_commitment_serde_human_readable_roundtrips() {
    let tc = witness_commitment();
    let ser = serde_json::to_string(&tc).unwrap();
    let got = serde_json::from_str::<WitnessCommitment>(&ser).unwrap();
    assert_eq!(got, tc);
}

#[test]
fn witness_commitment_serde_non_human_readable_roundtrips() {
    let tc = witness_commitment();
    let ser = bincode::serialize(&tc).unwrap();
    let got = bincode::deserialize::<WitnessCommitment>(&ser).unwrap();
    assert_eq!(got, tc);
}

#[test]
fn witness_commitment_deserialize_human_readable() {
    let ser = "\"56944c5d3f98413ef45cf54545538103cc9f298e0575820ad3591376e2e0f65d\"";
    let got = serde_json::from_str::<WitnessCommitment>(ser).unwrap();
    let want = witness_commitment();
    assert_eq!(got, want);
}

#[test]
fn witness_commitment_deserialize_non_human_readable() {
    let want = witness_commitment();
    let expected_bytes = bincode::serialize(&want).unwrap();
    let got = bincode::deserialize::<WitnessCommitment>(&expected_bytes).unwrap();
    assert_eq!(got, want);
}

/// WitnessMerkleNode tests: 1) round-trips; 2) format verifications
fn witness_merkle_node() -> WitnessMerkleNode {
    "56944c5d3f98413ef45cf54545538103cc9f298e0575820ad3591376e2e0f65d".parse::<WitnessMerkleNode>().unwrap()
}

#[test]
fn witness_merkle_node_serde_human_readable_roundtrips() {
    let tc = witness_merkle_node();
    let ser = serde_json::to_string(&tc).unwrap();
    let got = serde_json::from_str::<WitnessMerkleNode>(&ser).unwrap();
    assert_eq!(got, tc);
}

#[test]
fn witness_merkle_node_serde_non_human_readable_roundtrips() {
    let tc = witness_merkle_node();
    let ser = bincode::serialize(&tc).unwrap();
    let got = bincode::deserialize::<WitnessMerkleNode>(&ser).unwrap();
    assert_eq!(got, tc);
}

#[test]
fn witness_merkle_node_deserialize_human_readable() {
    let ser = "\"56944c5d3f98413ef45cf54545538103cc9f298e0575820ad3591376e2e0f65d\"";
    let got = serde_json::from_str::<WitnessMerkleNode>(ser).unwrap();
    let want = witness_merkle_node();
    assert_eq!(got, want);
}

#[test]
fn witness_merkle_node_deserialize_non_human_readable() {
    let want = witness_merkle_node();
    let expected_bytes = bincode::serialize(&want).unwrap();
    let got = bincode::deserialize::<WitnessMerkleNode>(&expected_bytes).unwrap();
    assert_eq!(got, want);
}

/// WScriptHash tests: 1) round-trips; 2) format verifications
fn wscript_hash() -> WScriptHash {
    "56944c5d3f98413ef45cf54545538103cc9f298e0575820ad3591376e2e0f65d".parse::<WScriptHash>().unwrap()
}

#[test]
fn wscript_hash_node_serde_human_readable_roundtrips() {
    let tc = wscript_hash();
    let ser = serde_json::to_string(&tc).unwrap();
    let got = serde_json::from_str::<WScriptHash>(&ser).unwrap();
    assert_eq!(got, tc);
}

#[test]
fn wscript_hash_node_serde_non_human_readable_roundtrips() {
    let tc = wscript_hash();
    let ser = bincode::serialize(&tc).unwrap();
    let got = bincode::deserialize::<WScriptHash>(&ser).unwrap();
    assert_eq!(got, tc);
}

#[test]
fn wscript_hash_node_deserialize_human_readable() {
    let ser = "\"56944c5d3f98413ef45cf54545538103cc9f298e0575820ad3591376e2e0f65d\"";
    let got = serde_json::from_str::<WScriptHash>(ser).unwrap();
    let want = wscript_hash();
    assert_eq!(got, want);
}

#[test]
fn wscript_hash_node_deserialize_non_human_readable() {
    let want = wscript_hash();
    let expected_bytes = bincode::serialize(&want).unwrap();
    let got = bincode::deserialize::<WScriptHash>(&expected_bytes).unwrap();
    assert_eq!(got, want);
}

/// Wtxid tests: 1) round-trips; 2) format verifications
fn wtx_id() -> Wtxid {
    "56944c5d3f98413ef45cf54545538103cc9f298e0575820ad3591376e2e0f65d".parse::<Wtxid>().unwrap()
}

#[test]
fn wtx_id_serde_human_readable_roundtrips() {
    let tc = wtx_id();
    let ser = serde_json::to_string(&tc).unwrap();
    let got = serde_json::from_str::<Wtxid>(&ser).unwrap();
    assert_eq!(got, tc);
}

#[test]
fn wtx_id_serde_non_human_readable_roundtrips() {
    let tc = wtx_id();
    let ser = bincode::serialize(&tc).unwrap();
    let got = bincode::deserialize::<Wtxid>(&ser).unwrap();
    assert_eq!(got, tc);
}

#[test]
fn wtx_id_deserialize_human_readable() {
    let ser = "\"56944c5d3f98413ef45cf54545538103cc9f298e0575820ad3591376e2e0f65d\"";
    let got = serde_json::from_str::<Wtxid>(ser).unwrap();
    let want = wtx_id();
    assert_eq!(got, want);
}

#[test]
fn wtx_id_deserialize_non_human_readable() {
    let want = wtx_id();
    let expected_bytes = bincode::serialize(&want).unwrap();
    let got = bincode::deserialize::<Wtxid>(&expected_bytes).unwrap();
    assert_eq!(got, want);
}