#[cfg(target_arch = "wasm32")]
type Handle = i32;
#[cfg(not(target_arch = "wasm32"))]
type Handle = i64;

#[cfg(target_arch = "wasm32")]
wit_bindgen_rust::export!("extension.wit");
#[cfg(not(target_arch = "wasm32"))]
mod extension {
    pub type State = i64;
    pub type Blob = Vec<u8>;
    pub type Input = String;
    pub trait Extension {
        fn bloom_init_handle() -> State;
        fn bloom_update_handle(s: State, input: Input) -> State;
        fn bloom_merge_handle(left: State, right: State) -> State;
        fn bloom_serialize_handle(s: State) -> Blob;
        fn bloom_deserialize_handle(filter: Blob) -> State;
        fn bloom_maybe_exists(filter: Blob, input: Input) -> bool;
    }
}

use fastbloom_rs::{BloomFilter, FilterBuilder, Membership};

struct Extension;

/// The number of hash functions to use in the bloom filter
/// See: https://hur.st/bloomfilter/?n=1000000&p=0.01&m=&k=100
const NUM_HASHES: u32 = 100;

/// The size of the bloom filter in bytes
/// See: https://hur.st/bloomfilter/?n=1000000&p=0.01&m=&k=100
const BLOOM_FILTER_SIZE: u64 = 4 * 1024 * 1024;

#[inline]
fn filter_from(handle: Handle) -> &'static mut BloomFilter {
    let ptr = handle as *mut BloomFilter;
    unsafe { &mut *ptr }
}

#[inline]
fn with_filter(handle: Handle, f: impl FnOnce(&mut BloomFilter)) -> Handle {
    let filter = filter_from(handle);
    f(filter);
    handle as *mut BloomFilter as Handle
}

#[inline]
fn drop_filter(handle: Handle) {
    drop(unsafe { std::ptr::read(handle as *mut BloomFilter) });
}

impl extension::Extension for Extension {
    /// Initializes a bloom filter with a predefined set of parameters
    fn bloom_init_handle() -> Handle {
        let filter =
            FilterBuilder::from_size_and_hashes(BLOOM_FILTER_SIZE, NUM_HASHES).build_bloom_filter();
        Box::leak(Box::new(filter)) as *const BloomFilter as Handle
    }

    /// Adds a value to the bloom filter
    fn bloom_update_handle(handle: Handle, value: String) -> Handle {
        with_filter(handle, |filter| {
            filter.add(value.as_bytes());
        })
    }

    /// Merges two bloom filters
    /// Returns the handle to the merged filter
    fn bloom_merge_handle(left: Handle, right: Handle) -> Handle {
        with_filter(left, |left| {
            with_filter(right, |right| {
                left.union(right);
            });
            drop_filter(right);
        })
    }

    /// Serializes a bloom filter
    fn bloom_serialize_handle(handle: Handle) -> Vec<u8> {
        let blob = {
            let filter = filter_from(handle);
            filter.get_u8_array().to_vec()
        };
        drop_filter(handle);
        blob
    }

    fn bloom_deserialize_handle(blob: Vec<u8>) -> Handle {
        let filter = BloomFilter::from_u8_array(blob.as_slice(), NUM_HASHES);
        Box::leak(Box::new(filter)) as *const BloomFilter as Handle
    }

    fn bloom_maybe_exists(blob: Vec<u8>, value: String) -> bool {
        BloomFilter::from_u8_array(blob.as_slice(), NUM_HASHES).contains(value.as_bytes())
    }
}

#[cfg(test)]
mod tests {
    use crate::{with_filter, Extension, extension};

    #[test]
    fn sanity() {
        let h1 = <Extension as extension::Extension>::bloom_init_handle();
        let h1 = <Extension as extension::Extension>::bloom_update_handle(h1, String::from("hello"));
        let h1 = <Extension as extension::Extension>::bloom_update_handle(h1, String::from("world"));
        
        let h2 = <Extension as extension::Extension>::bloom_init_handle();
        let h2 = <Extension as extension::Extension>::bloom_update_handle(h2, String::from("foo"));
        let h2 = <Extension as extension::Extension>::bloom_update_handle(h2, String::from("bar"));

        let m = <Extension as extension::Extension>::bloom_merge_handle(h1, h2);

        let s = <Extension as extension::Extension>::bloom_serialize_handle(m);
        let d = <Extension as extension::Extension>::bloom_deserialize_handle(s);
        /*
        with_filter(d.clone(), |filter| {
            println!("FILTER={:?}", &filter);
        });
        */
        let s = <Extension as extension::Extension>::bloom_serialize_handle(d);

        assert_eq!(<Extension as extension::Extension>::bloom_maybe_exists(s.clone(), String::from("foo")), true);
        assert_eq!(<Extension as extension::Extension>::bloom_maybe_exists(s.clone(), String::from("bar")), true);
        assert_eq!(<Extension as extension::Extension>::bloom_maybe_exists(s.clone(), String::from("hello")), true);
        assert_eq!(<Extension as extension::Extension>::bloom_maybe_exists(s.clone(), String::from("world")), true);
        assert_eq!(<Extension as extension::Extension>::bloom_maybe_exists(s.clone(), String::from("notfound")), false);
    }
}
