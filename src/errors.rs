// © Wasmium Network Developers

// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file,
// You can obtain one at https://mozilla.org/MPL/2.0/.

use zeroed_store::StoreError;

pub type WalletResult<T> = Result<T, WalletError>;

#[derive(Debug)]
pub enum WalletError {
    /// An increase in threshold of signing `Custodian`s is not enough
    /// since registered `Custodian`s are lower than the increase
    NotEnoughCustodiansForSignerIncrease,
    /// An error from `zeroed_store` crate
    Store(StoreError),
    /// The storage part for `TokenLimit` is corrupted
    TokenLimitNotApplicable,
    /// No more `Custodian`s can be added to the storage,
    /// remove an existing `Custodian` first
    CustodianStoreFull,
    /// The amount requested by the owner of the wallet does not
    /// match the amount approved by the `Custodian`
    RequestedAmountMismatch,
    /// The public key that approved the request is not a custodian
    PublicKeyIsNotACustodian,
}

impl From<StoreError> for WalletError {
    fn from(error: StoreError) -> Self {
        WalletError::Store(error)
    }
}
