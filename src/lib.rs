mod plugin;

use std::alloc::Layout;
use std::cell::RefCell;

thread_local! {
    // Holds the output until the host reads it after process() returns.
    static OUTPUT_BUF: RefCell<Vec<u8>> = RefCell::new(Vec::new());
}

/// Allocate `size` bytes for the host to write input JSON into.
/// Returns the pointer into WASM linear memory.
#[no_mangle]
pub extern "C" fn alloc(size: i32) -> i32 {
    let layout = Layout::from_size_align(size as usize, 1).expect("alloc layout");
    // SAFETY: layout has non-zero size (host always passes len > 0).
    unsafe { std::alloc::alloc(layout) as i32 }
}

/// Process the input JSON written at `ptr`/`len` and return the packed output.
/// Return value encodes `(output_ptr as i64) << 32 | output_len as i64`.
/// Returns 0 on error.
#[no_mangle]
pub extern "C" fn process(ptr: i32, len: i32) -> i64 {
    // SAFETY: host wrote `len` bytes at `ptr` after calling alloc().
    let input = unsafe { std::slice::from_raw_parts(ptr as *const u8, len as usize) };

    let output_bytes = match plugin::run(input) {
        Ok(json) => json.into_bytes(),
        Err(_) => return 0,
    };

    OUTPUT_BUF.with(|buf| {
        let mut buf = buf.borrow_mut();
        *buf = output_bytes;
        ((buf.as_ptr() as i64) << 32) | (buf.len() as i64)
    })
}
