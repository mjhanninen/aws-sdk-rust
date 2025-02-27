/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0.
 */

#![no_main]
use libfuzzer_sys::fuzz_target;
use smithy_eventstream::error::Error;
use smithy_eventstream::frame::Message;

fuzz_target!(|message: Message| {
    let mut buffer = Vec::new();
    match message.write_to(&mut buffer) {
        Err(
            Error::HeadersTooLong
            | Error::PayloadTooLong
            | Error::MessageTooLong
            | Error::InvalidHeaderNameLength
            | Error::TimestampValueTooLarge(_),
        ) => {}
        Err(err) => panic!("unexpected error on write: {}", err),
        Ok(_) => {
            let mut data = &buffer[..];
            let parsed = Message::read_from(&mut data).unwrap();
            assert_eq!(message, parsed);
        }
    }
});
