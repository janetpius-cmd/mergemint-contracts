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
