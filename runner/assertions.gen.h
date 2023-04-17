#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <stdio.h>

void assert_eq_char(char lhs, char rhs)
{
    printf("assert_eq_char(%lld, %lld)\n", (int64_t)lhs, (int64_t)rhs);
}
void assert_ne_char(char lhs, char rhs)
{
    printf("assert_ne_char(%lld, %lld)\n", (int64_t)lhs, (int64_t)rhs);
}

void assert_eq_unsigned_char(unsigned char lhs, unsigned char rhs)
{
    printf("assert_eq_unsigned_char(%llu, %llu)\n", (uint64_t)lhs, (uint64_t)rhs);
}
void assert_ne_unsigned_char(unsigned char lhs, unsigned char rhs)
{
    printf("assert_ne_unsigned_char(%llu, %llu)\n", (uint64_t)lhs, (uint64_t)rhs);
}

void assert_eq_short(short lhs, short rhs)
{
    printf("assert_eq_short(%lld, %lld)\n", (int64_t)lhs, (int64_t)rhs);
}
void assert_ne_short(short lhs, short rhs)
{
    printf("assert_ne_short(%lld, %lld)\n", (int64_t)lhs, (int64_t)rhs);
}

void assert_eq_unsigned_short(unsigned short lhs, unsigned short rhs)
{
    printf("assert_eq_unsigned_short(%llu, %llu)\n", (uint64_t)lhs, (uint64_t)rhs);
}
void assert_ne_unsigned_short(unsigned short lhs, unsigned short rhs)
{
    printf("assert_ne_unsigned_short(%llu, %llu)\n", (uint64_t)lhs, (uint64_t)rhs);
}

void assert_eq_int(int lhs, int rhs)
{
    printf("assert_eq_int(%lld, %lld)\n", (int64_t)lhs, (int64_t)rhs);
}
void assert_ne_int(int lhs, int rhs)
{
    printf("assert_ne_int(%lld, %lld)\n", (int64_t)lhs, (int64_t)rhs);
}

void assert_eq_unsigned_int(unsigned int lhs, unsigned int rhs)
{
    printf("assert_eq_unsigned_int(%llu, %llu)\n", (uint64_t)lhs, (uint64_t)rhs);
}
void assert_ne_unsigned_int(unsigned int lhs, unsigned int rhs)
{
    printf("assert_ne_unsigned_int(%llu, %llu)\n", (uint64_t)lhs, (uint64_t)rhs);
}

void assert_eq_long(long lhs, long rhs)
{
    printf("assert_eq_long(%lld, %lld)\n", (int64_t)lhs, (int64_t)rhs);
}
void assert_ne_long(long lhs, long rhs)
{
    printf("assert_ne_long(%lld, %lld)\n", (int64_t)lhs, (int64_t)rhs);
}

void assert_eq_unsigned_long(unsigned long lhs, unsigned long rhs)
{
    printf("assert_eq_unsigned_long(%llu, %llu)\n", (uint64_t)lhs, (uint64_t)rhs);
}
void assert_ne_unsigned_long(unsigned long lhs, unsigned long rhs)
{
    printf("assert_ne_unsigned_long(%llu, %llu)\n", (uint64_t)lhs, (uint64_t)rhs);
}

void assert_eq_long_long(long long lhs, long long rhs)
{
    printf("assert_eq_long_long(%lld, %lld)\n", (int64_t)lhs, (int64_t)rhs);
}
void assert_ne_long_long(long long lhs, long long rhs)
{
    printf("assert_ne_long_long(%lld, %lld)\n", (int64_t)lhs, (int64_t)rhs);
}

void assert_eq_unsigned_long_long(unsigned long long lhs, unsigned long long rhs)
{
    printf("assert_eq_unsigned_long_long(%llu, %llu)\n", (uint64_t)lhs, (uint64_t)rhs);
}
void assert_ne_unsigned_long_long(unsigned long long lhs, unsigned long long rhs)
{
    printf("assert_ne_unsigned_long_long(%llu, %llu)\n", (uint64_t)lhs, (uint64_t)rhs);
}

void assert_eq_int8_t(int8_t lhs, int8_t rhs)
{
    printf("assert_eq_int8_t(%lld, %lld)\n", (int64_t)lhs, (int64_t)rhs);
}
void assert_ne_int8_t(int8_t lhs, int8_t rhs)
{
    printf("assert_ne_int8_t(%lld, %lld)\n", (int64_t)lhs, (int64_t)rhs);
}

void assert_eq_uint8_t(uint8_t lhs, uint8_t rhs)
{
    printf("assert_eq_uint8_t(%llu, %llu)\n", (uint64_t)lhs, (uint64_t)rhs);
}
void assert_ne_uint8_t(uint8_t lhs, uint8_t rhs)
{
    printf("assert_ne_uint8_t(%llu, %llu)\n", (uint64_t)lhs, (uint64_t)rhs);
}

