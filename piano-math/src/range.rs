#![allow(dead_code)]

use std::ops::{Range, RangeBounds};

const KEY_CIS: u8 = 1;
const KEY_DIS: u8 = 3;
const KEY_FIS: u8 = 6;
const KEY_GIS: u8 = 8;
const KEY_AIS: u8 = 10;

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct KeyId(u8);

impl KeyId {
    pub fn is_black(&self) -> bool {
        let key = self.0 % 12;
        key == KEY_CIS || key == KEY_DIS || key == KEY_FIS || key == KEY_GIS || key == KEY_AIS
    }
}

/// Describe used slice of piano keyboard
pub struct KeyboardRange {
    range: Range<u8>,

    keys: Vec<KeyId>,
    white_keys: Vec<KeyId>,
    black_keys: Vec<KeyId>,
}

impl KeyboardRange {
    pub fn new<R>(range: R) -> Self
    where
        R: RangeBounds<usize>,
    {
        let mut keys = Vec::new();
        let mut white_keys = Vec::new();
        let mut black_keys = Vec::new();

        let start = range.start_bound();
        let end = range.end_bound();

        let start = match start {
            std::ops::Bound::Included(id) => *id,
            std::ops::Bound::Excluded(id) => *id + 1,
            std::ops::Bound::Unbounded => 0,
        } as u8;

        let end = match end {
            std::ops::Bound::Included(id) => *id + 1,
            std::ops::Bound::Excluded(id) => *id,
            std::ops::Bound::Unbounded => 0,
        } as u8;

        let range = start..end;

        for id in range.clone().map(KeyId) {
            keys.push(id);

            if id.is_black() {
                black_keys.push(id);
            } else {
                white_keys.push(id);
            }
        }

        Self {
            range,

            keys,
            white_keys,
            black_keys,
        }
    }

    pub fn standard_88_keys() -> Self {
        Self::new(21..=108)
    }
}

impl KeyboardRange {
    pub fn contains(&self, item: u8) -> bool {
        self.range.contains(&item)
    }

    pub fn count(&self) -> usize {
        self.keys.len()
    }

    pub fn white_count(&self) -> usize {
        self.white_keys.len()
    }

    pub fn black_count(&self) -> usize {
        self.black_keys.len()
    }

    pub fn iter(&self) -> std::slice::Iter<KeyId> {
        self.keys.iter()
    }

    pub fn white_iter(&self) -> std::slice::Iter<KeyId> {
        self.white_keys.iter()
    }

    pub fn black_iter(&self) -> std::slice::Iter<KeyId> {
        self.black_keys.iter()
    }
}

impl Default for KeyboardRange {
    fn default() -> Self {
        Self::standard_88_keys()
    }
}

#[cfg(test)]
mod tests {}
