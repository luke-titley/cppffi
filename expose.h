#define ffi_auto __attribute__((annotate("ffi_expose")))
#define ffi(S) __attribute__((annotate("ffi_expose" #S)))

#include "Imath/ImathVec.h"

namespace Imath_3_0 {

template class ffi(V2f) Vec2<float>;

}