void assert_eq_int16_t(int16_t lhs, int16_t rhs)
{
    printf("assert_eq_int16_t(%lld, %lld)\n", (int64_t)lhs, (int64_t)rhs);
}
void assert_ne_int16_t(int16_t lhs, int16_t rhs)
{
    printf("assert_ne_int16_t(%lld, %lld)\n", (int64_t)lhs, (int64_t)rhs);
}

void assert_eq_uint16_t(uint16_t lhs, uint16_t rhs)
{
    printf("assert_eq_uint16_t(%llu, %llu)\n", (uint64_t)lhs, (uint64_t)rhs);
}
void assert_ne_uint16_t(uint16_t lhs, uint16_t rhs)
{
    printf("assert_ne_uint16_t(%llu, %llu)\n", (uint64_t)lhs, (uint64_t)rhs);
}

void assert_eq_int32_t(int32_t lhs, int32_t rhs)
{
    printf("assert_eq_int32_t(%lld, %lld)\n", (int64_t)lhs, (int64_t)rhs);
}
void assert_ne_int32_t(int32_t lhs, int32_t rhs)
{
    printf("assert_ne_int32_t(%lld, %lld)\n", (int64_t)lhs, (int64_t)rhs);
}

void assert_eq_uint32_t(uint32_t lhs, uint32_t rhs)
{
    printf("assert_eq_uint32_t(%llu, %llu)\n", (uint64_t)lhs, (uint64_t)rhs);
}
void assert_ne_uint32_t(uint32_t lhs, uint32_t rhs)
{
    printf("assert_ne_uint32_t(%llu, %llu)\n", (uint64_t)lhs, (uint64_t)rhs);
}

void assert_eq_int64_t(int64_t lhs, int64_t rhs)
{
    printf("assert_eq_int64_t(%lld, %lld)\n", (int64_t)lhs, (int64_t)rhs);
}
void assert_ne_int64_t(int64_t lhs, int64_t rhs)
{
    printf("assert_ne_int64_t(%lld, %lld)\n", (int64_t)lhs, (int64_t)rhs);
}

void assert_eq_uint64_t(uint64_t lhs, uint64_t rhs)
{
    printf("assert_eq_uint64_t(%llu, %llu)\n", (uint64_t)lhs, (uint64_t)rhs);
}
void assert_ne_uint64_t(uint64_t lhs, uint64_t rhs)
{
    printf("assert_ne_uint64_t(%llu, %llu)\n", (uint64_t)lhs, (uint64_t)rhs);
}

void assert_eq_bool(bool lhs, bool rhs)
{
    printf("assert_eq_bool(%llu, %llu)\n", (uint64_t)lhs, (uint64_t)rhs);
}
void assert_ne_bool(bool lhs, bool rhs)
{
    printf("assert_ne_bool(%llu, %llu)\n", (uint64_t)lhs, (uint64_t)rhs);
}

void assert_eq_size_t(size_t lhs, size_t rhs)
{
    printf("assert_eq_size_t(%llu, %llu)\n", (uint64_t)lhs, (uint64_t)rhs);
}
void assert_ne_size_t(size_t lhs, size_t rhs)
{
    printf("assert_ne_size_t(%llu, %llu)\n", (uint64_t)lhs, (uint64_t)rhs);
}

void assert_eq_float(float lhs, float rhs)
{
    printf("assert_eq_float(%Lf, %Lf)\n", (long double)lhs, (long double)rhs);
}
void assert_ne_float(float lhs, float rhs)
{
    printf("assert_ne_float(%Lf, %Lf)\n", (long double)lhs, (long double)rhs);
}

void assert_eq_double(double lhs, double rhs)
{
    printf("assert_eq_double(%Lf, %Lf)\n", (long double)lhs, (long double)rhs);
}
void assert_ne_double(double lhs, double rhs)
{
    printf("assert_ne_double(%Lf, %Lf)\n", (long double)lhs, (long double)rhs);
}

void assert_eq_long_double(long double lhs, long double rhs)
{
    printf("assert_eq_long_double(%Lf, %Lf)\n", (long double)lhs, (long double)rhs);
}
void assert_ne_long_double(long double lhs, long double rhs)
{
    printf("assert_ne_long_double(%Lf, %Lf)\n", (long double)lhs, (long double)rhs);
}

void assert_eq_char_ptr(char *lhs, char *rhs)
{
    printf("assert_eq_char_ptr(%s, %s)\n", (char *)lhs, (char *)rhs);
}
void assert_ne_char_ptr(char *lhs, char *rhs)
{
    printf("assert_ne_char_ptr(%s, %s)\n", (char *)lhs, (char *)rhs);
}

void assert_eq_void_ptr(void *lhs, void *rhs)
{
    printf("assert_eq_void_ptr(%p, %p)\n", (void *)lhs, (void *)rhs);
}
void assert_ne_void_ptr(void *lhs, void *rhs)
{
    printf("assert_ne_void_ptr(%p, %p)\n", (void *)lhs, (void *)rhs);
}

