#include "../headers/dll-ol.h"

DEFINE_TEST(pass){
#include "assert_eq_all.gen.h"
}

DEFINE_TEST(fail)
{
    assert_eq_int(1, 2);
}

DEFINE_TEST(crash)
{
    int *ptr = 0;
    *ptr = 0;
}
