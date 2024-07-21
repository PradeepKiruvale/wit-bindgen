// Generated by `wit-bindgen` 0.3.0. DO NOT EDIT!

// Ensure that the *_component_type.o object is linked in
#ifdef __wasm32__
extern void __component_type_object_force_link_the_world(void);
void __component_type_object_force_link_the_world_public_use_in_this_compilation_unit(
    void) {
  __component_type_object_force_link_the_world();
}
#endif
#include "the_world_cpp.h"
#include <cstdlib> // realloc

extern "C" void *cabi_realloc(void *ptr, size_t old_size, size_t align,
                              size_t new_size);

__attribute__((__weak__, __export_name__("cabi_realloc"))) void *
cabi_realloc(void *ptr, size_t old_size, size_t align, size_t new_size) {
  (void)old_size;
  if (new_size == 0)
    return (void *)align;
  void *ret = realloc(ptr, new_size);
  if (!ret)
    abort();
  return ret;
}

extern "C" __attribute__((import_module("foo:foo/strings")))
__attribute__((import_name("a"))) void
fooX3AfooX2FstringsX00a(uint8_t *, size_t);
extern "C" __attribute__((import_module("foo:foo/strings")))
__attribute__((import_name("b"))) void
fooX3AfooX2FstringsX00b(uint8_t *);
extern "C" __attribute__((import_module("cabi_post_foo:foo/strings")))
__attribute__((import_name("b"))) void
cabi_post_fooX3AfooX2FstringsX00b(uint8_t *);
extern "C" __attribute__((import_module("foo:foo/strings")))
__attribute__((import_name("c"))) void
fooX3AfooX2FstringsX00c(uint8_t *, size_t, uint8_t *, size_t, uint8_t *);
extern "C" __attribute__((import_module("cabi_post_foo:foo/strings")))
__attribute__((import_name("c"))) void
cabi_post_fooX3AfooX2FstringsX00c(uint8_t *);
void comp_a::foo::foo::strings::A(std::string_view x) {
  auto const &vec0 = x;
  auto ptr0 = (uint8_t *)(vec0.data());
  auto len0 = (size_t)(vec0.size());
  fooX3AfooX2FstringsX00a(ptr0, len0);
}
wit::string comp_a::foo::foo::strings::B() {
  uintptr_t ret_area[((2 * sizeof(void *)) + sizeof(uintptr_t) - 1) /
                     sizeof(uintptr_t)];
  uint8_t *ptr0 = (uint8_t *)(&ret_area);
  fooX3AfooX2FstringsX00b(ptr0);
  auto len1 = *((size_t *)(ptr0 + sizeof(void *)));

  auto string1 = wit::string::from_view(
      std::string_view((char const *)(*((uint8_t **)(ptr0 + 0))), len1));

  cabi_post_fooX3AfooX2FstringsX00b(ptr0);
  return string1;
}
wit::string comp_a::foo::foo::strings::C(std::string_view a,
                                         std::string_view b) {
  auto const &vec0 = a;
  auto ptr0 = (uint8_t *)(vec0.data());
  auto len0 = (size_t)(vec0.size());
  auto const &vec1 = b;
  auto ptr1 = (uint8_t *)(vec1.data());
  auto len1 = (size_t)(vec1.size());
  uintptr_t ret_area[((2 * sizeof(void *)) + sizeof(uintptr_t) - 1) /
                     sizeof(uintptr_t)];
  uint8_t *ptr2 = (uint8_t *)(&ret_area);
  fooX3AfooX2FstringsX00c(ptr0, len0, ptr1, len1, ptr2);
  auto len3 = *((size_t *)(ptr2 + sizeof(void *)));

  auto string3 = wit::string::from_view(
      std::string_view((char const *)(*((uint8_t **)(ptr2 + 0))), len3));

  cabi_post_fooX3AfooX2FstringsX00c(ptr2);
  return string3;
}
extern "C" __attribute__((__export_name__("foo:foo/strings#a"))) void
a_fooX3AfooX2FstringsX00a(uint8_t *arg0, size_t arg1) {
  auto len0 = arg1;

  auto string0 =
      wit::string::from_view(std::string_view((char const *)(arg0), len0));

  comp_a::exports::foo::foo::strings::A(std::move(string0));
}
extern "C" __attribute__((__export_name__("foo:foo/strings#b"))) void
a_fooX3AfooX2FstringsX00b(uint8_t *arg0) {
  auto result0 = comp_a::exports::foo::foo::strings::B();
  auto const &vec1 = result0;
  auto ptr1 = (uint8_t *)(vec1.data());
  auto len1 = (size_t)(vec1.size());
  result0.leak();

  *((size_t *)(arg0 + sizeof(void *))) = len1;
  *((uint8_t **)(arg0 + 0)) = ptr1;
}
extern "C"
    __attribute__((__weak__,
                   __export_name__("cabi_post_fooX3AfooX2FstringsX00b"))) void
    a_cabi_post_fooX3AfooX2FstringsX00b(uint8_t *retptr) {
  if ((*((size_t *)(retptr + sizeof(void *)))) > 0) {
    wit::string::drop_raw((void *)(*((uint8_t **)(retptr + 0))));
  }
}
extern "C" __attribute__((__export_name__("foo:foo/strings#c"))) void
a_fooX3AfooX2FstringsX00c(uint8_t *arg0, size_t arg1, uint8_t *arg2, size_t arg3,
                        uint8_t *arg4) {
  auto len0 = arg1;

  auto string0 =
      wit::string::from_view(std::string_view((char const *)(arg0), len0));

  auto len1 = arg3;

  auto string1 =
      wit::string::from_view(std::string_view((char const *)(arg2), len1));

  auto result2 = comp_a::exports::foo::foo::strings::C(std::move(string0),
                                                       std::move(string1));
  auto const &vec3 = result2;
  auto ptr3 = (uint8_t *)(vec3.data());
  auto len3 = (size_t)(vec3.size());
  result2.leak();

  *((size_t *)(arg4 + sizeof(void *))) = len3;
  *((uint8_t **)(arg4 + 0)) = ptr3;
}
extern "C"
    __attribute__((__weak__,
                   __export_name__("cabi_post_fooX3AfooX2FstringsX00c"))) void
    a_cabi_post_fooX3AfooX2FstringsX00c(uint8_t *retptr) {
  if ((*((size_t *)(retptr + sizeof(void *)))) > 0) {
    wit::string::drop_raw((void *)(*((uint8_t **)(retptr + 0))));
  }
}

// Component Adapters
