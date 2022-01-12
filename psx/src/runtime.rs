use core::mem::{size_of, transmute};

/// Define a constructor that runs before main.
#[macro_export]
macro_rules! ctor {
    (fn $name:ident() { $($body:tt)* }) => {
        #[used]
        #[link_section = ".ctors"]
        static $name: fn() = || {
            $($body)*
        };
    };
}

#[cfg(feature = "loadable_app")]
type RtReturn = ();
#[cfg(not(feature = "loadable_app"))]
type RtReturn = !;

/// The runtime used by the default linker scripts.
#[no_mangle]
extern "C" fn _start() -> RtReturn {
    // SAFETY: If there is no unmangled function named `main` this causes an error
    // at link-time.
    unsafe {
        #[cfg(not(test))]
        extern "Rust" {
            fn main() -> Result<(), &'static str>;
        }
        extern "C" {
            static __ctors_start: usize;
            static __ctors_end: usize;
        }
        let ptr_size = size_of::<usize>();
        let end = &__ctors_end as *const usize as usize;
        let start = &__ctors_start as *const usize as usize;
        let ctors_range = end - start;
        assert!(
            (ctors_range % 4) == 0,
            ".ctors section is not 4-byte aligned"
        );
        let num_ctors = ctors_range / ptr_size;
        for n in 0..num_ctors {
            let ptr = __ctors_start + (n * ptr_size);
            let ctor = transmute::<usize, fn()>(ptr);
            ctor();
        }
        #[cfg(not(test))]
        main().unwrap();

        #[cfg(test)]
        crate::main();
    }
    #[cfg(not(feature = "loadable_app"))]
    panic!("`main` should not return")
}

// Define string-literals to embed in PSEXE header
// Using the same identifier for all regions conveniently makes the crate
// features mutually exclusive
macro_rules! as_array {
    ($msg:literal) => {
        // SAFETY: This dereferences a pointer to a literal which has a static lifetime.
        unsafe { *($msg.as_ptr() as *const _) }
    };
}

#[cfg(any(feature = "NA_region", test))]
#[used]
#[no_mangle]
#[doc(hidden)]
#[link_section = ".region"]
pub static _REGION: [u8; 55] = as_array!("Sony Computer Entertainment Inc. for North America area");

#[cfg(feature = "EU_region")]
#[used]
#[no_mangle]
#[doc(hidden)]
#[link_section = ".region"]
pub static _REGION: [u8; 48] = as_array!("Sony Computer Entertainment Inc. for Europe area");

#[cfg(feature = "J_region")]
#[used]
#[no_mangle]
#[doc(hidden)]
#[link_section = ".region"]
pub static _REGION: [u8; 47] = as_array!("Sony Computer Entertainment Inc. for Japan area");

#[used]
#[no_mangle]
#[doc(hidden)]
#[link_section = ".psx_exe"]
pub static _PSX_EXE: [u8; 8] = as_array!("PS-X EXE");
