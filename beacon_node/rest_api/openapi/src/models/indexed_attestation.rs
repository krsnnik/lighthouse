/*
 * Minimal Beacon Node API for Validator
 *
 * A minimal API specification for the beacon node, which enables a validator to connect and perform its obligations on the Ethereum 2.0 phase 0 beacon chain.
 *
 * The version of the OpenAPI document: 0.2.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IndexedAttestation : The [`IndexedAttestation`](https://github.com/ethereum/eth2.0-specs/blob/master/specs/core/0_beacon-chain.md#indexedattestation) object from the Eth2.0 spec.

#[allow(unused_imports)]
use serde_json::Value;


#[derive(Debug, Serialize, Deserialize)]
pub struct IndexedAttestation {
    /// Validator indices for 0 bits.
    #[serde(rename = "custody_bit_0_indices", skip_serializing_if = "Option::is_none")]
    pub custody_bit_0_indices: Option<Vec<i32>>,
    /// Validator indices for 1 bits.
    #[serde(rename = "custody_bit_1_indices", skip_serializing_if = "Option::is_none")]
    pub custody_bit_1_indices: Option<Vec<i32>>,
    /// The BLS signature of the `IndexedAttestation`, created by the validator of the attestation.
    #[serde(rename = "signature", skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<::models::AttestationData>,
}

impl IndexedAttestation {
    /// The [`IndexedAttestation`](https://github.com/ethereum/eth2.0-specs/blob/master/specs/core/0_beacon-chain.md#indexedattestation) object from the Eth2.0 spec.
    pub fn new() -> IndexedAttestation {
        IndexedAttestation {
            custody_bit_0_indices: None,
            custody_bit_1_indices: None,
            signature: None,
            data: None,
        }
    }
}


