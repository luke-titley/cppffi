#include "Imath_c.h"
#include "stdio.h"

int main()
{
    Imath_3_0__Vec2f a;
    Imath_3_0__Vec2f_new(&a);

    *Imath_3_0__Vec2f_mut_x(&a) = 123.0f;
    *Imath_3_0__Vec2f__mul(&a, 2.0f);

    printf("The x value of the vector %f\n", *Imath_3_0__Vec2f_get_x(&a));

    return 0;
}
