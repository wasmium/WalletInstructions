// © Wasmium Network Developers

// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file,
// You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::{WalletError, WalletResult};
use borsh::{BorshDeserialize, BorshSerialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, BorshSerialize, BorshDeserialize)]
pub enum TokenLimit {
    Third,
    Quarter,
    Half,
    TwoThirds,
    All,
}

impl TokenLimit {
    pub fn pack(&self) -> u8 {
        match self {
            TokenLimit::Third => 0,
            TokenLimit::Quarter => 1,
            TokenLimit::Half => 2,
            TokenLimit::TwoThirds => 3,
            TokenLimit::All => 4,
        }
    }

    pub fn unpack(value: u8) -> WalletResult<TokenLimit> {
        match value {
            0 => Ok(TokenLimit::Third),
            1 => Ok(TokenLimit::Quarter),
            2 => Ok(TokenLimit::Half),
            3 => Ok(TokenLimit::TwoThirds),
            4 => Ok(TokenLimit::All),
            _ => Err(WalletError::TokenLimitNotApplicable),
        }
    }

    pub fn calculate_token(&self, tokens_available: u64) -> u64 {
        let tokens_available = tokens_available as f32;
        let allotment = match self {
            Self::Third => (tokens_available * (1.0 / 3.0)).ceil(),
            Self::Quarter => (tokens_available * (1.0 / 4.0)).ceil(),
            Self::Half => (tokens_available * (1.0 / 2.0)).ceil(),
            Self::TwoThirds => (tokens_available * (2.0 / 3.0)).ceil(),
            Self::All => tokens_available,
        };

        allotment as u64
    }
}

impl Default for TokenLimit {
    fn default() -> Self {
        TokenLimit::Third
    }
}
