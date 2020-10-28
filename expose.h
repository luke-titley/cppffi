
#define xstr(s) str(s)
#define str(s) #s

#define ffi_expose __attribute__((annotate("ffi_expose")))

#include "Imath/ImathVec.h"

namespace Imath_3_0 {

// Instantiate templates
//template class Imath::Vec2<float> ffi_expose;
template class ffi_expose Imath::Vec2<float>;

// Expose them with typedefs 
//typedef Imath::Vec2<float> Vec2f ffi_expose;

}
