
#define xstr(s) str(s)
#define str(s) #s

#define ffi_expose __attribute__((annotate("ffi_expose")))
#define ffi_expose_named(S) __attribute__((annotate("ffi_expose" #S)))

#include "Imath/ImathVec.h"

namespace Imath_3_0 {

/*
class ffi_expose Test {
    void foo(){}
};
*/

// Instantiate templates
//template class Imath::Vec2<float> ffi_expose;
template class ffi_expose Imath::Vec2<float>;
//template class ffi_expose Imath::Vec2<double>;

// Expose them with typedefs 
//typedef Imath::Vec2<float> Vec2f ffi_expose;

}
