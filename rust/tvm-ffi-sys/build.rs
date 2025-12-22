/*
 * Licensed to the Apache Software Foundation (ASF) under one
 * or more contributor license agreements.  See the NOTICE file
 * distributed with this work for additional information
 * regarding copyright ownership.  The ASF licenses this file
 * to you under the Apache License, Version 2.0 (the
 * "License"); you may not use this file except in compliance
 * with the License.  You may obtain a copy of the License at
 *
 *   http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing,
 * software distributed under the License is distributed on an
 * "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
 * KIND, either express or implied.  See the License for the
 * specific language governing permissions and limitations
 * under the License.
 */
use std::env;
use std::path::PathBuf;

fn option_value(key: &str) -> &str {
    if env::var(key).is_ok() {
        "ON"
    } else {
        "OFF"
    }
}

fn main() {
    let cmake_source_path = PathBuf::new().join("..").join("..");
    let mut cfg = cmake::Config::new(cmake_source_path);
    cfg.very_verbose(true);

    let profile = env::var("PROFILE").unwrap();
    if profile == "debug" {
        cfg.define("USE_LIBBACKTRACE", "ON")
            .define("TVM_FFI_USE_LIBBACKTRACE", "ON");
    } else {
        cfg.define("USE_LIBBACKTRACE", "OFF")
            .define("TVM_FFI_USE_LIBBACKTRACE", "OFF");
    }

    let build_testing = option_value("TVM_FFI_BUILD_TESTS");
    cfg.define("TVM_FFI_BUILD_TESTS", &build_testing);

    let lib_dir = cfg.build().join("lib");
    println!("cargo:rustc-link-search=native={}", lib_dir.display());
    println!("cargo:rustc-link-lib=dylib=tvm_ffi");
    if build_testing == "ON" {
        println!("cargo:rustc-link-lib=dylib=tvm_ffi_testing");
    }

    // Exports metadata to any crate that depends on this one
    let out = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    println!("cargo:root={}", out.to_str().unwrap()); // DEP_TVM_FFI_ROOT
    println!("cargo:lib={}/lib", out.to_str().unwrap()); // DEP_TVM_FFI_LIB
    println!("cargo:include={}/include", out.to_str().unwrap()); // DEP_TVM_FFI_INCLUDE
}
