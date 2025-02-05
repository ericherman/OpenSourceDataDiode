// Copyright 2020 Ministerie van Defensie
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![allow(clippy::large_enum_variant)]
use error_chain::*;

pub fn clean_unwrap<T>(res: Result<T>) -> T {
    match res {
        Ok(v) => v,
        Err(e) => panic!("{}", e.display_chain()),
    }
}

error_chain! {
    types {
        Error, ErrorKind, ResultExt, Result;
    }
    foreign_links {
        Io(::std::io::Error);
    }
    errors {
        UdpSocketError(t: String) {
            description("Udp Socket error")
            display("Udp Socket error: '{}'", t)
        }
        CommandError(t: String) {
            description("Cannot execute command")
            display("Cannot execute command: {}", t)
        }
    }
}
