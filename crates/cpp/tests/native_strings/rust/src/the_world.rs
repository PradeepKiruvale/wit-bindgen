// Generated by `wit-bindgen` 0.22.0. DO NOT EDIT!
// Options used:
pub mod foo {
  pub mod foo {
    #[allow(clippy::all)]
    pub mod strings {
      #[used]
      #[doc(hidden)]
      #[cfg(target_arch = "wasm32")]
      static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
      use super::super::super::_rt;
      #[allow(unused_unsafe, clippy::all)]
      pub fn a(x: &str,){
        unsafe {
          let vec0 = x;
          let ptr0 = vec0.as_ptr().cast::<u8>();
          let len0 = vec0.len();

          #[link(wasm_import_module = "foo:foo/strings")]
          extern "C" {
            #[cfg_attr(target_arch = "wasm32", link_name = "a")]
            fn fooX3AfooX2FstringsX00a(_: *mut u8, _: usize, );
          }
          fooX3AfooX2FstringsX00a(ptr0.cast_mut(), len0);
        }
      }
      #[allow(unused_unsafe, clippy::all)]
      pub fn b() -> _rt::String{
        unsafe {
          #[repr(align(8))]
          struct RetArea([::core::mem::MaybeUninit::<u8>; 16]);
          let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 16]);
          let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
          #[link(wasm_import_module = "foo:foo/strings")]
          extern "C" {
            #[cfg_attr(target_arch = "wasm32", link_name = "b")]
            fn fooX3AfooX2FstringsX00b(_: *mut u8, );
          }
          fooX3AfooX2FstringsX00b(ptr0);
          let l1 = *ptr0.add(0).cast::<*mut u8>();
          let l2 = *ptr0.add(8).cast::<usize>();
          let len3 = l2;
          let bytes3 = _rt::Vec::from_raw_parts(l1.cast(), len3, len3);
          _rt::string_lift(bytes3)
        }
      }
      #[allow(unused_unsafe, clippy::all)]
      pub fn c(a: &str,b: &str,) -> _rt::String{
        unsafe {
          #[repr(align(8))]
          struct RetArea([::core::mem::MaybeUninit::<u8>; 16]);
          let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 16]);
          let vec0 = a;
          let ptr0 = vec0.as_ptr().cast::<u8>();
          let len0 = vec0.len();
          let vec1 = b;
          let ptr1 = vec1.as_ptr().cast::<u8>();
          let len1 = vec1.len();
          let ptr2 = ret_area.0.as_mut_ptr().cast::<u8>();
          #[link(wasm_import_module = "foo:foo/strings")]
          extern "C" {
            #[cfg_attr(target_arch = "wasm32", link_name = "c")]
            fn fooX3AfooX2FstringsX00c(_: *mut u8, _: usize, _: *mut u8, _: usize, _: *mut u8, );
          }
          fooX3AfooX2FstringsX00c(ptr0.cast_mut(), len0, ptr1.cast_mut(), len1, ptr2);
          let l3 = *ptr2.add(0).cast::<*mut u8>();
          let l4 = *ptr2.add(8).cast::<usize>();
          let len5 = l4;
          let bytes5 = _rt::Vec::from_raw_parts(l3.cast(), len5, len5);
          _rt::string_lift(bytes5)
        }
      }

    }

  }
}
pub mod exports {
  pub mod foo {
    pub mod foo {
      #[allow(clippy::all)]
      pub mod strings {
        #[used]
        #[doc(hidden)]
        #[cfg(target_arch = "wasm32")]
        static __FORCE_SECTION_REF: fn() = super::super::super::super::__link_custom_section_describing_imports;
        use super::super::super::super::_rt;
        #[doc(hidden)]
        #[allow(non_snake_case)]
        pub unsafe fn _export_a_cabi<T: Guest>(arg0: *mut u8,arg1: usize,) {let len0 = arg1;
        let bytes0 = _rt::Vec::from_raw_parts(arg0.cast(), len0, len0);
        T::a(_rt::string_lift(bytes0));
      }
      #[doc(hidden)]
      #[allow(non_snake_case)]
      pub unsafe fn _export_b_cabi<T: Guest>() -> *mut u8 {let result0 = T::b();
      let ptr1 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
      let vec2 = (result0.into_bytes()).into_boxed_slice();
      let ptr2 = vec2.as_ptr().cast::<u8>();
      let len2 = vec2.len();
      ::core::mem::forget(vec2);
      *ptr1.add(8).cast::<usize>() = len2;
      *ptr1.add(0).cast::<*mut u8>() = ptr2.cast_mut();
      ptr1
    }
    #[doc(hidden)]
    #[allow(non_snake_case)]
    pub unsafe fn __post_return_b<T: Guest>(arg0: *mut u8,) {
      let l0 = *arg0.add(0).cast::<*mut u8>();
      let l1 = *arg0.add(8).cast::<usize>();
      _rt::cabi_dealloc(l0, l1, 1);
    }
    #[doc(hidden)]
    #[allow(non_snake_case)]
    pub unsafe fn _export_c_cabi<T: Guest>(arg0: *mut u8,arg1: usize,arg2: *mut u8,arg3: usize,) -> *mut u8 {let len0 = arg1;
    let bytes0 = _rt::Vec::from_raw_parts(arg0.cast(), len0, len0);
    let len1 = arg3;
    let bytes1 = _rt::Vec::from_raw_parts(arg2.cast(), len1, len1);
    let result2 = T::c(_rt::string_lift(bytes0), _rt::string_lift(bytes1));
    let ptr3 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
    let vec4 = (result2.into_bytes()).into_boxed_slice();
    let ptr4 = vec4.as_ptr().cast::<u8>();
    let len4 = vec4.len();
    ::core::mem::forget(vec4);
    *ptr3.add(8).cast::<usize>() = len4;
    *ptr3.add(0).cast::<*mut u8>() = ptr4.cast_mut();
    ptr3
  }
  #[doc(hidden)]
  #[allow(non_snake_case)]
  pub unsafe fn __post_return_c<T: Guest>(arg0: *mut u8,) {
    let l0 = *arg0.add(0).cast::<*mut u8>();
    let l1 = *arg0.add(8).cast::<usize>();
    _rt::cabi_dealloc(l0, l1, 1);
  }
  pub trait Guest {
    fn a(x: _rt::String,);
    fn b() -> _rt::String;
    fn c(a: _rt::String,b: _rt::String,) -> _rt::String;
  }
  #[doc(hidden)]

