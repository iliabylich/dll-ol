#ifndef DLL_OL_H
#define DLL_OL_H

#if defined(_MSC_VER)
// MSVC requires special handling, as always
#define DEFINE_TEST(name) __declspec(dllexport) void __ol_test_##name()
#else
#define DEFINE_TEST(name) \
    __attribute__((visibility("default"))) void __ol_test_##name()
#endif

#include "./assertions.gen.h"

void assert_true(bool value);
void assert_false(bool value);

#endif // DLL_OL_H
