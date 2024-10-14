// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
use std::env;

use rquickjs::{loader::Resolver, Ctx, Error, Result};
use tracing::trace;

include!(concat!(env!("OUT_DIR"), "/bytecode_cache.rs"));

#[derive(Debug, Default)]
pub struct CustomResolver;

#[allow(clippy::manual_strip)]
impl Resolver for CustomResolver {
    fn resolve(&mut self, ctx: &Ctx, base: &str, name: &str) -> Result<String> {
        trace!("Try resolve '{}' from '{}'", name, base);
        require_resolve(ctx, name, base, true)
    }
}

// [CJS Reference Implementation](https://nodejs.org/api/modules.html#all-together)
// require(X) from module at path Y
pub fn require_resolve(ctx: &Ctx<'_>, x: &str, y: &str, is_esm: bool) -> Result<String> {
    trace!("require_resolve(x, y):({}, {})", x, y);

    // 1. If X is a core module,
    //   a. return the core module
    //   b. STOP

    // 1'. If X is a bytecode cache,
    for check_x in [x].iter() {
        if BYTECODE_CACHE.contains_key(check_x) {
            // a. return the bytecode cache
            // b. STOP
            trace!("+- Resolved by `BYTECODE_CACHE`: {}\n", check_x);
            return Ok(check_x.to_string());
        }
    }

    // 2. If X begins with '/'

    // 3. If X begins with './' or '/' or '../'

    // 4. If X begins with '#'

    // 5. LOAD_PACKAGE_SELF(X, dirname(Y))

    // 6. LOAD_NODE_MODULES(X, dirname(Y))

    // 6.5. LOAD_AS_FILE(X)

    // 7. THROW "not found"
    Err(Error::new_resolving(y.to_string(), x.to_string()))
}

// LOAD_AS_FILE(X)

// LOAD_INDEX(X)

// LOAD_AS_DIRECTORY(X)

// LOAD_NODE_MODULES(X, START)

// NODE_MODULES_PATHS(START)

// LOAD_PACKAGE_IMPORTS(X, DIR)

// LOAD_PACKAGE_EXPORTS(X, DIR)

// LOAD_PACKAGE_SELF(X, DIR)

// Implementation equivalent to PACKAGE_EXPORTS_RESOLVE including RESOLVE_ESM_MATCH
