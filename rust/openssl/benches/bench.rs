// Copyright (c) Meta Platforms, Inc. and affiliates.
//
// This source code is dual-licensed under either the MIT license found in the
// LICENSE-MIT file in the root directory of this source tree or the Apache
// License, Version 2.0 found in the LICENSE-APACHE file in the root directory
// of this source tree. You may select, at your option, one of the above-listed licenses.

extern crate openssl_benchmarks;

fn main() {
    perf_caliper::benchmark_main::main(Some(|metadata| {
        metadata.insert(
            "version".to_string(),
            openssl_benchmarks::version().to_string(),
        );
        metadata.remove("path");
    }));
}
