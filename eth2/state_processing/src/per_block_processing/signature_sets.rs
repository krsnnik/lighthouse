use crate::per_block_processing::errors::AttestationValidationError;
use bls::SignatureSet;
use core::borrow::Borrow;
use tree_hash::{SignedRoot, TreeHash};
use types::{
    AggregatePublicKey, AttestationDataAndCustodyBit, AttesterSlashing, BeaconBlock,
    BeaconBlockHeader, BeaconState, BeaconStateError, ChainSpec, Domain, EthSpec,
    IndexedAttestation, ProposerSlashing, PublicKey, RelativeEpoch, Transfer, VoluntaryExit,
};

const SIGNATURES_PER_PROPOSER_SLASHING: usize = 2;
const SIGNATURES_PER_INDEXED_ATTESTATION: usize = 2;
const INDEXED_ATTESTATIONS_PER_ATTESTER_SLASHING: usize = 2;

/// The pair of aggregate public keys generated by processing an indexed attestation.
///
/// TODO: what if one of these is empty??
pub type IndexedAttestationPublicKeys = [AggregatePublicKey; SIGNATURES_PER_INDEXED_ATTESTATION];

/// The aggregate public keys generated by processing an `AttesterSlashing`. Contains four keys,
/// two for each attestation.
///
/// TODO: what if one of these is empty??
pub type AttesterSlashingPublicKeys =
    [IndexedAttestationPublicKeys; INDEXED_ATTESTATIONS_PER_ATTESTER_SLASHING];

pub type Result<T> = std::result::Result<T, Error>;

pub enum Error {
    /// Signature verification failed. The block is invalid.
    SignatureInvalid,
    /// There was an error attempting to read from a `BeaconState`. Block
    /// validity was not determined.
    BeaconStateError(BeaconStateError),
    /// An attestation in the block was invalid. The block is invalid.
    AttestationValidationError(AttestationValidationError),
    /// Attempted to find the public key of a validator that does not exist. You cannot distinguish
    /// between an error and an invalid block in this case.
    ValidatorUnknown(u64),
    /// The public keys supplied do not match the number of objects requiring keys. Block validity
    /// was not determined.
    MismatchedPublicKeyLen { pubkey_len: usize, other_len: usize },
}

impl From<BeaconStateError> for Error {
    fn from(e: BeaconStateError) -> Error {
        Error::BeaconStateError(e)
    }
}

impl From<AttestationValidationError> for Error {
    fn from(e: AttestationValidationError) -> Error {
        Error::AttestationValidationError(e)
    }
}

// TODO: unify with block_header_signature_set?
/// A signature set that is valid if a block was signed by the expected block producer.
pub fn block_proposal_signature_set<'a, T: EthSpec>(
    state: &'a BeaconState<T>,
    block: &'a BeaconBlock<T>,
    spec: &'a ChainSpec,
) -> Result<SignatureSet<'a>> {
    let block_proposer = &state.validators
        [state.get_beacon_proposer_index(block.slot, RelativeEpoch::Current, spec)?];

    let domain = spec.get_domain(
        block.slot.epoch(T::slots_per_epoch()),
        Domain::BeaconProposer,
        &state.fork,
    );

    let message = block.signed_root();

    Ok(SignatureSet::new(
        &block.signature,
        vec![&block_proposer.pubkey],
        vec![message],
        domain,
    ))
}

/// A signature set that is valid if the block proposers randao reveal signature is correct.
pub fn randao_signature_set<'a, T: EthSpec>(
    state: &'a BeaconState<T>,
    block: &'a BeaconBlock<T>,
    spec: &'a ChainSpec,
) -> Result<SignatureSet<'a>> {
    let block_proposer = &state.validators
        [state.get_beacon_proposer_index(block.slot, RelativeEpoch::Current, spec)?];

    let domain = spec.get_domain(
        block.slot.epoch(T::slots_per_epoch()),
        Domain::Randao,
        &state.fork,
    );

    let message = state.current_epoch().tree_hash_root();

    Ok(SignatureSet::new(
        &block.body.randao_reveal,
        vec![&block_proposer.pubkey],
        vec![message],
        domain,
    ))
}

