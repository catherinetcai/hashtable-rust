pub struct HTItem {
    key: char,
    value: char,
}

pub struct HTHashTable {
    size: i16,
    count: i16,
}

pub fn ht_new_item(k: char, v: char) -> HTItem {
    HTItem{ key: k, value: v }
}

pub fn ht_new() -> HTHashTable {
    HTHashTable{ size: 53, count: 0 }
}
