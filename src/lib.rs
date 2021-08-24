use wasm_bindgen::prelude::*;
use bloomfilter::Bloom;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct BloomFilterSerialData {
    bitmap: Vec<u8>,
    bitmap_bits: u64,
    k_num: u32, //number of hash functions
    sip_keys: [(u64, u64); 2]
}

#[wasm_bindgen]
pub struct BloomFilter {
    bloom: Bloom<String>
}

#[wasm_bindgen]
impl BloomFilter {
    #[wasm_bindgen(constructor)]
    pub fn new(number_of_bytes: usize, items_count_estimate: usize) -> BloomFilter {
        return BloomFilter {
            bloom: Bloom::<String>::new(number_of_bytes, items_count_estimate)
        }
    }
    #[wasm_bindgen]
    pub fn add(&mut self, item_to_add: String) {
        self.bloom.set(&item_to_add)
    }
    #[wasm_bindgen]
    pub fn test(&mut self, item_to_test: String) -> bool {
        return self.bloom.check(&item_to_test)
    }
    #[wasm_bindgen]
    pub fn serialise(&self) -> String {
        let serial_data = BloomFilterSerialData {
            bitmap: self.bloom.bitmap(),
            bitmap_bits: self.bloom.number_of_bits(),
            k_num: self.bloom.number_of_hash_functions(), //number of hash functions
            sip_keys: self.bloom.sip_keys()
        };
        return serde_json::to_string(&serial_data).unwrap().into()
    }
    #[wasm_bindgen]
    pub fn deserialise(serial_bloomfilter: String) -> BloomFilter {
        let bloom_filter_data: BloomFilterSerialData = serde_json::from_str(&serial_bloomfilter).unwrap();
        return BloomFilter {
            bloom: Bloom::<String>::from_existing(
                &bloom_filter_data.bitmap, 
                bloom_filter_data.bitmap_bits,
                bloom_filter_data.k_num, //number of hash functions
                bloom_filter_data.sip_keys
            )
        }
    }
}