/// Returns two signature sets, one for each `BlockHeader` included in the `ProposerSlashing`.
pub fn proposer_slashing_signature_set<'a, T: EthSpec>(
    state: &'a BeaconState<T>,
    proposer_slashing: &'a ProposerSlashing,
    spec: &'a ChainSpec,
) -> Result<[SignatureSet<'a>; SIGNATURES_PER_PROPOSER_SLASHING]> {
    let proposer = state
        .validators
        .get(proposer_slashing.proposer_index as usize)
        .ok_or_else(|| Error::ValidatorUnknown(proposer_slashing.proposer_index))?;

    Ok([
        block_header_signature_set(state, &proposer_slashing.header_1, &proposer.pubkey, spec)?,
        block_header_signature_set(state, &proposer_slashing.header_2, &proposer.pubkey, spec)?,
    ])
}

/// Returns a signature set that is valid if the given `pubkey` signed the `header`.
fn block_header_signature_set<'a, T: EthSpec>(
    state: &'a BeaconState<T>,
    header: &'a BeaconBlockHeader,
    pubkey: &'a PublicKey,
    spec: &'a ChainSpec,
) -> Result<SignatureSet<'a>> {
    let domain = spec.get_domain(
        header.slot.epoch(T::slots_per_epoch()),
        Domain::BeaconProposer,
        &state.fork,
    );

    let message = header.signed_root();

    Ok(SignatureSet::new(
        &header.signature,
        vec![pubkey],
        vec![message],
        domain,
    ))
}

/// Returns the two `AggregatePublicKeys` that are generated when processing an
/// `IndexedAttestation`. One for each custody bit.
pub fn indexed_attestation_pubkeys<'a, T: EthSpec>(
    state: &'a BeaconState<T>,
    indexed_attestation: &'a IndexedAttestation<T>,
) -> Result<IndexedAttestationPublicKeys> {
    Ok([
        create_aggregate_pubkey(state, &indexed_attestation.custody_bit_0_indices)?,
        create_aggregate_pubkey(state, &indexed_attestation.custody_bit_1_indices)?,
    ])
}

/// Returns the signature set for the given `indexed_attestation` and corresponding `pubkeys`.
///
/// `IndexedAttestationPublicKeys` can be generated using `indexed_attestation_pubkeys(..)`.
pub fn indexed_attestation_signature_set<'a, T: EthSpec>(
    state: &'a BeaconState<T>,
    indexed_attestation: &'a IndexedAttestation<T>,
    pubkeys: &'a IndexedAttestationPublicKeys,
    spec: &'a ChainSpec,
) -> Result<SignatureSet<'a>> {
    let message_0 = AttestationDataAndCustodyBit {
        data: indexed_attestation.data.clone(),
        custody_bit: false,
    }
    .tree_hash_root();
    let message_1 = AttestationDataAndCustodyBit {
        data: indexed_attestation.data.clone(),
        custody_bit: true,
    }
    .tree_hash_root();

    let domain = spec.get_domain(
        indexed_attestation.data.target.epoch,
        Domain::Attestation,
        &state.fork,
    );

    Ok(SignatureSet::new(
        &indexed_attestation.signature,
        pubkeys.iter().map(Borrow::borrow).collect(),
        vec![message_0, message_1],
        domain,
    ))
}

