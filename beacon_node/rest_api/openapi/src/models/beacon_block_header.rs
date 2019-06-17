/*
 * Minimal Beacon Node API for Validator
 *
 * A minimal API specification for the beacon node, which enables a validator to connect and perform its obligations on the Ethereum 2.0 phase 0 beacon chain.
 *
 * The version of the OpenAPI document: 0.2.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// BeaconBlockHeader : The [`BeaconBlockHeader`](https://github.com/ethereum/eth2.0-specs/blob/master/specs/core/0_beacon-chain.md#beaconblockheader) object from the Eth2.0 spec.

#[allow(unused_imports)]
use serde_json::Value;


#[derive(Debug, Serialize, Deserialize)]
pub struct BeaconBlockHeader {
    /// The slot to which this block corresponds.
    #[serde(rename = "slot", skip_serializing_if = "Option::is_none")]
    pub slot: Option<i32>,
    /// The signing merkle root of the parent `BeaconBlock`.
    #[serde(rename = "parent_root", skip_serializing_if = "Option::is_none")]
    pub parent_root: Option<String>,
    /// The tree hash merkle root of the `BeaconState` for the `BeaconBlock`.
    #[serde(rename = "state_root", skip_serializing_if = "Option::is_none")]
    pub state_root: Option<String>,
    /// The BLS signature of the `BeaconBlock` made by the validator of the block.
    #[serde(rename = "signature", skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
    /// The tree hash merkle root of the `BeaconBlockBody` for the `BeaconBlock`
    #[serde(rename = "body_root", skip_serializing_if = "Option::is_none")]
    pub body_root: Option<String>,
}

impl BeaconBlockHeader {
    /// The [`BeaconBlockHeader`](https://github.com/ethereum/eth2.0-specs/blob/master/specs/core/0_beacon-chain.md#beaconblockheader) object from the Eth2.0 spec.
    pub fn new() -> BeaconBlockHeader {
        BeaconBlockHeader {
            slot: None,
            parent_root: None,
            state_root: None,
            signature: None,
            body_root: None,
        }
    }
}


