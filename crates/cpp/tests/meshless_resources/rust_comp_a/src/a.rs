// Generated by `wit-bindgen` 0.28.0. DO NOT EDIT!
// Options used:
#[allow(dead_code)]
pub mod foo {
  #[allow(dead_code)]
  pub mod foo {
    #[allow(dead_code, clippy::all)]
    pub mod resources {
      #[used]
      #[doc(hidden)]
      #[cfg(target_arch = "wasm32")]
      static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
      use super::super::super::_rt;

      #[derive(Debug)]
      #[repr(transparent)]
      pub struct R{
        handle: _rt::Resource<R>,
      }

      impl R{
        #[doc(hidden)]
        pub unsafe fn from_handle(handle: usize) -> Self {
          Self {
            handle: _rt::Resource::from_handle(handle),
          }
        }

        #[doc(hidden)]
        pub fn take_handle(&self) -> usize{
          _rt::Resource::take_handle(&self.handle)
        }

        #[doc(hidden)]
        pub fn handle(&self) -> usize{
          _rt::Resource::handle(&self.handle)
        }
      }


      unsafe impl _rt::WasmResource for R{
        #[inline]
        unsafe fn drop(_handle: usize) {
          {
            #[link(wasm_import_module = "foo:foo/resources")]
            extern "C" {
              #[cfg_attr(target_arch = "wasm32", link_name = "[resource-drop]r")]
              fn fooX3AfooX2FresourcesX00X5Bresource_dropX5Dr(_: usize);
            }

            fooX3AfooX2FresourcesX00X5Bresource_dropX5Dr(_handle);
          }
        }
      }

      impl R {
        #[allow(unused_unsafe, clippy::all)]
        pub fn new(a: u32,) -> Self{
          unsafe {

            #[link(wasm_import_module = "foo:foo/resources")]
            extern "C" {
              #[cfg_attr(target_arch = "wasm32", link_name = "[constructor]r")]
              fn fooX3AfooX2FresourcesX00X5BconstructorX5Dr(_: i32, ) -> *mut u8;
            }
            let ret = fooX3AfooX2FresourcesX00X5BconstructorX5Dr(_rt::as_i32(&a));
            R::from_handle(ret as usize)
          }
        }
      }
      impl R {
        #[allow(unused_unsafe, clippy::all)]
        pub fn add(&self,b: u32,) -> (){
          unsafe {

            #[link(wasm_import_module = "foo:foo/resources")]
            extern "C" {
              #[cfg_attr(target_arch = "wasm32", link_name = "[method]r.add")]
              fn fooX3AfooX2FresourcesX00X5BmethodX5DrX2Eadd(_: *mut u8, _: i32, );
            }
            fooX3AfooX2FresourcesX00X5BmethodX5DrX2Eadd((self).handle() as *mut u8, _rt::as_i32(&b));
          }
        }
      }
      #[allow(unused_unsafe, clippy::all)]
      pub fn create() -> R{
        unsafe {

          #[link(wasm_import_module = "foo:foo/resources")]
          extern "C" {
            #[cfg_attr(target_arch = "wasm32", link_name = "create")]
            fn fooX3AfooX2FresourcesX00create() -> *mut u8;
          }
          let ret = fooX3AfooX2FresourcesX00create();
          R::from_handle(ret as usize)
        }
      }
      #[allow(unused_unsafe, clippy::all)]
      /// borrows: func(o: borrow<r>);
      pub fn consume(o: R,) -> (){
        unsafe {

          #[link(wasm_import_module = "foo:foo/resources")]
          extern "C" {
            #[cfg_attr(target_arch = "wasm32", link_name = "consume")]
            fn fooX3AfooX2FresourcesX00consume(_: *mut u8, );
          }
          fooX3AfooX2FresourcesX00consume((&o).take_handle() as *mut u8);
        }
      }

    }

  }
}
mod _rt {


  use core::fmt;
  use core::marker;
  use core::sync::atomic::{AtomicUsize, Ordering::Relaxed};

  /// A type which represents a component model resource, either imported or
  /// exported into this component.
  ///
  /// This is a low-level wrapper which handles the lifetime of the resource
  /// (namely this has a destructor). The `T` provided defines the component model
  /// intrinsics that this wrapper uses.
  ///
  /// One of the chief purposes of this type is to provide `Deref` implementations
  /// to access the underlying data when it is owned.
  ///
  /// This type is primarily used in generated code for exported and imported
  /// resources.
  #[repr(transparent)]
  pub struct Resource<T: WasmResource> {
    // NB: This would ideally be `usize` but it is not. The fact that this has
    // interior mutability is not exposed in the API of this type except for the
    // `take_handle` method which is supposed to in theory be private.
    //
    // This represents, almost all the time, a valid handle value. When it's
    // invalid it's stored as `0`.
    handle: AtomicUsize,
    _marker: marker::PhantomData<T>,
  }

