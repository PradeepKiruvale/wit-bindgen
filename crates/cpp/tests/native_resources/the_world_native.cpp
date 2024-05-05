// Generated by `wit-bindgen` 0.3.0. DO NOT EDIT!
#include "the_world_cpp_native.h"
#include <stdlib.h>
template <class R> std::map<int32_t, R> wit::ResourceTable<R>::resources;

extern "C" __attribute__((import_module("foo:foo/resources")))
__attribute__((import_name("[dtor]r")))
void fooX3AfooX2FresourcesX23X5BdtorX5Dr(uint8_t*);
extern "C" __attribute__((import_module("foo:foo/resources")))
__attribute__((import_name("[constructor]r")))
int32_t fooX3AfooX2FresourcesX23X5BconstructorX5Dr(int32_t);
extern "C" __attribute__((import_module("foo:foo/resources")))
__attribute__((import_name("[method]r.add"))) void
fooX3AfooX2FresourcesX23X5BmethodX5DrX2Eadd(uint8_t *, int32_t);
extern "C" __attribute__((import_module("foo:foo/resources")))
__attribute__((import_name("create"))) int32_t
fooX3AfooX2FresourcesX23create();
extern "C" __attribute__((import_module("foo:foo/resources")))
__attribute__((import_name("borrows"))) void
    fooX3AfooX2FresourcesX23borrows(uint8_t*);
extern "C" __attribute__((import_module("foo:foo/resources")))
__attribute__((import_name("consume"))) void
    fooX3AfooX2FresourcesX23consume(int32_t);

extern "C" void fooX3AfooX2FresourcesX00X5Bresource_dropX5Dr(int32_t arg0) {
  auto ptr = foo::foo::resources::R::remove_resource(arg0);
  assert(ptr.has_value());
  foo::foo::resources::R::Dtor(*ptr);
}
extern "C" int32_t fooX3AfooX2FresourcesX00X5BconstructorX5Dr(int32_t arg0) {
  auto result0 = (foo::foo::resources::R::New((uint32_t(arg0)))).release();
  return (*(result0)).get_handle();
}
extern "C" void fooX3AfooX2FresourcesX00X5BmethodX5DrX2Eadd(int32_t arg0,
                                                            int32_t arg1) {
  (*foo::foo::resources::R::lookup_resource(arg0))->Add((uint32_t(arg1)));
}
extern "C" int32_t fooX3AfooX2FresourcesX00create() {
  auto result0 = foo::foo::resources::Create();
  return result0.release()->get_handle();
}
extern "C" void fooX3AfooX2FresourcesX00borrows(int32_t arg0) {
  foo::foo::resources::Borrows(**foo::foo::resources::R::lookup_resource(arg0));
}
extern "C" void fooX3AfooX2FresourcesX00consume(int32_t arg0) {
  auto objptr = foo::foo::resources::R::remove_resource(arg0);
  assert(objptr.has_value());
  foo::foo::resources::Consume(foo::foo::resources::R::Owned(*objptr));
}

exports::foo::foo::resources::R::~R() {
  if (this->rep)
    fooX3AfooX2FresourcesX23X5BdtorX5Dr(this->rep);
}
exports::foo::foo::resources::R::R(uint32_t a) {
  auto ret = fooX3AfooX2FresourcesX23X5BconstructorX5Dr((int32_t(a)));
  this->index = ret;
  this->rep = *lookup_resource(ret);
}
void exports::foo::foo::resources::R::Add(uint32_t b) const {
  fooX3AfooX2FresourcesX23X5BmethodX5DrX2Eadd((*this).get_rep(),
                                              (int32_t(b)));
}
exports::foo::foo::resources::R::R(wit::ResourceExportBase&& b)
 : wit::ResourceExportBase(std::move(b)) {}
extern "C" int32_t
X5BexportX5DfooX3AfooX2FresourcesX00X5Bresource_newX5Dr(uint8_t *arg0) {
  return exports::foo::foo::resources::R::store_resource(std::move(arg0));
}
extern "C" uint8_t *
X5BexportX5DfooX3AfooX2FresourcesX00X5Bresource_repX5Dr(int32_t arg0) {
  return *exports::foo::foo::resources::R::lookup_resource(arg0);
}
extern "C" void
X5BexportX5DfooX3AfooX2FresourcesX00X5Bresource_dropX5Dr(int32_t arg0) {
  exports::foo::foo::resources::R::remove_resource(arg0);
}
exports::foo::foo::resources::R exports::foo::foo::resources::Create() {
  auto ret = fooX3AfooX2FresourcesX23create();
  return wit::ResourceExportBase{ret};
}
void exports::foo::foo::resources::Borrows(std::reference_wrapper<const R> o) {
  fooX3AfooX2FresourcesX23borrows(o.get().get_rep());
}
void exports::foo::foo::resources::Consume(R &&o) {
  auto rep = o.take_rep();
  fooX3AfooX2FresourcesX23consume(o.get_handle());
}

// Component Adapters