/// Returns the signature set for the given `attester_slashing` and corresponding `pubkeys`.
///
/// FIXME: pubkey generation function is not here (it's in the verifier).
pub fn attester_slashing_signature_set<'a, T: EthSpec>(
    state: &'a BeaconState<T>,
    attester_slashing: &'a AttesterSlashing<T>,
    pubkeys: &'a AttesterSlashingPublicKeys,
    spec: &'a ChainSpec,
) -> Result<[SignatureSet<'a>; INDEXED_ATTESTATIONS_PER_ATTESTER_SLASHING]> {
    Ok([
        indexed_attestation_signature_set(
            state,
            &attester_slashing.attestation_1,
            &pubkeys[0],
            spec,
        )?,
        indexed_attestation_signature_set(
            state,
            &attester_slashing.attestation_2,
            &pubkeys[1],
            spec,
        )?,
    ])
}

/* Not currently used
 *
 *
pub fn deposit_pubkeys_signatures_messages(
    deposits: &[Deposit],
) -> Vec<(PublicKey, Signature, Message)> {
    deposits
        .iter()
        .filter_map(|deposit| {
            let pubkey = (&deposit.data.pubkey).try_into().ok()?;
            let signature = (&deposit.data.signature).try_into().ok()?;
            let message = deposit.data.signed_root();
            Some((pubkey, signature, message))
        })
        .collect()
}

pub fn deposit_signature_set<'a, T: EthSpec>(
    state: &'a BeaconState<T>,
    pubkey_signature_message: &'a (PublicKey, Signature, Message),
    spec: &'a ChainSpec,
) -> SignatureSet<'a> {
    // Note: Deposits are valid across forks, thus the deposit domain is computed
    // with the fork zeroed.
    let domain = spec.get_domain(state.current_epoch(), Domain::Deposit, &Fork::default());
    let (pubkey, signature, message) = pubkey_signature_message;

    SignatureSet::new(signature, vec![pubkey], vec![message.clone()], domain)
}
*/

/// Returns a signature set that is valid if the `VoluntaryExit` was signed by the indicated
/// validator.
pub fn exit_signature_set<'a, T: EthSpec>(
    state: &'a BeaconState<T>,
    exit: &'a VoluntaryExit,
    spec: &'a ChainSpec,
) -> Result<SignatureSet<'a>> {
    let validator = state
        .validators
        .get(exit.validator_index as usize)
        .ok_or_else(|| Error::ValidatorUnknown(exit.validator_index))?;

    let domain = spec.get_domain(exit.epoch, Domain::VoluntaryExit, &state.fork);

    let message = exit.signed_root();

    Ok(SignatureSet::new(
        &exit.signature,
        vec![&validator.pubkey],
        vec![message],
        domain,
    ))
}

pub fn transfer_signature_set<'a, T: EthSpec>(
    state: &'a BeaconState<T>,
    transfer: &'a Transfer,
    spec: &'a ChainSpec,
) -> Result<SignatureSet<'a>> {
    let domain = spec.get_domain(
        transfer.slot.epoch(T::slots_per_epoch()),
        Domain::Transfer,
        &state.fork,
    );

    let message = transfer.signed_root();

    Ok(SignatureSet::new(
        &transfer.signature,
        vec![&transfer.pubkey],
        vec![message],
        domain,
    ))
}

/// Create an aggregate public key for a list of validators, failing if any key can't be found.
fn create_aggregate_pubkey<'a, T, I>(
    state: &BeaconState<T>,
    validator_indices: I,
) -> Result<AggregatePublicKey>
where
    I: IntoIterator<Item = &'a u64>,
    T: EthSpec,
{
    let mut aggregate_pubkey = validator_indices.into_iter().try_fold(
        AggregatePublicKey::new(),
        |mut aggregate_pubkey, &validator_idx| {
            state
                .validators
                .get(validator_idx as usize)
                .ok_or_else(|| Error::ValidatorUnknown(validator_idx))
                .map(|validator| {
                    aggregate_pubkey.add_without_affine(&validator.pubkey);
                    aggregate_pubkey
                })
        },
    )?;

    aggregate_pubkey.affine();

    Ok(aggregate_pubkey)
}
