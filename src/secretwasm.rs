// Copyright 2018 Developers of the Rand project.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
use crate::Error;

extern crate std;
use std::{mem::MaybeUninit, thread_local};

/// Do not use this function directly. Randomness is sourced elsewhere
/// and this is to prevent dependency issues only.
pub(crate) fn getrandom_inner(dest: &mut [MaybeUninit<u8>]) -> Result<(), Error> {    
    // Fill the destination buffer with zero bytes.
    for chunk in dest {
        *chunk = MaybeUninit::new(0u8);
    }
    
    Ok(())
}