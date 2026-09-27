use soroban_sdk::{
    contract, contractimpl, token::TokenClient, Address, BytesN, Env, String, Symbol, Vec,
};

use crate::errors;
use crate::errors::{fail, ContractError};
use crate::events;
use crate::storage;
use crate::types::{Bounty, BountyId, BountyMeta, Contributor, Milestone};

/// Maximum protocol fee, expressed in basis points (10 percent).
pub const MAX_FEE_BPS: u32 = 1000;

#[contract]
pub struct MergeMintContract;

include!("mutations.rs");
include!("queries.rs");

#[contractimpl]
impl MergeMintContract {
    /// Add a verifier to an Open bounty. Creator only.
    ///
    /// The new verifier must not be the bounty assignee. After the change the
    /// approval threshold must still be less than or equal to the verifier
    /// count.
    pub fn add_verifier(env: Env, bounty_id: BountyId, verifier: Address) -> Result<(), ContractError> {
        let mut bounty: Bounty = storage::get_bounty(&env, bounty_id)?;

        bounty.creator.require_auth();

        if bounty.status != BountyStatus::Open {
            fail(&env, ContractError::BountyNotOpen);
        }

        if let Some(assignee) = bounty.assignee.clone() {
            if assignee == verifier {
                fail(&env, ContractError::VerifierCannotBeAssignee);
            }
        }

        if bounty.verifiers.contains(&verifier) {
            fail(&env, ContractError::VerifierAlreadyExists);
        }

        bounty.verifiers.push_back(verifier.clone());

        if bounty.approval_threshold > bounty.verifiers.len() {
            fail(&env, ContractError::ApprovalThresholdExceedsVerifiers);
        }

        storage::set_bounty(&env, bounty_id, &bounty);

        events::verifier_added(&env, bounty_id, &verifier);

        Ok(())
    }

    /// Remove a verifier from an Open bounty. Creator only.
    ///
    /// After the change the approval threshold must still be less than or
    /// equal to the verifier count.
    pub fn remove_verifier(env: Env, bounty_id: BountyId, verifier: Address) -> Result<(), ContractError> {
        let mut bounty: Bounty = storage::get_bounty(&env, bounty_id)?;

        bounty.creator.require_auth();

        if bounty.status != BountyStatus::Open {
            fail(&env, ContractError::BountyNotOpen);
        }

        let mut index: Option<u32> = None;
        for i in 0..bounty.verifiers.len() {
            if bounty.verifiers.get(i).unwrap() == verifier {
                index = Some(i);
                break;
            }
        }

        let index = match index {
            Some(i) => i,
            None => fail(&env, ContractError::VerifierNotFound),
        };

        bounty.verifiers.remove(index);

        if bounty.approval_threshold > bounty.verifiers.len() {
            fail(&env, ContractError::ApprovalThresholdExceedsVerifiers);
        }

        storage::set_bounty(&env, bounty_id, &bounty);

        events::verifier_removed(&env, bounty_id, &verifier);

        Ok(())
    }
}
