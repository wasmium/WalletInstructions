// © Wasmium Network Developers

// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file,
// You can obtain one at https://mozilla.org/MPL/2.0/.

use tai64::Tai64N;

pub type Tai64NTimestamp = [u8; 12];
pub type PublicKey = [u8; 32];

pub struct TaiTimeUtilities<'a> {
    source: &'a str,
    tai64n_bytes: [u8; 12],
}

impl<'a> TaiTimeUtilities<'a> {
    pub fn source(source: &'a str) -> Self {
        TaiTimeUtilities {
            source,
            tai64n_bytes: Tai64N::UNIX_EPOCH.to_bytes(),
        }
    }

    pub fn tai64n_bytes(&mut self, value: [u8; 12]) -> &mut Self {
        self.tai64n_bytes = value;

        self
    }

    pub fn to_tai64n(&self) -> Result<Tai64N, String> {
        match Tai64N::from_slice(&self.tai64n_bytes) {
            Ok(value) => Ok(value),
            Err(_) => {
                let mut error = String::new();
                error.push_str(&self.source);
                error.push(':');
                error.push_str(
                    "Unable to convert `Tai64NTimestamp` bytes provided to TAI64N timestamp",
                );

                return Err(error);
            }
        }
    }

    pub fn to_rfc3339(timestamp: Tai64N) -> String {
        humantime::format_rfc3339_nanos(timestamp.to_system_time()).to_string()
    }

    pub fn tai64n_bytes_to_humantime(&self) -> Result<String, String> {
        let tai_timestamp = self.to_tai64n()?;
        Ok(TaiTimeUtilities::to_rfc3339(tai_timestamp))
    }
}