  macro_rules! __export_foo_foo_strings_cabi{
    ($ty:ident with_types_in $($path_to_types:tt)*) => (const _: () = {

      #[cfg_attr(target_arch = "wasm32", export_name = "foo:foo/strings#a")]
      #[cfg_attr(not(target_arch = "wasm32"), no_mangle)]
      unsafe extern "C" fn fooX3AfooX2FstringsX23a(arg0: *mut u8,arg1: usize,) {
        $($path_to_types)*::_export_a_cabi::<$ty>(arg0, arg1)
      }
      #[cfg_attr(target_arch = "wasm32", export_name = "foo:foo/strings#b")]
      #[cfg_attr(not(target_arch = "wasm32"), no_mangle)]
      unsafe extern "C" fn fooX3AfooX2FstringsX23b() -> *mut u8 {
        $($path_to_types)*::_export_b_cabi::<$ty>()
      }
      #[cfg_attr(target_arch = "wasm32", export_name = "cabi_post_foo:foo/strings#b")]
      #[cfg_attr(not(target_arch = "wasm32"), no_mangle)]
      unsafe extern "C" fn cabi_post_fooX3AfooX2FstringsX23b(arg0: *mut u8,) {
        $($path_to_types)*::__post_return_b::<$ty>(arg0)
      }
      #[cfg_attr(target_arch = "wasm32", export_name = "foo:foo/strings#c")]
      #[cfg_attr(not(target_arch = "wasm32"), no_mangle)]
      unsafe extern "C" fn fooX3AfooX2FstringsX23c(arg0: *mut u8,arg1: usize,arg2: *mut u8,arg3: usize,) -> *mut u8 {
        $($path_to_types)*::_export_c_cabi::<$ty>(arg0, arg1, arg2, arg3)
      }
      #[cfg_attr(target_arch = "wasm32", export_name = "cabi_post_foo:foo/strings#c")]
      #[cfg_attr(not(target_arch = "wasm32"), no_mangle)]
      unsafe extern "C" fn cabi_post_fooX3AfooX2FstringsX23c(arg0: *mut u8,) {
        $($path_to_types)*::__post_return_c::<$ty>(arg0)
      }
    };);
  }
  #[doc(hidden)]
  pub(crate) use __export_foo_foo_strings_cabi;
  #[repr(align(8))]
  struct _RetArea([::core::mem::MaybeUninit::<u8>; 16]);
  static mut _RET_AREA: _RetArea = _RetArea([::core::mem::MaybeUninit::uninit(); 16]);

}

}
}
}
mod _rt {
  pub use alloc_crate::string::String;
  pub use alloc_crate::vec::Vec;
  pub unsafe fn string_lift(bytes: Vec<u8>) -> String {
    if cfg!(debug_assertions) {
      String::from_utf8(bytes).unwrap()
    } else {
      String::from_utf8_unchecked(bytes)
    }
  }
  pub unsafe fn cabi_dealloc(ptr: *mut u8, size: usize, align: usize) {
    if size == 0 {
      return;
    }
    let layout = alloc::Layout::from_size_align_unchecked(size, align);
    alloc::dealloc(ptr as *mut u8, layout);
  }
  extern crate alloc as alloc_crate;
  pub use alloc_crate::alloc;
}

