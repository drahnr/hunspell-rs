//   Copyright 2016 Lipka Boldizsár
//   Copyright 2019 Alberto Piai
//   Copyright 2020 Bernhard Schuster
//   Copyright 2024 Frank Schuurmans
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

use hunspell_sys as ffi;
use std::{
    ffi::{CStr, CString},
    path::{Path, PathBuf},
    ptr::null_mut,
};

#[cfg(feature = "serde")]
use serde::Serialize;

use crate::{Error, Result};

/// Hunspell spelk checker.
///
///
// Should not derive Clone because when the struct is dropped
// the handle is destroyed, see manual impl Clone below.
// Deserialize is manually implemented.
#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Debug)]
pub struct SpellChecker {
    pub(crate) affix: PathBuf,
    pub(crate) dictionary: PathBuf,
    pub(crate) additional_dictionaries: Vec<PathBuf>,
    pub(crate) key: Option<String>,
    #[cfg_attr(feature = "serde", serde(skip))]
    pub(crate) handle: *mut ffi::Hunhandle,
}

impl SpellChecker {
    /// Opens a spell checking dictionary, which consist of a hunspell affix
    /// file (with the .aff extention) and the hunspell dictionary file itself
    /// (with the .dic extension). Both need to be existing files.
    ///
    /// For encrypted dictionaries use `new_with_key()`
    pub fn new<P>(affix: P, dictionary: P) -> Result<SpellChecker>
    where
        P: AsRef<Path>,
    {
        let (affix, dictionary) = check_paths(affix, dictionary)?;
        Ok(unsafe {
            SpellChecker {
                handle: ffi::Hunspell_create(
                    CString::new(affix.as_os_str().as_encoded_bytes())?.as_ptr(),
                    CString::new(dictionary.as_os_str().as_encoded_bytes())?.as_ptr(), // affix_cstring.as_ptr(), dicpath.as_ptr()
                ),
                affix,
                dictionary,
                additional_dictionaries: Vec::new(),
                key: None,
            }
        })
    }

    /// Opens an encrypted spell checking dictionary, which consist of a hunspell affix
    /// file (with the .aff extention) and the hunspell dictionary file itself
    /// (with the .dic extension). Both need to be existing files.
    ///
    /// The `key` (last) parameter is to decrypt the dictionaries encrypted by
    /// the hzip tool of the Hunspell distribution.
    pub fn new_with_key<P, S>(affix: P, dictionary: P, key: S) -> Result<SpellChecker>
    where
        P: AsRef<Path>,
        S: AsRef<str>,
    {
        let (affix, dictionary) = check_paths(affix, dictionary)?;
        Ok(unsafe {
            SpellChecker {
                handle: ffi::Hunspell_create_key(
                    CString::new(affix.as_os_str().as_encoded_bytes())?.as_ptr(),
                    CString::new(dictionary.as_os_str().as_encoded_bytes())?.as_ptr(),
                    CString::new(key.as_ref())?.as_ptr(),
                ),
                affix,
                dictionary,
                additional_dictionaries: Vec::new(),
                key: Some(key.as_ref().to_string()),
            }
        })
    }

    /// Returns the `Path` if the affix file.
    pub fn affix(&self) -> &Path {
        self.affix.as_path()
    }

    /// Returns the `Path` of the dictionary file.
    pub fn dictionary(&self) -> &Path {
        self.dictionary.as_path()
    }

    /// Add an additional dictonary for lookup usage for i.e. `check()`.
    ///
    /// The extra dictionaries use the affix file of `SpellChecker`.
    /// The maximal number of the extra dictionaries is limited ito 20.
    pub fn add_dictionary<P>(&mut self, dictionary: P) -> Result<bool>
    where
        P: AsRef<Path>,
    {
        if self.additional_dictionaries.len() == 20 {
            return Err(Error::CannotAddMoreDictionaries(
                dictionary.as_ref().to_path_buf(),
            ));
        }
        let dictionary = dictionary.as_ref().to_path_buf();
        if !dictionary.is_file() {
            return Err(Error::DictionaryFileIsNoFile(
                dictionary.to_string_lossy().into_owned(),
            ));
        }
        let dictionary_cstring = CString::new(dictionary.as_os_str().as_encoded_bytes())?;
        self.additional_dictionaries.push(dictionary);
        Ok(unsafe { ffi::Hunspell_add_dic(self.handle, dictionary_cstring.as_ptr()) == 0 })
    }

