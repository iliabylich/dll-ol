#ifndef DLL_OL_H
#define DLL_OL_H

#if defined(_MSC_VER)
// MSVC requires special handling, as always
#define DEFINE_TEST(name) __declspec(dllexport) void __ol_test_##name()
#else
#define DEFINE_TEST(name) \
    __attribute__((visibility("default"))) void __ol_test_##name()
#endif

#include <stdint.h>

// clang-format off
#if defined(_MSC_VER)
#define SIGNED_CHAR_TO_ACTION(action)
#else
#define SIGNED_CHAR_TO_ACTION(action) signed char: action,
#endif

#if defined(_MSC_VER)
#define SIGNED_CHAR_TO_ITS_NAME
#else
#define SIGNED_CHAR_TO_ITS_NAME signed char: "signed char",
#endif

#define FOR_ANY_INTEGER(action) \
    char: action, \
    SIGNED_CHAR_TO_ACTION(action) \
    unsigned char: action, \
    \
    short: action, \
    unsigned short: action, \
    \
    int: action, \
    unsigned int: action, \
    \
    long: action, \
    unsigned long: action, \
    \
    long long: action, \
    unsigned long long: action

#define FOR_ANY_FLOAT(action) \
    float: action, \
    double: action, \
    long double: action

#define DLL_OL_OBJECT_TYPE(v) \
    _Generic(v, \
        char: "char", \
        SIGNED_CHAR_TO_ITS_NAME \
        unsigned char: "unsigned char", \
        \
        short: "short", \
        unsigned short: "unsigned short", \
        \
        int: "int", \
        unsigned int: "unsigned int", \
        \
        long: "long", \
        unsigned long: "unsigned long", \
        \
        long long: "long long", \
        unsigned long long: "unsigned long long", \
        \
        float: "float", \
        double: "double", \
        long double: "long double", \
        char *: "char *", \
        default: "unsupported" \
    )

#define ASSERT_EQ_BOTH_LHS_AND_RHS_ARE_INTS(lhs, rhs) \
    assert_eq_signed_ints(DLL_OL_OBJECT_TYPE(lhs), DLL_OL_OBJECT_TYPE(rhs), (uint64_t)(lhs), (uint64_t)rhs)

#define ASSERT_EQ_BOTH_LHS_AND_RHS_ARE_FLOATS(lhs, rhs) \
    assert_eq_floats(DLL_OL_OBJECT_TYPE(lhs), DLL_OL_OBJECT_TYPE(rhs), (double)(lhs), (double)rhs)

#define ASSERT_EQ_LHS_IS_SIGNED_INT(lhs, rhs) \
    _Generic((rhs), \
        FOR_ANY_INTEGER(ASSERT_EQ_BOTH_LHS_AND_RHS_ARE_INTS(lhs, rhs)), \
        FOR_ANY_FLOAT(ASSERT_EQ_BOTH_LHS_AND_RHS_ARE_FLOATS(lhs, rhs)), \
        default: dll_ol_unsupported_assert_eq(DLL_OL_OBJECT_TYPE(lhs), DLL_OL_OBJECT_TYPE(rhs)) \
    )

#define ASSERT_EQ_LHS_IS_FLOAT(lhs, rhs) \
    _Generic((rhs), \
        FOR_ANY_INTEGER(ASSERT_EQ_BOTH_LHS_AND_RHS_ARE_FLOATS(lhs, rhs)), \
        FOR_ANY_FLOAT(ASSERT_EQ_BOTH_LHS_AND_RHS_ARE_FLOATS(lhs, rhs)), \
        default: dll_ol_unsupported_assert_eq(DLL_OL_OBJECT_TYPE(lhs), DLL_OL_OBJECT_TYPE(rhs)) \
    )

#define ASSERT_EQ(lhs, rhs) \
    _Generic((lhs), \
        FOR_ANY_INTEGER(ASSERT_EQ_LHS_IS_SIGNED_INT((lhs), (rhs))), \
        FOR_ANY_FLOAT(ASSERT_EQ_LHS_IS_FLOAT((lhs), (rhs))), \
        default: dll_ol_unsupported_assert_eq(DLL_OL_OBJECT_TYPE(lhs), DLL_OL_OBJECT_TYPE(rhs)) \
    )
// clang-format on

void assert_eq_signed_ints(const char *lhs_type, const char *rhs_type, int64_t lhs, int64_t rhs);
void assert_eq_floats(const char *lhs_type, const char *rhs_type, double lhs, double rhs);
void dll_ol_unsupported_assert_eq(const char *lhs_type, const char *rhs_type);

#endif // DLL_OL_H
