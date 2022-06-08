// © Wasmium Network Developers

// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file,
// You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::{Custodian, TokenLimit, WalletError, WalletResult};
use borsh::{BorshDeserialize, BorshSerialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, BorshDeserialize, BorshSerialize)]
pub struct OnChainWallet {
    custodians: [Custodian; 5],
    signers: u8,
    checking: [u8; 32],
    limit: TokenLimit,
    request: TransferRequest,
}

impl OnChainWallet {
    pub fn new() -> Self {
        OnChainWallet {
            custodians: [Custodian::default(); 5],
            signers: 0,
            checking: [0u8; 32], //TODO
            limit: TokenLimit::default(),
            request: TransferRequest::default(),
        }
    }

    pub fn add_custodian(&mut self, custodian: Custodian) -> WalletResult<&mut Self> {
        if let Some((index, _)) = self
            .custodians
            .iter()
            .enumerate()
            .find(|(_, custodian)| custodian.public_key() == [0u8; 32])
        {
            self.custodians[index] = custodian;

            Ok(self)
        } else {
            Err(WalletError::CustodianStoreFull)
        }
    }

    pub fn add_custodian_signers(&mut self, threshold: u8) -> WalletResult<&mut Self> {
        let valid_keys = self.valid_keys();

        if (valid_keys.len() as u8) < self.signers + 1 {
            return Err(WalletError::NotEnoughCustodiansForSignerIncrease);
        } else {
            self.signers = threshold;

            Ok(self)
        }
    }
    fn valid_keys(&self) -> Vec<Custodian> {
        let mut valid_keys = Vec::<Custodian>::new();

        self.custodians.into_iter().for_each(|custodian| {
            if custodian.public_key() != [0u8; 32] {
                valid_keys.push(custodian)
            }
        });

        valid_keys
    }

    pub fn new_request(&mut self, amount: u64) -> &mut Self {
        self.request.amount = amount;

        self
    }

    pub fn approve_request(
        &mut self,
        amount: u64,
        custodian_public_key: [u8; 32],
    ) -> WalletResult<TransferOutcome> {
        if self.request.amount != amount {
            return Err(WalletError::RequestedAmountMismatch);
        }

        let valid_keys = self.valid_keys();

        if valid_keys
            .iter()
            .find(|custodian| custodian.public_key() == custodian_public_key)
            .is_some()
        {
            let mut approved_by = self.parse_approved_by();
            approved_by.push(custodian_public_key);

            if (approved_by.len() as u8) >= self.signers {
                self.request.approved_by = [[0u8; 32]; 3];

                approved_by.into_iter().enumerate().for_each(|(index, pk)| {
                    self.request.approved_by[index] = pk;
                });

                return Ok(TransferOutcome::Approved);
            } else {
                self.request.approved_by = [[0u8; 32]; 3];

                approved_by.into_iter().enumerate().for_each(|(index, pk)| {
                    self.request.approved_by[index] = pk;
                });

                return Ok(TransferOutcome::Pending);
            }
        } else {
            return Err(WalletError::PublicKeyIsNotACustodian);
        }
    }

    pub fn drop_request(&mut self) -> &mut Self {
        self.request = TransferRequest::default();

        self
    }

    fn parse_approved_by(&self) -> Vec<[u8; 32]> {
        let mut valid_custodians = Vec::<[u8; 32]>::new();

        self.request
            .approved_by
            .into_iter()
            .for_each(|custodian_pk| {
                if custodian_pk != [0u8; 32] {
                    valid_custodians.push(custodian_pk)
                }
            });

        valid_custodians
    }

    pub fn change_limit(&mut self, new_limit: TokenLimit) -> &mut Self {
        self.limit = new_limit;

        self
    }

    pub fn custodians(&self) -> [Custodian; 5] {
        self.custodians
    }

    pub fn signers(&self) -> u8 {
        self.signers
    }

    pub fn limit(&self) -> TokenLimit {
        self.limit
    }
}

impl Default for OnChainWallet {
    fn default() -> Self {
        OnChainWallet::new()
    }
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, BorshDeserialize, BorshSerialize, Default,
)]

pub struct TransferRequest {
    credit_to: [u8; 32],
    amount: u64,
    approved_by: [[u8; 32]; 3],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, BorshDeserialize, BorshSerialize)]
pub enum TransferOutcome {
    Pending,
    Approved,
}

impl Default for TransferOutcome {
    fn default() -> Self {
        TransferOutcome::Pending
    }
}
