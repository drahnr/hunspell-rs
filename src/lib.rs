use std::ffi::CString;

#[test]
fn spell_check() {
    unsafe {
        let affpath = CString::new("/usr/share/hunspell/en_US.aff").unwrap();
        let dicpath = CString::new("/usr/share/hunspell/en_US.dic").unwrap();
        let good_word = CString::new("programming").unwrap();
        let bad_word = CString::new("progrmaing").unwrap();
        let spell = Hunspell_create(affpath.as_ptr(), dicpath.as_ptr());

        assert!(Hunspell_spell(spell, good_word.as_ptr()) == 1);
        assert!(Hunspell_spell(spell, bad_word.as_ptr()) == 0);

        Hunspell_destroy(spell);
    }
}

enum Hunhandle {}

#[link(name = "hunspell")]
extern {
    fn Hunspell_create(affpath: *const i8, dpath: *const i8) -> *mut Hunhandle;
    fn Hunspell_create_key(affpath: *const i8, dpath: *const i8, key: *const i8) -> *mut Hunhandle;
    fn Hunspell_destroy(pHunspell: *mut Hunhandle);

    fn Hunspell_spell(pHunspell: *mut Hunhandle, word: *const i8) -> i32;
    fn Hunspell_get_dic_encoding(pHunspell: *mut Hunhandle) -> *mut i8;

    fn Hunspell_suggest(pHunspell: *mut Hunhandle, slst: *mut *mut *mut i8, word: *const i8) -> i32;
    fn Hunspell_analyze(pHunspell: *mut Hunhandle, slst: *mut *mut *mut i8, word: *const i8) -> i32;
    fn Hunspell_stem(pHunspell: *mut Hunhandle, slst: *mut *mut *mut i8, word: *const i8) -> i32;
    fn Hunspell_stem2(pHunspell: *mut Hunhandle, slst: *mut *mut *mut i8, desc: *mut *mut i8, n: i32) -> i32;
    fn Hunspell_generate(pHunspell: *mut Hunhandle, slst: *mut *mut *mut i8, word: *const i8, word2: *const i8) -> i32;
    fn Hunspell_generate2(pHunspell: *mut Hunhandle, slst: *mut *mut *mut i8, word: *const i8, desc: *mut *mut i8, n: i32) -> i32;

    fn Hunspell_add(pHunspell: *mut Hunhandle, word: *const i8) -> i32;
    fn Hunspell_add_with_affix(pHunspell: *mut Hunhandle, word: *const i8, example: *const i8) -> i32;
    fn Hunspell_remove(pHunspell: *mut Hunhandle, slst: *mut *mut *mut i8, n: i32) -> i32;

    fn Hunspell_free_list(pHunspell: *mut Hunhandle, slst: *mut *mut *mut i8, n: i32);
}
