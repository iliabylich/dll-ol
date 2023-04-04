#include "../headers/dll-ol.h"

DEFINE_TEST(pass)
{
    assert_eq(1, 1);
}

DEFINE_TEST(fail)
{
    assert_eq(1, 2);
}

DEFINE_TEST(crash)
{
    *(int *)0 = 0;
}
