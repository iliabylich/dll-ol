#if defined(_MSC_VER)
// MSVC requires special handling, as always
#define DEFINE_TEST(name) __declspec(dllexport) void __test_##name()
#else
#define DEFINE_TEST(name) __attribute__((visibility("default"))) void __test_##name()
#endif

DEFINE_TEST(pass) {}

DEFINE_TEST(fail)
{
    *(int *)0 = 0;
}
