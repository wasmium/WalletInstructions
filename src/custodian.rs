// © Wasmium Network Developers

// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file,
// You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::{PublicKey, Tai64NTimestamp, TaiTimeUtilities};
use borsh::{BorshDeserialize, BorshSerialize};
use core::{fmt, ops::RangeInclusive};
use tai64::{Tai64, Tai64N};

pub const PACKED_CUSTODIAN: usize = 32 + 12 + 12;
pub const PUBLIC_KEY_RANGE: RangeInclusive<usize> = 0..=31;
pub const TIMESTAMP_RANGE: RangeInclusive<usize> = 32..=43;
pub const CLUSTER_TIMESTAMP_RANGE: RangeInclusive<usize> = 44..=55;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, BorshDeserialize, BorshSerialize)]
pub struct Custodian {
    public_key: PublicKey,
    timestamp: Tai64NTimestamp,
    cluster_timestamp: Tai64NTimestamp,
}

impl Custodian {
    pub fn new(public_key: PublicKey) -> Self {
        Custodian {
            public_key,
            timestamp: Tai64N::UNIX_EPOCH.to_bytes(),
            cluster_timestamp: Tai64N::UNIX_EPOCH.to_bytes(),
        }
    }

    pub fn add_cluster_timestamp(&mut self, unix_timestamp: i64) -> &mut Self {
        let tai_timestamp = Tai64::from_unix(unix_timestamp);
        let tai_timestamp: Tai64N = tai_timestamp.into();
        self.cluster_timestamp = tai_timestamp.to_bytes();

        self
    }

    pub fn build(mut self) -> Self {
        self.timestamp = Tai64N::now().to_bytes();

        self
    }

    pub fn public_key(&self) -> PublicKey {
        self.public_key
    }

    pub fn timestamp(&self) -> Tai64NTimestamp {
        self.timestamp
    }

    pub fn cluster_timestamp(&self) -> Tai64NTimestamp {
        self.cluster_timestamp
    }

    /// The Data is packed in a 56 byte array
    /// public key bytes | TAI64N Timestamp bytes | TAI64N Timestamp bytes
    /// 32 bytes | 12 bytes | 12 bytes
    pub fn pack(&self) -> [u8; PACKED_CUSTODIAN] {
        let mut packed = [0u8; PACKED_CUSTODIAN];
        packed[PUBLIC_KEY_RANGE].copy_from_slice(&self.public_key);
        packed[TIMESTAMP_RANGE].copy_from_slice(&self.timestamp);
        packed[CLUSTER_TIMESTAMP_RANGE].copy_from_slice(&self.cluster_timestamp);

        packed
    }

    pub fn unpack(packed_data: [u8; 56]) -> Custodian {
        let public_key: [u8; 32] = packed_data[PUBLIC_KEY_RANGE].try_into().unwrap(); // Never fails since it maxes out at 56 bytes
        let timestamp: [u8; 12] = packed_data[TIMESTAMP_RANGE].try_into().unwrap(); // Never fails since it maxes out at 56 bytes
        let cluster_timestamp: [u8; 12] = packed_data[CLUSTER_TIMESTAMP_RANGE].try_into().unwrap(); // Never fails since it maxes out at 56 bytes

        Custodian {
            public_key,
            timestamp,
            cluster_timestamp,
        }
    }
}

impl fmt::Debug for Custodian {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let timestamp = match TaiTimeUtilities::source("timestamp")
            .tai64n_bytes(self.timestamp)
            .tai64n_bytes_to_humantime()
        {
            Ok(value) => value,
            Err(error) => return f.write_str(&error),
        };

        let cluster_timestamp = match TaiTimeUtilities::source("cluster_timestamp")
            .tai64n_bytes(self.cluster_timestamp)
            .tai64n_bytes_to_humantime()
        {
            Ok(value) => value,
            Err(error) => return f.write_str(&error),
        };

        f.debug_struct("Custodian")
            .field("public_key", &bs58::encode(&self.public_key).into_string())
            .field("timestamp", &timestamp)
            .field("cluster_timestamp", &cluster_timestamp)
            .finish()
    }
}

impl Default for Custodian {
    fn default() -> Self {
        Custodian::new(PublicKey::default())
    }
}
