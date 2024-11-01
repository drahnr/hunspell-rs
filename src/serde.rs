use serde::{
    de::{Error, MapAccess, SeqAccess, Visitor},
    Deserialize, Deserializer,
};
use std::path::PathBuf;

use crate::SpellChecker;

impl<'de> Deserialize<'de> for SpellChecker {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "lowercase")]
        enum Field {
            Affix,
            Dictionary,
            AdditionalDictionaries,
            Key,
        }

        struct SpellCheckerVisitor;

        impl<'de> Visitor<'de> for SpellCheckerVisitor {
            type Value = SpellChecker;
            fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
                formatter.write_str("struct SpellChecker")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<SpellChecker, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let affix: PathBuf = seq
                    .next_element()?
                    .ok_or_else(|| Error::invalid_length(0, &self))?;
                let dictionary: PathBuf = seq
                    .next_element()?
                    .ok_or_else(|| Error::invalid_length(1, &self))?;
                let additional_dictionaries: Vec<PathBuf> = seq
                    .next_element()?
                    .ok_or_else(|| Error::invalid_length(2, &self))?;
                let key: Option<String> = seq
                    .next_element()?
                    .ok_or_else(|| Error::invalid_length(3, &self))?;
                let mut new_dictionary = match key {
                    Some(k) => SpellChecker::new_with_key(&affix, &dictionary, k)
                        .map_err(|e| Error::custom(e))?,
                    _ => SpellChecker::new(&affix, &dictionary).map_err(|e| Error::custom(e))?,
                };
                for d in additional_dictionaries {
                    new_dictionary
                        .add_dictionary(d)
                        .map_err(|e| Error::custom(e))?;
                }
                Ok(new_dictionary)
            }

            fn visit_map<V>(self, mut map: V) -> Result<SpellChecker, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut affix = None;
                let mut dictionary = None;
                let mut additional_dictionaries = None;
                let mut key = None;
                while let Some(mkey) = map.next_key()? {
                    match mkey {
                        Field::Affix => {
                            if affix.is_some() {
                                return Err(Error::duplicate_field("affix"));
                            }
                            affix = Some(map.next_value()?);
                        }
                        Field::Dictionary => {
                            if dictionary.is_some() {
                                return Err(Error::duplicate_field("dictionary"));
                            }
                            dictionary = Some(map.next_value()?);
                        }
                        Field::AdditionalDictionaries => {
                            if additional_dictionaries.is_some() {
                                return Err(Error::duplicate_field("additional_dictionaries"));
                            }
                            additional_dictionaries = Some(map.next_value()?);
                        }
                        Field::Key => {
                            if key.is_some() {
                                return Err(Error::duplicate_field("key"));
                            }
                            key = Some(map.next_value()?);
                        }
                    }
                }
                let affix: PathBuf = affix.ok_or_else(|| Error::missing_field("affix"))?;
                let dictionary: PathBuf =
                    dictionary.ok_or_else(|| Error::missing_field("dictionary"))?;
                let additional_dictionaries: Vec<PathBuf> = additional_dictionaries
                    .ok_or_else(|| Error::missing_field("additional_dictionaries"))?;
                let key: Option<String> = key.ok_or_else(|| Error::missing_field("key"))?;

                let mut new_dictionary = match key {
                    Some(k) => SpellChecker::new_with_key(affix, dictionary, k)
                        .map_err(|e| Error::custom(e))?,
                    _ => SpellChecker::new(affix, dictionary).map_err(|e| Error::custom(e))?,
                };
                for d in additional_dictionaries {
                    new_dictionary
                        .add_dictionary(d)
                        .map_err(|e| Error::custom(e))?;
                }
                Ok(new_dictionary)
            }
        }
        const FIELDS: &'static [&'static str] =
            &["affix", "dictionary", "additional_dictionaries", "key"];
        deserializer.deserialize_struct("SpellChecker", FIELDS, SpellCheckerVisitor)
    }
}