/// Generates `#[no_mangle]` functions to export the specified type as the
/// root implementation of all generated traits.
///
/// For more information see the documentation of `wit_bindgen::generate!`.
///
/// ```rust
/// # macro_rules! export{ ($($t:tt)*) => (); }
/// # trait Guest {}
/// struct MyType;
///
/// impl Guest for MyType {
///     // ...
/// }
///
/// export!(MyType);
/// ```
#[allow(unused_macros)]
#[doc(hidden)]

macro_rules! __export_the_world_impl {
  ($ty:ident) => (self::export!($ty with_types_in self););
  ($ty:ident with_types_in $($path_to_types_root:tt)*) => (
  $($path_to_types_root)*::exports::foo::foo::strings::__export_foo_foo_strings_cabi!($ty with_types_in $($path_to_types_root)*::exports::foo::foo::strings);
  )
}
#[doc(inline)]
pub(crate) use __export_the_world_impl as export;

#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:wit-bindgen:0.22.0:the-world:encoded world"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 286] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07\x9e\x01\x01A\x02\x01\
A\x04\x01B\x06\x01@\x01\x01xs\x01\0\x04\0\x01a\x01\0\x01@\0\0s\x04\0\x01b\x01\x01\
\x01@\x02\x01as\x01bs\0s\x04\0\x01c\x01\x02\x03\x01\x0ffoo:foo/strings\x05\0\x01\
B\x06\x01@\x01\x01xs\x01\0\x04\0\x01a\x01\0\x01@\0\0s\x04\0\x01b\x01\x01\x01@\x02\
\x01as\x01bs\0s\x04\0\x01c\x01\x02\x04\x01\x0ffoo:foo/strings\x05\x01\x04\x01\x11\
foo:foo/the-world\x04\0\x0b\x0f\x01\0\x09the-world\x03\0\0\0G\x09producers\x01\x0c\
processed-by\x02\x0dwit-component\x070.201.0\x10wit-bindgen-rust\x060.22.0";

#[inline(never)]
#[doc(hidden)]
#[cfg(target_arch = "wasm32")]
pub fn __link_custom_section_describing_imports() {
  wit_bindgen::rt::maybe_link_cabi_realloc();
}

