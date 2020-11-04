#include "Imath_c.h"
#include "stdio.h"

int main()
{
    Imath_3_0__V2f a;
    Imath_3_0__V2f_new(&a);

    *Imath_3_0__V2f_mut_x(&a) = 123.0f;
    Imath_3_0__V2f__imul_scalar(&a, 2.0f);

    printf("The x value of the vector %f\n", *Imath_3_0__V2f_get_x(&a));

    return 0;
}
