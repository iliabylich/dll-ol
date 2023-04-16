#ifndef DLL_OL_H
#define DLL_OL_H

#if defined(_MSC_VER)
// MSVC requires special handling, as always
#define DEFINE_TEST(name) __declspec(dllexport) void __ol_test_##name()
#else
#define DEFINE_TEST(name) \
    __attribute__((visibility("default"))) void __ol_test_##name()
#endif

#include "./types.h"

void assert_true(bool value);

#define GENERATE_ASSERT_EQ(Type, Typename) void assert_eq_##Typename(Type lhs, Type rhs);
#define GENERATE_ASSERT_NE(Type, Typename) void assert_ne_##Typename(Type lhs, Type rhs);

FOREACH_TYPE(GENERATE_ASSERT_EQ);
FOREACH_TYPE(GENERATE_ASSERT_NE);

#endif // DLL_OL_H
