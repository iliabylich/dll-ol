#include "../headers/dll-ol.h"

DEFINE_TEST(pass)
{
#define GEN_DUMMY_ASSERTION(T, var_name) \
    T var_name = 0;                      \
    ASSERT_EQ(var_name, var_name);

    GEN_DUMMY_ASSERTION(char, char_);
    GEN_DUMMY_ASSERTION(signed char, signed_char_);
    GEN_DUMMY_ASSERTION(unsigned char, unsigned_char_);
    GEN_DUMMY_ASSERTION(short, short_);
    GEN_DUMMY_ASSERTION(short int, short_int_);
    GEN_DUMMY_ASSERTION(signed short, signed_short_);
    GEN_DUMMY_ASSERTION(signed short int, signed_short_int_);
    GEN_DUMMY_ASSERTION(unsigned short, unsigned_short_);
    GEN_DUMMY_ASSERTION(unsigned short int, unsigned_short_int_);
    GEN_DUMMY_ASSERTION(int, int_);
    GEN_DUMMY_ASSERTION(signed, signed_);
    GEN_DUMMY_ASSERTION(signed int, signed_int_);
    GEN_DUMMY_ASSERTION(unsigned, unsigned_);
    GEN_DUMMY_ASSERTION(unsigned int, unsigned_int_);
    GEN_DUMMY_ASSERTION(long, long_);
    GEN_DUMMY_ASSERTION(long int, long_int_);
    GEN_DUMMY_ASSERTION(signed long, signed_long_);
    GEN_DUMMY_ASSERTION(signed long int, signed_long_int_);
    GEN_DUMMY_ASSERTION(unsigned long, unsigned_long_);
    GEN_DUMMY_ASSERTION(unsigned long int, unsigned_long_int_);
    GEN_DUMMY_ASSERTION(long long, long_long_);
    GEN_DUMMY_ASSERTION(long long int, long_long_int_);
    GEN_DUMMY_ASSERTION(signed long long, signed_long_long_);
    GEN_DUMMY_ASSERTION(signed long long int, signed_long_long_int_);
    GEN_DUMMY_ASSERTION(unsigned long long, unsigned_long_long_);
    GEN_DUMMY_ASSERTION(unsigned long long int, unsigned_long_long_int_);
    GEN_DUMMY_ASSERTION(float, float_);
    GEN_DUMMY_ASSERTION(double, double_);
    GEN_DUMMY_ASSERTION(long double, long_double_);

#undef GEN_DUMMY_ASSERTION
}

DEFINE_TEST(fail)
{
    ASSERT_EQ(1, 2);
}

DEFINE_TEST(crash)
{
    *(int *)0 = 0;
}
