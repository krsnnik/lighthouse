/*
 * Minimal Beacon Node API for Validator
 *
 * A minimal API specification for the beacon node, which enables a validator to connect and perform its obligations on the Ethereum 2.0 phase 0 beacon chain.
 *
 * The version of the OpenAPI document: 0.2.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use std::rc::Rc;
use std::borrow::Borrow;

use hyper;
use serde_json;
use futures::Future;

use super::{Error, configuration};
use super::request as __internal_request;

pub struct MinimalSetApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> MinimalSetApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> MinimalSetApiClient<C> {
        MinimalSetApiClient {
            configuration: configuration,
        }
    }
}

pub trait MinimalSetApi {
    fn node_genesis_time_get(&self, ) -> Box<Future<Item = i32, Error = Error<serde_json::Value>>>;
    fn node_syncing_get(&self, ) -> Box<Future<Item = ::models::InlineResponse200, Error = Error<serde_json::Value>>>;
    fn node_version_get(&self, ) -> Box<Future<Item = String, Error = Error<serde_json::Value>>>;
    fn validator_attestation_get(&self, validator_pubkey: String, poc_bit: i32, slot: i32, shard: i32) -> Box<Future<Item = ::models::IndexedAttestation, Error = Error<serde_json::Value>>>;
    fn validator_attestation_post(&self, attestation: ::models::::models::IndexedAttestation) -> Box<Future<Item = (), Error = Error<serde_json::Value>>>;
    fn validator_block_get(&self, slot: i32, randao_reveal: String) -> Box<Future<Item = ::models::BeaconBlock, Error = Error<serde_json::Value>>>;
    fn validator_block_post(&self, beacon_block: ::models::::models::BeaconBlock) -> Box<Future<Item = (), Error = Error<serde_json::Value>>>;
    fn validator_duties_get(&self, validator_pubkeys: Vec<String>, epoch: i32) -> Box<Future<Item = Vec<::models::ValidatorDuty>, Error = Error<serde_json::Value>>>;
}


impl<C: hyper::client::Connect>MinimalSetApi for MinimalSetApiClient<C> {
    fn node_genesis_time_get(&self, ) -> Box<Future<Item = i32, Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Get, "/node/genesis_time".to_string())
            .execute(self.configuration.borrow())
    }

    fn node_syncing_get(&self, ) -> Box<Future<Item = ::models::InlineResponse200, Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Get, "/node/syncing".to_string())
            .execute(self.configuration.borrow())
    }

    fn node_version_get(&self, ) -> Box<Future<Item = String, Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Get, "/node/version".to_string())
            .execute(self.configuration.borrow())
    }

    fn validator_attestation_get(&self, validator_pubkey: String, poc_bit: i32, slot: i32, shard: i32) -> Box<Future<Item = ::models::IndexedAttestation, Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Get, "/validator/attestation".to_string())
            .with_query_param("validator_pubkey".to_string(), validator_pubkey.to_string())
            .with_query_param("poc_bit".to_string(), poc_bit.to_string())
            .with_query_param("slot".to_string(), slot.to_string())
            .with_query_param("shard".to_string(), shard.to_string())
            .execute(self.configuration.borrow())
    }

    fn validator_attestation_post(&self, attestation: ::models::::models::IndexedAttestation) -> Box<Future<Item = (), Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Post, "/validator/attestation".to_string())
            .with_query_param("attestation".to_string(), attestation.to_string())
            .returns_nothing()
            .execute(self.configuration.borrow())
    }

    fn validator_block_get(&self, slot: i32, randao_reveal: String) -> Box<Future<Item = ::models::BeaconBlock, Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Get, "/validator/block".to_string())
            .with_query_param("slot".to_string(), slot.to_string())
            .with_query_param("randao_reveal".to_string(), randao_reveal.to_string())
            .execute(self.configuration.borrow())
    }

    fn validator_block_post(&self, beacon_block: ::models::::models::BeaconBlock) -> Box<Future<Item = (), Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Post, "/validator/block".to_string())
            .with_query_param("beacon_block".to_string(), beacon_block.to_string())
            .returns_nothing()
            .execute(self.configuration.borrow())
    }

    fn validator_duties_get(&self, validator_pubkeys: Vec<String>, epoch: i32) -> Box<Future<Item = Vec<::models::ValidatorDuty>, Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Get, "/validator/duties".to_string())
            .with_query_param("validator_pubkeys".to_string(), validator_pubkeys.join(",").to_string())
            .with_query_param("epoch".to_string(), epoch.to_string())
            .execute(self.configuration.borrow())
    }

}
