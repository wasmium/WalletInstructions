// © Wasmium Network Developers

// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file,
// You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::{PublicKey, TokenLimit};
use borsh::{BorshDeserialize, BorshSerialize};

#[derive(Debug, BorshSerialize, BorshDeserialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum WalletInstruction {
    RaiseLimit(TokenLimit),
    IncreaseCustodianLimit(u8),
    DecreaseCustodianLimit(u8),
    IncreaseArbitratorLimit(u8),
    DecreaseArbitratorLimit(u8),
    AddCustodian(crate::Custodian),
    RemoveCustodian([u8; 32]),
    RequestTransfer { public_key: PublicKey, token: u64 },
    Transfer { public_key: PublicKey, token: u64 },
    ApproveTransfer { public_key: PublicKey, token: u64 },
}