    /// Add a word to the runtime dictionary.
    ///
    /// When `SpellChecker` is dropped, the added words are as well.
    /// For a more permanent addition, create a dictionary file
    /// and load it with `add_dictionary()`.
    pub fn add<S>(&self, word: S) -> Result<()>
    where
        S: AsRef<str>,
    {
        let cword = CString::new(word.as_ref())?;

        let result = unsafe { ffi::Hunspell_add(self.handle, cword.as_ptr()) };
        if result == 0 {
            Ok(())
        } else {
            Err(Error::HunspellLibError(result))
        }
    }

    /// Add a word to the runtime dictionary. The example word is used
    /// as the model of the enabled affixation and compounding of the
    /// new word.
    ///
    /// When `SpellChecker` is dropped, the added words are as well.
    /// For a more permanent addition, create a dictionary file
    /// and load it with `add_dictionary()`.
    pub fn add_with_affix<S>(&self, word: S, example: S) -> Result<()>
    where
        S: AsRef<str>,
    {
        let cword = CString::new(word.as_ref())?;
        let cexample = CString::new(example.as_ref())?;
        let result =
            unsafe { ffi::Hunspell_add_with_affix(self.handle, cword.as_ptr(), cexample.as_ptr()) };
        if result == 0 {
            Ok(())
        } else {
            Err(Error::HunspellLibError(result))
        }
    }

    /// Remove a word added with `add()` or `add_with_affix()`.
    pub fn remove<S>(&self, word: S) -> Result<()>
    where
        S: AsRef<str>,
    {
        let cword = CString::new(word.as_ref())?;
        let result = unsafe { ffi::Hunspell_remove(self.handle, cword.as_ptr()) };
        if result == 0 {
            Ok(())
        } else {
            Err(Error::HunspellLibError(result))
        }
    }

    /// Returns true if the word is spelled correctly.
    pub fn check<S>(&self, word: S) -> Result<bool>
    where
        S: AsRef<str>,
    {
        let word = CString::new(word.as_ref())?;
        match unsafe { ffi::Hunspell_spell(self.handle, word.as_ptr()) } {
            // match ret {
            0 => Ok(false),
            _ => Ok(true),
        }
    }

    /// Returns a list of suggested spellings.
    pub fn suggest<S>(&self, word: S) -> Result<Vec<String>>
    where
        S: AsRef<str>,
    {
        let word = CString::new(word.as_ref())?;
        let mut list = null_mut();
        let n = unsafe { ffi::Hunspell_suggest(self.handle, &mut list, word.as_ptr()) };
        let strings = list_to_vec(list, n)?;
        // unsafe { ffi::Hunspell_free_list(self.handle, &mut list, n) };
        Ok(strings)
    }

    /// Morphological analysis
    pub fn analyze<S>(&self, word: S) -> Result<Vec<String>>
    where
        S: AsRef<str>,
    {
        let word = CString::new(word.as_ref())?;
        let mut list = null_mut();
        let n = unsafe { ffi::Hunspell_analyze(self.handle, &mut list, word.as_ptr()) };
        let strings = list_to_vec(list, n)?;
        unsafe { ffi::Hunspell_free_list(self.handle, &mut list, n) };

        Ok(strings)
    }

    /// Returns a list of stems
    pub fn stem<S>(&self, word: S) -> Result<Vec<String>>
    where
        S: AsRef<str>,
    {
        let word = CString::new(word.as_ref())?;
        let mut list = null_mut();
        let n = unsafe { ffi::Hunspell_stem(self.handle, &mut list, word.as_ptr()) };
        let strings = list_to_vec(list, n)?;
        // unsafe { ffi::Hunspell_free_list(self.handle, &mut list, n) };
        Ok(strings)
    }

    /// Returns a list of stems based on morphological analysis.
    pub fn extended_stem<S>(&self, word: S) -> Result<Vec<String>>
    where
        S: AsRef<str>,
    {
        let word = CString::new(word.as_ref())?;
        let mut analyzed = null_mut();
        let mut list = null_mut();
        let n_analyzed =
            unsafe { ffi::Hunspell_analyze(self.handle, &mut analyzed, word.as_ptr()) };
        let n = unsafe { ffi::Hunspell_stem2(self.handle, &mut list, analyzed, n_analyzed) };
        let strings = list_to_vec(list, n)?;
        unsafe {
            ffi::Hunspell_free_list(self.handle, &mut analyzed, n_analyzed);
            // ffi::Hunspell_free_list(self.handle, &mut list, n);
        }
        Ok(strings)
    }

