// Copyright 2020, Collabora Ltd.
// SPDX-License-Identifier: MIT

use thiserror::Error;
#[derive(Error, Debug)]
pub enum Error {
    #[error("Creation failed")]
    CreationFailed,
    #[error("No memory")]
    NoMemory,
    #[error("Wrong proxy type")]
    WrongProxyType,
}