  /// A trait which all wasm resources implement, namely providing the ability to
  /// drop a resource.
  ///
  /// This generally is implemented by generated code, not user-facing code.
  #[allow(clippy::missing_safety_doc)]
  pub unsafe trait WasmResource {
    /// Invokes the `[resource-drop]...` intrinsic.
    unsafe fn drop(handle: usize);
  }

  impl<T: WasmResource> Resource<T> {
    #[doc(hidden)]
    pub unsafe fn from_handle(handle: usize) -> Self {
      debug_assert!(handle != 0);
      Self {
        handle: AtomicUsize::new(handle),
        _marker: marker::PhantomData,
      }
    }

    /// Takes ownership of the handle owned by `resource`.
    ///
    /// Note that this ideally would be `into_handle` taking `Resource<T>` by
    /// ownership. The code generator does not enable that in all situations,
    /// unfortunately, so this is provided instead.
    ///
    /// Also note that `take_handle` is in theory only ever called on values
    /// owned by a generated function. For example a generated function might
    /// take `Resource<T>` as an argument but then call `take_handle` on a
    /// reference to that argument. In that sense the dynamic nature of
    /// `take_handle` should only be exposed internally to generated code, not
    /// to user code.
    #[doc(hidden)]
    pub fn take_handle(resource: &Resource<T>) -> usize {
      resource.handle.swap(0, Relaxed)
    }

    #[doc(hidden)]
    pub fn handle(resource: &Resource<T>) -> usize {
      resource.handle.load(Relaxed)
    }
  }

  impl<T: WasmResource> fmt::Debug for Resource<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      f.debug_struct("Resource")
      .field("handle", &self.handle)
      .finish()
    }
  }

  impl<T: WasmResource> Drop for Resource<T> {
    fn drop(&mut self) {
      unsafe {
        match self.handle.load(Relaxed) {
          // If this handle was "taken" then don't do anything in the
          // destructor.
          0 => {}

          // ... but otherwise do actually destroy it with the imported
          // component model intrinsic as defined through `T`.
          other => T::drop(other),
        }
      }
    }
  }
  
  pub fn as_i32<T: AsI32>(t: T) -> i32 {
    t.as_i32()
  }

  pub trait AsI32 {
    fn as_i32(self) -> i32;
  }

  impl<'a, T: Copy + AsI32> AsI32 for &'a T {
    fn as_i32(self) -> i32 {
      (*self).as_i32()
    }
  }
  
  impl AsI32 for i32 {
    #[inline]
    fn as_i32(self) -> i32 {
      self as i32
    }
  }
  
  impl AsI32 for u32 {
    #[inline]
    fn as_i32(self) -> i32 {
      self as i32
    }
  }
  
  impl AsI32 for i16 {
    #[inline]
    fn as_i32(self) -> i32 {
      self as i32
    }
  }
  
  impl AsI32 for u16 {
    #[inline]
    fn as_i32(self) -> i32 {
      self as i32
    }
  }
  
  impl AsI32 for i8 {
    #[inline]
    fn as_i32(self) -> i32 {
      self as i32
    }
  }
  
  impl AsI32 for u8 {
    #[inline]
    fn as_i32(self) -> i32 {
      self as i32
    }
  }
  
  impl AsI32 for char {
    #[inline]
    fn as_i32(self) -> i32 {
      self as i32
    }
  }
  
  impl AsI32 for usize {
    #[inline]
    fn as_i32(self) -> i32 {
      self as i32
    }
  }
}

#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:wit-bindgen:0.28.0:a:encoded world"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 272] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07\x98\x01\x01A\x02\x01\
A\x02\x01B\x0b\x04\0\x01r\x03\x01\x01i\0\x01@\x01\x01ay\0\x01\x04\0\x0e[construc\
tor]r\x01\x02\x01h\0\x01@\x02\x04self\x03\x01by\x01\0\x04\0\x0d[method]r.add\x01\
\x04\x01@\0\0\x01\x04\0\x06create\x01\x05\x01@\x01\x01o\x01\x01\0\x04\0\x07consu\
me\x01\x06\x03\x01\x11foo:foo/resources\x05\0\x04\x01\x09foo:foo/a\x04\0\x0b\x07\
\x01\0\x01a\x03\0\0\0G\x09producers\x01\x0cprocessed-by\x02\x0dwit-component\x07\
0.214.0\x10wit-bindgen-rust\x060.28.0";

#[inline(never)]
#[doc(hidden)]
#[cfg(target_arch = "wasm32")]
pub fn __link_custom_section_describing_imports() {
  wit_bindgen::rt::maybe_link_cabi_realloc();
}

