
#define ffi_expose __attribute__((annotate("ffi_expose")))

#include "Imath/ImathVec.h"

namespace Imath {

typedef Imath::Vec2<float> Vec2f ffi_expose;
//using Vec2f ffi_expose = Imath::Vec2<float> ffi_expose;
//class ffi_expose Vec2f : public Imath::Vec2<float> {};

}
