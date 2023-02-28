wit_bindgen_rust::export!("extension.wit");

use fastbloom_rs::{BloomFilter, FilterBuilder, Membership};

struct Extension;

/// The number of hash functions to use in the bloom filter
/// See: https://hur.st/bloomfilter/?n=1000000&p=0.01&m=&k=100
const NUM_HASHES: u32 = 100;

/// The size of the bloom filter in bytes
/// See: https://hur.st/bloomfilter/?n=1000000&p=0.01&m=&k=100
const BLOOM_FILTER_SIZE: u64 = 4 * 1024 * 1024;

#[inline]
fn filter_from(handle: i32) -> &'static mut BloomFilter {
    let ptr = handle as *mut BloomFilter;
    unsafe { &mut *ptr }
}

#[inline]
fn with_filter(handle: i32, f: impl FnOnce(&mut BloomFilter)) -> i32 {
    let filter = filter_from(handle);
    f(filter);
    handle as *mut BloomFilter as i32
}

#[inline]
fn drop_filter(handle: i32) {
    drop(unsafe { std::ptr::read(handle as *mut BloomFilter) });
}

impl extension::Extension for Extension {
    /// Initializes a bloom filter with a predefined set of parameters
    fn bloom_init_handle() -> i32 {
        let filter =
            FilterBuilder::from_size_and_hashes(BLOOM_FILTER_SIZE, NUM_HASHES).build_bloom_filter();
        Box::leak(Box::new(filter)) as *const BloomFilter as i32
    }

    /// Adds a value to the bloom filter
    fn bloom_update_handle(handle: i32, value: String) -> i32 {
        with_filter(handle, |filter| {
            filter.add(value.as_bytes());
        })
    }

    /// Merges two bloom filters
    /// Returns the handle to the merged filter
    fn bloom_merge_handle(left: i32, right: i32) -> i32 {
        with_filter(left, |left| {
            with_filter(right, |right| {
                left.intersect(right);
            });
            drop_filter(right);
        })
    }

    /// Serializes a bloom filter
    fn bloom_serialize_handle(handle: i32) -> Vec<u8> {
        let blob = {
            let filter = filter_from(handle);
            filter.get_u8_array().to_vec()
        };
        drop_filter(handle);
        blob
    }

    fn bloom_deserialize_handle(blob: Vec<u8>) -> i32 {
        let filter = BloomFilter::from_u8_array(blob.as_slice(), NUM_HASHES);
        Box::leak(Box::new(filter)) as *const BloomFilter as i32
    }

    fn bloom_maybe_exists(blob: Vec<u8>, value: String) -> bool {
        BloomFilter::from_u8_array(blob.as_slice(), NUM_HASHES).contains(value.as_bytes())
    }
}
