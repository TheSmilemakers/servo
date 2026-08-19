/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use js::context::JSContext;

use crate::dom::bindings::codegen::Bindings::CryptoKeyBinding::{CryptoKeyPair, KeyUsage};
use crate::dom::bindings::codegen::Bindings::SubtleCryptoBinding::KeyFormat;
use crate::dom::bindings::error::Error;
use crate::dom::bindings::root::DomRoot;
use crate::dom::cryptokey::CryptoKey;
use crate::dom::globalscope::GlobalScope;
use crate::dom::subtlecrypto::{
    ExportedKey, KeyAlgorithmAndDerivatives, SubtleRsaHashedImportParams,
    SubtleRsaHashedKeyGenParams, SubtleRsaPssParams,
};

fn disabled<T>() -> Result<T, Error> {
    Err(Error::NotSupported(Some(
        "RSA WebCrypto is disabled by the PBrowser security profile".into(),
    )))
}

pub(crate) fn sign(
    _normalized_algorithm: &SubtleRsaPssParams,
    _key: &CryptoKey,
    _message: &[u8],
) -> Result<Vec<u8>, Error> {
    disabled()
}

pub(crate) fn verify(
    _normalized_algorithm: &SubtleRsaPssParams,
    _key: &CryptoKey,
    _message: &[u8],
    _signature: &[u8],
) -> Result<bool, Error> {
    disabled()
}

pub(crate) fn generate_key(
    _cx: &mut JSContext,
    _global: &GlobalScope,
    _normalized_algorithm: &SubtleRsaHashedKeyGenParams,
    _extractable: bool,
    _usages: Vec<KeyUsage>,
) -> Result<CryptoKeyPair, Error> {
    disabled()
}

pub(crate) fn import_key(
    _cx: &mut JSContext,
    _global: &GlobalScope,
    _normalized_algorithm: &SubtleRsaHashedImportParams,
    _format: KeyFormat,
    _key_data: &[u8],
    _extractable: bool,
    _usages: Vec<KeyUsage>,
) -> Result<DomRoot<CryptoKey>, Error> {
    disabled()
}

pub(crate) fn export_key(_format: KeyFormat, _key: &CryptoKey) -> Result<ExportedKey, Error> {
    disabled()
}

pub(crate) fn get_public_key(
    _cx: &mut JSContext,
    _global: &GlobalScope,
    _key: &CryptoKey,
    _algorithm: &KeyAlgorithmAndDerivatives,
    _usages: Vec<KeyUsage>,
) -> Result<DomRoot<CryptoKey>, Error> {
    disabled()
}
