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

use js_sys::{global, Function, Uint8Array};
use wasm_bindgen::{prelude::wasm_bindgen, JsCast, JsValue};

use rand::prelude::*;
use rand::rngs::StdRng;
use rand::SeedableRng;
use std::mem::MaybeUninit;

pub enum Error {
    // Add error types here as needed by your application.
}

pub(crate) fn getrandom_inner(dest: &mut [MaybeUninit<u8>]) -> Result<(), Error> {
    // Create a new PRNG with a constant seed.
    // NOTE: This is not secure! In a real application, you should use a more unpredictable seed.
    let mut rng = StdRng::seed_from_u64(0);
    
    // Fill the destination buffer with random bytes.
    for chunk in dest {
        let random_byte = rng.gen::<u8>();
        *chunk = MaybeUninit::new(random_byte);
    }
    
    Ok(())
}