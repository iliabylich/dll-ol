#ifndef DLL_OL_H
#define DLL_OL_H

#if defined(_MSC_VER)
// MSVC requires special handling, as always
#define DEFINE_TEST(name) __declspec(dllexport) void __ol_test_##name()
#else
#define DEFINE_TEST(name) \
    __attribute__((visibility("default"))) void __ol_test_##name()
#endif

void assert_eq(int x, int y);

#endif // DLL_OL_H
