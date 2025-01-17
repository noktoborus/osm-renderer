use crate::geodata::reader::OsmEntity;
use crate::geodata::reader::{Multipolygon, Node, OsmArea, Way};
use crate::mapcss::parser::Rule;
use crate::mapcss::parser::Test;
use crate::mapcss::parser::UnaryTestType;
use crate::mapcss::styler::{LabelStyleValue, StyleValue};
use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq)]
struct StyleCacheKey {
    cache_slot: usize,
    tags: Vec<usize>,
    zoom: u8,
}

pub struct StyleCache {
    cache: HashMap<StyleCacheKey, Vec<(StyleValue, LabelStyleValue)>>,
    tag_value_matters: HashMap<String, bool>,
}

impl StyleCache {
    pub fn new(rules: &[Rule]) -> StyleCache {
        let mut tag_value_matters = HashMap::new();

        tag_value_matters.insert("layer".to_string(), true);

        for r in rules.iter() {
            for sel in r.selectors.iter() {
                for test in sel.tests.iter() {
                    let (tag_name, value_matters) = match test {
                        Test::Unary {
                            ref tag_name,
                            ref test_type,
                        } => {
                            let value_matters = !matches!(test_type, UnaryTestType::Exists | UnaryTestType::NotExists);
                            (tag_name, value_matters)
                        }
                        Test::BinaryStringCompare { ref tag_name, .. } => (tag_name, true),
                        Test::BinaryNumericCompare { ref tag_name, .. } => (tag_name, true),
                    };

                    *tag_value_matters.entry(tag_name.clone()).or_default() |= value_matters;
                }
            }
        }

        StyleCache {
            cache: HashMap::default(),
            tag_value_matters,
        }
    }

    pub fn get<'e, E>(&self, entity: &E, zoom: u8) -> Option<Vec<(StyleValue, LabelStyleValue)>>
    where
        E: CacheableEntity + OsmEntity<'e>,
    {
        self.cache.get(&self.to_cache_key(entity, zoom)).cloned()
    }

    pub fn insert<'e, E>(&mut self, entity: &E, zoom: u8, styles: Vec<(StyleValue, LabelStyleValue)>)
    where
        E: CacheableEntity + OsmEntity<'e>,
    {
        self.cache.insert(self.to_cache_key(entity, zoom), styles);
    }

    fn to_cache_key<'e, E>(&self, entity: &E, zoom: u8) -> StyleCacheKey
    where
        E: CacheableEntity + OsmEntity<'e>,
    {
        let mut tags = Vec::new();
        for (k, v) in entity.tags().iter() {
            if let Some(value_matters) = self.tag_value_matters.get(k.str) {
                tags.push(k.offset);
                if *value_matters {
                    tags.push(v.offset);
                }
            }
        }

        StyleCacheKey {
            cache_slot: entity.cache_slot(),
            tags,
            zoom,
        }
    }
}

pub trait CacheableEntity {
    fn cache_slot(&self) -> usize;
}

impl<'a> CacheableEntity for Node<'a> {
    fn cache_slot(&self) -> usize {
        0
    }
}

impl<'a> CacheableEntity for Way<'a> {
    fn cache_slot(&self) -> usize {
        if self.is_closed() {
            1
        } else {
            2
        }
    }
}

impl<'a> CacheableEntity for Multipolygon<'a> {
    fn cache_slot(&self) -> usize {
        3
    }
}
