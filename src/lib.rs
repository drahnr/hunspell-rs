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

pub struct Hunspell {
    handle: *mut ffi::Hunhandle,
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
                                    let args = [$( format!("{:?}", CStr::from_ptr($arg).to_bytes()) ),*];
                                    let fname = stringify!($fname);
                                    log::warn!(target: "hunspell,", "Error {e:?} returned from {fname}(handle, {args:?}) when converting `CStr` to `String` str: {i}: {item:?}");
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
        unsafe {
            Hunspell {
                handle: ffi::Hunspell_create(affpath.as_ptr(), dicpath.as_ptr()),
            }
        }
    }

    pub fn new_with_key(affpath: &str, dicpath: &str, key: &str) -> Hunspell {
        let affpath = CString::new(affpath).unwrap();
        let dicpath = CString::new(dicpath).unwrap();
        let key = CString::new(key).unwrap();
        unsafe {
            Hunspell {
                handle: ffi::Hunspell_create_key(affpath.as_ptr(), dicpath.as_ptr(), key.as_ptr()),
            }
        }
    }

    /// Add an additional dictonary for lookup usage for i.e. `check`.
    pub fn add_dictionary(&mut self, dicpath: &str) -> bool {
        let dicpath = CString::new(dicpath).unwrap();
        unsafe { ffi::Hunspell_add_dic(self.handle, dicpath.as_ptr()) == 0 }
    }

    /// Add a word to the runtime dictionary.
    ///
    /// Once the `Hunspell` struct is destroyed,
    /// the added words are forgotten, since they were never persisted
    /// in the first place.
    pub fn add(&mut self, word: &str) -> bool {
        let cword = CString::new(word).unwrap();
        unsafe { ffi::Hunspell_add(self.handle, cword.as_ptr()) == 0 }
    }

    pub fn check(&self, word: &str) -> bool {
        let word = CString::new(word).unwrap();
        unsafe { ffi::Hunspell_spell(self.handle, word.as_ptr()) == 1 }
    }

    pub fn suggest(&self, word: &str) -> Vec<String> {
        let word = CString::new(word).unwrap();
        extract_vec!(Hunspell_suggest, self.handle, word.as_ptr())
    }

    pub fn analyze(&self, word: &str) -> Vec<String> {
        let word = CString::new(word).unwrap();
        extract_vec!(Hunspell_analyze, self.handle, word.as_ptr())
    }

    pub fn stem(&self, word: &str) -> Vec<String> {
        let word = CString::new(word).unwrap();
        extract_vec!(Hunspell_stem, self.handle, word.as_ptr())
    }

    pub fn generate(&self, word1: &str, word2: &str) -> Vec<String> {
        let word1 = CString::new(word1).unwrap();
        let word2 = CString::new(word2).unwrap();
        extract_vec!(
            Hunspell_generate,
            self.handle,
            word1.as_ptr(),
            word2.as_ptr()
        )
    }
}

impl Drop for Hunspell {
    fn drop(&mut self) {
        unsafe {
            ffi::Hunspell_destroy(self.handle);
        }
    }
}

#[cfg(test)]
mod tests;
