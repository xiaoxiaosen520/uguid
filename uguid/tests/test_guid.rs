// Copyright 2022 Google LLC
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use core::mem;
use uguid::{guid, Guid, GuidFromStrError, Variant};

#[test]
fn test_guid() {
    assert_eq!(mem::size_of::<Guid>(), 16);
    assert_eq!(mem::align_of::<Guid>(), 4);

    // Constructors.
    let guid = Guid::new(
        0x01234567_u32.to_le_bytes(),
        0x89ab_u16.to_le_bytes(),
        0xcdef_u16.to_le_bytes(),
        0x01,
        0x23,
        [0x45, 0x67, 0x89, 0xab, 0xcd, 0xef],
    );
    let guid2 = Guid::from_bytes([
        0x67, 0x45, 0x23, 0x01, 0xab, 0x89, 0xef, 0xcd, 0x01, 0x23, 0x45, 0x67,
        0x89, 0xab, 0xcd, 0xef,
    ]);
    assert_eq!(guid, guid2);

    // Accessors.
    assert_eq!(guid.time_low(), [0x67, 0x45, 0x23, 0x01]);
    assert_eq!(guid.time_mid(), [0xab, 0x89]);
    assert_eq!(guid.time_high_and_version(), [0xef, 0xcd]);
    assert_eq!(guid.clock_seq_high_and_reserved(), 0x01);
    assert_eq!(guid.clock_seq_low(), 0x23);
    assert_eq!(guid.node(), [0x45, 0x67, 0x89, 0xab, 0xcd, 0xef]);

    // To byte array.
    assert_eq!(
        guid.to_bytes(),
        [
            0x67, 0x45, 0x23, 0x01, 0xab, 0x89, 0xef, 0xcd, 0x01, 0x23, 0x45,
            0x67, 0x89, 0xab, 0xcd, 0xef
        ]
    );

    // Formatting.
    assert_eq!(
        guid.to_ascii_hex_lower(),
        *b"01234567-89ab-cdef-0123-456789abcdef"
    );
    assert_eq!(guid.to_string(), "01234567-89ab-cdef-0123-456789abcdef");

    // Parsing.
    assert_eq!(
        "01234567-89ab-cdef-0123-456789abcdef"
            .parse::<Guid>()
            .unwrap(),
        guid
    );
    assert_eq!(
        Guid::try_parse("01234567-89ab-cdef-0123-456789abcdef").unwrap(),
        guid
    );

    // Macro.
    assert_eq!(guid!("01234567-89ab-cdef-0123-456789abcdef"), guid);
}

#[test]
fn test_from_random_bytes() {
    let random_bytes = [
        0x68, 0xc0, 0x5f, 0xd7, 0x78, 0x21, 0xf9, 0x01, 0x66, 0x15, 0xab, 0x54,
        0xe9, 0xcc, 0x44, 0xb0,
    ];
    let expected_bytes = [
        0x68, 0xc0, 0x5f, 0xd7, 0x78, 0x21, 0xf9, 0x4f, 0xa6, 0x15, 0xab, 0x54,
        0xe9, 0xcc, 0x44, 0xb0,
    ];

    let guid = Guid::from_random_bytes(random_bytes);
    assert_eq!(guid.to_bytes(), expected_bytes);
    assert_eq!(guid.variant(), Variant::Rfc4122);
    assert_eq!(guid.version(), 4);
}

#[test]
fn test_parse_or_panic_success() {
    let _g = Guid::parse_or_panic("01234567-89ab-cdef-0123-456789abcdef");
}

#[test]
#[should_panic]
fn test_parse_or_panic_len() {
    let _g = Guid::parse_or_panic("01234567-89ab-cdef-0123-456789abcdef0");
}

#[test]
#[should_panic]
fn test_parse_or_panic_sep() {
    let _g = Guid::parse_or_panic("01234567089ab-cdef-0123-456789abcdef");
}

#[test]
#[should_panic]
fn test_parse_or_panic_hex() {
    let _g = Guid::parse_or_panic("g1234567-89ab-cdef-0123-456789abcdef");
}

#[test]
fn test_guid_error() {
    // Wrong length.
    let s = "01234567-89ab-cdef-0123-456789abcdef0";
    assert_eq!(s.len(), 37);
    assert_eq!(s.parse::<Guid>(), Err(GuidFromStrError::Length));

    // Wrong separator.
    let s = "01234567089ab-cdef-0123-456789abcdef";
    assert_eq!(s.parse::<Guid>(), Err(GuidFromStrError::Separator(8)));
    let s = "01234567-89ab0cdef-0123-456789abcdef";
    assert_eq!(s.parse::<Guid>(), Err(GuidFromStrError::Separator(13)));
    let s = "01234567-89ab-cdef00123-456789abcdef";
    assert_eq!(s.parse::<Guid>(), Err(GuidFromStrError::Separator(18)));
    let s = "01234567-89ab-cdef-01230456789abcdef";
    assert_eq!(s.parse::<Guid>(), Err(GuidFromStrError::Separator(23)));

    // Invalid hex.
    let s = "g1234567-89ab-cdef-0123-456789abcdef";
    assert_eq!(s.parse::<Guid>(), Err(GuidFromStrError::Hex(0)));

    assert_eq!(
        GuidFromStrError::Length.to_string(),
        "GUID string has wrong length (expected 36 bytes)"
    );
    assert_eq!(
        GuidFromStrError::Separator(8).to_string(),
        "GUID string is missing a separator (`-`) at index 8"
    );
    assert_eq!(
        GuidFromStrError::Hex(10).to_string(),
        "GUID string contains invalid ASCII hex at index 10"
    );
}

#[test]
fn test_guid_variant() {
    assert_eq!(
        guid!("00000000-0000-0000-0000-000000000000").variant(),
        Variant::ReservedNcs
    );
    assert_eq!(
        guid!("00000000-0000-0000-8000-000000000000").variant(),
        Variant::Rfc4122
    );
    assert_eq!(
        guid!("00000000-0000-0000-c000-000000000000").variant(),
        Variant::ReservedMicrosoft
    );
    assert_eq!(
        guid!("00000000-0000-0000-e000-000000000000").variant(),
        Variant::ReservedFuture
    );
}

#[test]
fn test_guid_version() {
    assert_eq!(guid!("00000000-0000-0000-8000-000000000000").version(), 0);
    assert_eq!(guid!("00000000-0000-1000-8000-000000000000").version(), 1);
    assert_eq!(guid!("00000000-0000-2000-8000-000000000000").version(), 2);
    assert_eq!(guid!("00000000-0000-4000-8000-000000000000").version(), 4);
}

#[test]
fn test_guid_is_zero() {
    assert!(guid!("00000000-0000-0000-0000-000000000000").is_zero());
    assert!(!guid!("308bbc16-a308-47e8-8977-5e5646c5291f").is_zero());
}

/// Inner module that only imports the `guid!` macro.
mod inner {
    use uguid::guid;

    /// Test that the `guid!` macro works without importing anything
    /// else.
    #[test]
    fn test_guid_macro_paths() {
        let _g = guid!("01234567-89ab-cdef-0123-456789abcdef");
    }
}