    /// The second word and its affixation will be the model of the
    /// morphological generation of the requested forms of the first word.
    pub fn generate<S>(&self, word1: S, word2: S) -> Result<Vec<String>>
    where
        S: AsRef<str>,
    {
        let word1 = CString::new(word1.as_ref())?;
        let word2 = CString::new(word2.as_ref())?;
        let mut list = null_mut();
        let n = unsafe {
            ffi::Hunspell_generate(self.handle, &mut list, word1.as_ptr(), word2.as_ptr())
        };
        let strings = list_to_vec(list, n)?;
        // unsafe { ffi::Hunspell_free_list(self.handle, &mut list, n) };
        Ok(strings)
    }

    /// The second word and its affixation will be the model of the
    /// morphological generation of the requested forms of the first word.
    /// Returns a list of words based on morphological analysis of first word.
    pub fn extended_generate<S>(&self, word1: S, word2: S) -> Result<Vec<String>>
    where
        S: AsRef<str>,
    {
        let word1 = CString::new(word1.as_ref())?;
        let word2 = CString::new(word2.as_ref())?;
        let mut analyzed = null_mut();
        let mut list = null_mut();
        let n_analyzed =
            unsafe { ffi::Hunspell_analyze(self.handle, &mut analyzed, word1.as_ptr()) };
        let n = unsafe {
            ffi::Hunspell_generate2(self.handle, &mut list, word2.as_ptr(), analyzed, n_analyzed)
        };
        let strings = list_to_vec(list, n)?;
        unsafe {
            ffi::Hunspell_free_list(self.handle, &mut analyzed, n_analyzed);
            // ffi::Hunspell_free_list(self.handle, &mut list, n);
        }
        Ok(strings)
    }
}

impl Clone for SpellChecker {
    /// **Panics** if the files that the `SpellChecker` was created from
    /// no longer exist.
    fn clone(&self) -> Self {
        let mut clone = match &self.key {
            Some(key) => Self::new_with_key(&self.affix, &self.dictionary, key)
                .expect(&format!("Affix file '{:?}' no longer exists", &self.affix)),
            None => Self::new(&self.affix, &self.dictionary).expect(&format!(
                "Dictionary file '{:?}' no longer exists",
                &self.dictionary
            )),
        };
        for d in &self.additional_dictionaries {
            clone.add_dictionary(d).expect(&format!(
                "Additional dictionary file '{:?}' no longer exists",
                d
            ));
        }
        clone
    }
}

impl Drop for SpellChecker {
    fn drop(&mut self) {
        unsafe {
            ffi::Hunspell_destroy(self.handle);
        }
    }
}

pub(crate) fn check_paths<P: AsRef<Path>>(affix: P, dictionary: P) -> Result<(PathBuf, PathBuf)> {
    let affix = affix.as_ref().to_path_buf();
    let dictionary = dictionary.as_ref().to_path_buf();
    if !affix.is_file() {
        return Err(Error::AffixFileIsNoFile(
            affix.to_string_lossy().into_owned(),
        ));
    }
    if !dictionary.is_file() {
        return Err(Error::DictionaryFileIsNoFile(
            dictionary.to_string_lossy().into_owned(),
        ));
    }
    Ok((affix, dictionary))
}

pub(crate) fn list_to_vec(ptr: *mut *mut u8, len: i32) -> Result<Vec<String>> {
    if ptr.is_null() {
        return Err(Error::NullPtr);
    }
    if len < 0 {
        return Err(Error::NegativeListLength(len));
    }
    if len == 0 {
        return Ok(Vec::new());
    }

    // SAFETY:
    //    - checked for null ptr, other issues depend on the hunspell library
    //    - len has been checked: safe cast
    unsafe { Vec::from_raw_parts(ptr, len as usize, len as usize) }
        .into_iter()
        .map(|p| {
            if p.is_null() {
                Err(Error::NullPtr)
            } else {
                // SAFETY: checked for null ptr, other issues depend on the hunspell library
                unsafe { CStr::from_ptr(p) }
                    .to_str()
                    .and_then(|s| Ok(s.to_string()))
                    .map_err(|e| e.into())
            }
        })
        .collect()
}
