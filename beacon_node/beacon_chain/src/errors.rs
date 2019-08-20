use crate::fork_choice::Error as ForkChoiceError;
use state_processing::per_block_processing::errors::AttestationValidationError;
use state_processing::BlockProcessingError;
use state_processing::SlotProcessingError;
use types::*;

macro_rules! easy_from_to {
    ($from: ident, $to: ident) => {
        impl From<$from> for $to {
            fn from(e: $from) -> $to {
                $to::$from(e)
            }
        }
    };
}

#[derive(Debug, PartialEq)]
pub enum BeaconChainError {
    InsufficientValidators,
    BadRecentBlockRoots,
    UnableToReadSlot,
    RevertedFinalizedEpoch {
        previous_epoch: Epoch,
        new_epoch: Epoch,
    },
    UnableToFindTargetRoot(Slot),
    BeaconStateError(BeaconStateError),
    DBInconsistent(String),
    DBError(store::Error),
    ForkChoiceError(ForkChoiceError),
    MissingBeaconBlock(Hash256),
    MissingBeaconState(Hash256),
    SlotProcessingError(SlotProcessingError),
    NoStateForAttestation {
        beacon_block_root: Hash256,
    },
    AttestationValidationError(AttestationValidationError),
}

easy_from_to!(SlotProcessingError, BeaconChainError);

#[derive(Debug, PartialEq)]
pub enum BlockProductionError {
    UnableToGetBlockRootFromState,
    UnableToReadSlot,
    SlotProcessingError(SlotProcessingError),
    BlockProcessingError(BlockProcessingError),
    BeaconStateError(BeaconStateError),
}

easy_from_to!(BlockProcessingError, BlockProductionError);
easy_from_to!(BeaconStateError, BlockProductionError);
easy_from_to!(SlotProcessingError, BlockProductionError);
easy_from_to!(AttestationValidationError, BeaconChainError);
