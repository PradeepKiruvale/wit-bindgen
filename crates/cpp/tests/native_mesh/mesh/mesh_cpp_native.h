// Generated by `wit-bindgen` 0.3.0. DO NOT EDIT!
#ifndef __CPP_NATIVE_BINDINGS_MESH_H
#define __CPP_NATIVE_BINDINGS_MESH_H
#define WIT_HOST_DIRECT
#include "mesh-exports-foo-foo-resources.h"
#include <cstdint>
#include <map>
#include <utility>
#include <wit-host.h>
/* User class definition file, autogenerated once, then user modified
 * Updated versions of this file are generated into R.template.
 */
namespace mesh {
namespace foo {
namespace foo {
namespace resources {
class R : public wit::ResourceImportBase<R> {

public:
  static void Dtor(R *self) { delete self; }
  R(uint32_t a);
  static Owned New(uint32_t a) { return Owned(new R(a)); }
  void Add(uint32_t b);
};

R::Owned Create();
void Consume(R::Owned o);
} // namespace resources
} // namespace foo
} // namespace foo
} // namespace mesh

#endif