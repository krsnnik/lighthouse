/*
 * Minimal Beacon Node API for Validator
 *
 * A minimal API specification for the beacon node, which enables a validator to connect and perform its obligations on the Ethereum 2.0 phase 0 beacon chain.
 *
 * The version of the OpenAPI document: 0.2.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// AttestationData : The [`AttestationData`](https://github.com/ethereum/eth2.0-specs/blob/master/specs/core/0_beacon-chain.md#attestationdata) object from the Eth2.0 spec.

#[allow(unused_imports)]
use serde_json::Value;


#[derive(Debug, Serialize, Deserialize)]
pub struct AttestationData {
    /// LMD GHOST vote.
    #[serde(rename = "beacon_block_root", skip_serializing_if = "Option::is_none")]
    pub beacon_block_root: Option<String>,
    /// Source epoch from FFG vote.
    #[serde(rename = "source_epoch", skip_serializing_if = "Option::is_none")]
    pub source_epoch: Option<i32>,
    /// Source root from FFG vote.
    #[serde(rename = "source_root", skip_serializing_if = "Option::is_none")]
    pub source_root: Option<String>,
    /// Target epoch from FFG vote.
    #[serde(rename = "target_epoch", skip_serializing_if = "Option::is_none")]
    pub target_epoch: Option<i32>,
    /// Target root from FFG vote.
    #[serde(rename = "target_root", skip_serializing_if = "Option::is_none")]
    pub target_root: Option<String>,
    #[serde(rename = "crosslink", skip_serializing_if = "Option::is_none")]
    pub crosslink: Option<::models::CrossLink>,
}

impl AttestationData {
    /// The [`AttestationData`](https://github.com/ethereum/eth2.0-specs/blob/master/specs/core/0_beacon-chain.md#attestationdata) object from the Eth2.0 spec.
    pub fn new() -> AttestationData {
        AttestationData {
            beacon_block_root: None,
            source_epoch: None,
            source_root: None,
            target_epoch: None,
            target_root: None,
            crosslink: None,
        }
    }
}


