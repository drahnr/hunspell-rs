//   Copyright 2016 Lipka Boldizsár
//   Copyright 2019 Alberto Piai
//   Copyright 2020 Bernhard Schuster
//
//   Licensed under the Apache License, Version 2.0 (the "License");
//   you may not use this file except in compliance with the License.
//   You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
//   Unless required by applicable law or agreed to in writing, software
//   distributed under the License is distributed on an "AS IS" BASIS,
//   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//   See the License for the specific language governing permissions and
//   limitations under the License.

use std::ffi::{CStr, CString};
use std::ptr::null_mut;

use hunspell_sys as ffi;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CheckResult {
    FoundInDictionary,
    MissingInDictionary,
}

pub struct Hunspell {
    // some operations of hunspell have global state
    // hence the lock is needed around those
    guarded_handle: std::sync::Mutex<*mut ffi::Hunhandle>,
}

fn to_hex_str(s: &CStr) -> String {
    let bytes = s.to_bytes();
    let mut acc = String::with_capacity(6 * bytes.len());
    let mut iter = bytes.iter();
    acc += "[";
    if let Some(byte) = iter.next() {
        acc += &format!("0x{:02x}",byte);
        for byte in iter {
            acc += &format!(", 0x{:02x}",byte);
        }
    }
    acc += "]";
    acc
}

macro_rules! extract_vec {
    ( $fname:ident, $handle:expr, $( $arg:expr ),* ) => {
        {
            let mut result = Vec::new();
            unsafe {
                let mut list = null_mut();
                let n = ffi::$fname($handle, &mut list, $( $arg ),*) as isize;
                if n != 0 && list != null_mut() {
                    for i in 0..n {
                        let item_ptr_ptr = list.offset(i);
                        if *item_ptr_ptr != null_mut() {
                            let item = CStr::from_ptr(*item_ptr_ptr);
                            match item.to_str() {
                                Ok(s) => result.push(String::from(s)),
                                Err(e) => {
                                    let args = [$( format!("{:?}", CStr::from_ptr($arg)) ),*];
                                    let fname = stringify!($fname);
                                    let hex = to_hex_str(item);
                                    log::warn!(target: "hunspell,", "Error {e:?} returned from {fname}(handle, {args:?}) when converting `CStr` to `String` str: {i}: {hex:?}");
                                },
                            }
                        } else {
                            let args = [$( format!("{:?}", CStr::from_ptr($arg)) ),*];
                            let fname = stringify!($fname);
                            log::warn!(target: "hunspell", "Suggestion list contained null-pointer when calling {fname}(handle, {args:?})");
                        }
                    }
                    ffi::Hunspell_free_list($handle, &mut list, n as i32);
                }
            }
            result
        }
    }
}

impl Hunspell {
    pub fn new(affpath: &str, dicpath: &str) -> Hunspell {
        let affpath = CString::new(affpath).unwrap();
        let dicpath = CString::new(dicpath).unwrap();
        let handle = unsafe { ffi::Hunspell_create(affpath.as_ptr(), dicpath.as_ptr()) };
        Hunspell {
            guarded_handle: std::sync::Mutex::new(handle),
        }
    }

    pub fn new_with_key(affpath: &str, dicpath: &str, key: &str) -> Hunspell {
        let affpath = CString::new(affpath).unwrap();
        let dicpath = CString::new(dicpath).unwrap();
        let key = CString::new(key).unwrap();

        let handle =
            unsafe { ffi::Hunspell_create_key(affpath.as_ptr(), dicpath.as_ptr(), key.as_ptr()) };
        Hunspell {
            guarded_handle: std::sync::Mutex::new(handle),
        }
    }

    /// Add an additional dictonary for lookup usage for i.e. `check`.
    pub fn add_dictionary(&mut self, dicpath: &str) -> bool {
        let dicpath = CString::new(dicpath).unwrap();
        let handle = self.guarded_handle.lock().unwrap();
        unsafe { ffi::Hunspell_add_dic(*handle, dicpath.as_ptr()) == 0 }
    }

    /// Add a word to the runtime dictionary.
    ///
    /// Once the `Hunspell` struct is destroyed,
    /// the added words are forgotten, since they were never persisted
    /// in the first place.
    pub fn add(&mut self, word: &str) -> bool {
        let cword = CString::new(word).unwrap();
        let handle = self.guarded_handle.lock().unwrap();
        unsafe { ffi::Hunspell_add(*handle, cword.as_ptr()) == 0 }
    }

    pub fn check(&self, word: &str) -> CheckResult {
        let word = CString::new(word).unwrap();
        let handle = self.guarded_handle.lock().unwrap();
        let ret = unsafe { ffi::Hunspell_spell(*handle, word.as_ptr()) };
        match ret {
            0 => CheckResult::MissingInDictionary,
            _ => CheckResult::FoundInDictionary,
        }
    }

    pub fn suggest(&self, word: &str) -> Vec<String> {
        let word = CString::new(word).unwrap();
        let handle = self.guarded_handle.lock().unwrap();
        extract_vec!(Hunspell_suggest, *handle, word.as_ptr())
    }

    pub fn analyze(&self, word: &str) -> Vec<String> {
        let word = CString::new(word).unwrap();
        let handle = self.guarded_handle.lock().unwrap();
        extract_vec!(Hunspell_analyze, *handle, word.as_ptr())
    }

    pub fn stem(&self, word: &str) -> Vec<String> {
        let word = CString::new(word).unwrap();
        let handle = self.guarded_handle.lock().unwrap();
        extract_vec!(Hunspell_stem, *handle, word.as_ptr())
    }

    pub fn generate(&self, word1: &str, word2: &str) -> Vec<String> {
        let word1 = CString::new(word1).unwrap();
        let word2 = CString::new(word2).unwrap();
        let handle = self.guarded_handle.lock().unwrap();
        extract_vec!(Hunspell_generate, *handle, word1.as_ptr(), word2.as_ptr())
    }
}

impl Drop for Hunspell {
    fn drop(&mut self) {
        let handle = self.guarded_handle.lock().unwrap();
        unsafe {
            ffi::Hunspell_destroy(*handle);
        }
    }
}

#[cfg(test)]
mod tests;
