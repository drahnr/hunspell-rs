// TODO move this into a separate crate
// extracted from crate `hunspell` which is horribly outdated
// and adapted to use hunspell-sys

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
                if n != 0 {
                    for i in 0..n {
                        let item = CStr::from_ptr(*list.offset(i));
                        result.push(String::from(item.to_str().unwrap()));
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
