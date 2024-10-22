//! BSD 3-Clause License
//!
//! Copyright (c) 2024,
//!     Marcus Cvjeticanin
//!     Chase Willden
//!
//! Redistribution and use in source and binary forms, with or without
//! modification, are permitted provided that the following conditions are met.
//!
//! 1. Redistributions of source code must retain the above copyright notice, this
//!    list of conditions and the following disclaimer.
//!
//! 2. Redistributions in binary form must reproduce the above copyright notice,
//!    this list of conditions and the following disclaimer in the documentation
//!    and/or other materials provided with the distribution.
//!
//! 3. Neither the name of the copyright holder nor the names of its
//!    contributors may be used to endorse or promote products derived from this software without specific prior written permission.
//!
//! THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
//! AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
//! IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
//! DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
//! FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
//! DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
//! SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
//! CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
//! OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
//! OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

use std::{
    fmt::{Debug, Display},
    str::FromStr,
};

use serde::{Deserialize, Deserializer};

/// A simple primary key wrapper.
///
/// The `PrimaryKey` struct wraps a value that acts as a primary key in a database.
/// This struct provides various utility methods and traits for working with primary keys.
///
/// # Type Parameters
///
/// * `T` - The type of the primary key value, which can be any type.
#[derive(Debug, Clone)]
pub struct PrimaryKey<T>(T);

impl<T> PrimaryKey<T> {
    /// Creates a new `PrimaryKey` from the given value.
    ///
    /// # Arguments
    ///
    /// * `value` - The value of the primary key.
    ///
    /// # Returns
    ///
    /// A new `PrimaryKey` instance containing the given value.
    pub fn new(value: T) -> Self {
        PrimaryKey(value)
    }

    /// Retrieves a reference to the value inside the `PrimaryKey`.
    ///
    /// # Returns
    ///
    /// A reference to the primary key value.
    pub fn get(&self) -> &T {
        &self.0
    }
}

impl<T: Default> Default for PrimaryKey<T> {
    /// Creates a default `PrimaryKey` using the `Default` trait of `T`.
    fn default() -> Self {
        PrimaryKey(T::default())
    }
}

impl<T: Debug + Display> Display for PrimaryKey<T> {
    /// Implements the `Display` trait for `PrimaryKey`.
    ///
    /// Formats the primary key using the inner value's `Display` implementation.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<T: Debug + FromStr> FromStr for PrimaryKey<T> {
    type Err = T::Err;

    /// Implements the `FromStr` trait for `PrimaryKey`.
    ///
    /// Tries to parse a string into the inner type `T` and returns a `PrimaryKey`.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<T>() {
            Ok(value) => Ok(PrimaryKey(value)),
            Err(err) => Err(err),
        }
    }
}

impl<'de, T> Deserialize<'de> for PrimaryKey<T>
where
    T: FromStr + Debug,
    <T as FromStr>::Err: Debug + Display,
{
    /// Implements the `Deserialize` trait for `PrimaryKey`.
    ///
    /// Deserializes a string into the inner type `T` and wraps it into a `PrimaryKey`.
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value =
            T::from_str(&String::deserialize(deserializer)?).map_err(serde::de::Error::custom)?;
        Ok(PrimaryKey(value))
    }
}

/// A wrapper for auto-incremented primary keys.
///
/// The `AutoIncrementPrimaryKey` struct wraps an optional value that can act as an auto-incremented
/// primary key in a database.
///
/// # Type Parameters
///
/// * `T` - The type of the primary key value, which can be any type.
#[derive(Debug, Clone)]
pub struct AutoIncrementPrimaryKey<T>(Option<T>);

impl<T> AutoIncrementPrimaryKey<T> {
    /// Creates a new `AutoIncrementPrimaryKey` from the given optional value.
    ///
    /// # Arguments
    ///
    /// * `value` - An optional value of the primary key.
    ///
    /// # Returns
    ///
    /// A new `AutoIncrementPrimaryKey` instance containing the given value.
    pub fn new(value: Option<T>) -> Self {
        AutoIncrementPrimaryKey(value)
    }

    /// Retrieves a reference to the value inside the `AutoIncrementPrimaryKey`, if it exists.
    ///
    /// # Returns
    ///
    /// An optional reference to the primary key value.
    pub fn get(&self) -> Option<&T> {
        self.0.as_ref()
    }

    /// Sets the value of the `AutoIncrementPrimaryKey`.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to set as the primary key.
    pub fn set(&mut self, value: T) {
        self.0 = Some(value);
    }
}

impl<T: Debug> Default for AutoIncrementPrimaryKey<T> {
    /// Creates a default `AutoIncrementPrimaryKey` with no value (i.e., `None`).
    fn default() -> Self {
        AutoIncrementPrimaryKey(None)
    }
}

impl<T: Debug + Display> Display for AutoIncrementPrimaryKey<T> {
    /// Implements the `Display` trait for `AutoIncrementPrimaryKey`.
    ///
    /// Formats the primary key using the inner value's `Display` implementation if it exists,
    /// otherwise displays "NULL".
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            Some(value) => write!(f, "{}", value),
            None => write!(f, "NULL"),
        }
    }
}

impl<T: Debug + FromStr> FromStr for AutoIncrementPrimaryKey<T> {
    type Err = T::Err;

    /// Implements the `FromStr` trait for `AutoIncrementPrimaryKey`.
    ///
    /// Tries to parse a string into the inner type `T` and wraps it into an `AutoIncrementPrimaryKey`.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<T>() {
            Ok(value) => Ok(AutoIncrementPrimaryKey(Some(value))),
            Err(_) => Ok(AutoIncrementPrimaryKey(None)),
        }
    }
}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for AutoIncrementPrimaryKey<T> {
    /// Implements the `Deserialize` trait for `AutoIncrementPrimaryKey`.
    ///
    /// Deserializes an optional value into the inner type `T` and wraps it into an `AutoIncrementPrimaryKey`.
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: Option<T> = Option::deserialize(deserializer)?;
        Ok(AutoIncrementPrimaryKey(value))
    }
}

impl<T: PartialEq> PartialEq for AutoIncrementPrimaryKey<T> {
    /// Implements the `PartialEq` trait for `AutoIncrementPrimaryKey`.
    ///
    /// Checks equality between two `AutoIncrementPrimaryKey` instances by comparing their inner values.
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
