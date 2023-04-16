#include "../headers/dll-ol.h"

DEFINE_TEST(pass)
{
// Integers
#define GENERATE_INTEGER_TEST(Type, Typename)       \
    Type Typename##_ = 42;                          \
    assert_eq_##Typename(Typename##_, Typename##_); \
    assert_ne_##Typename(Typename##_, 0);

    FOREACH_INTEGER(GENERATE_INTEGER_TEST);
#undef GENERATE_INTEGER_TEST

    // Floats
#define GENERATE_FLOAT_TEST(Type, Typename)         \
    Type Typename##_ = 42.0;                        \
    assert_eq_##Typename(Typename##_, Typename##_); \
    assert_ne_##Typename(Typename##_, 0);

    FOREACH_FLOAT(GENERATE_FLOAT_TEST);
#undef GENERATE_FLOAT_TEST

    // Char pointers
    char *char_ptr = "Hello, world!";
    assert_eq_char_ptr(char_ptr, char_ptr);
    assert_ne_char_ptr(char_ptr, "different string");
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
