#include <dlfcn.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef void (*test_fn)();

void assert_eq_signed_ints(const char *lhs_type, const char *rhs_type, int64_t lhs, int64_t rhs)
{
    printf("assert_eq(%s, %lli, %s, %lli)\n", lhs_type, lhs, rhs_type, rhs);
}

void assert_eq_floats(const char *lhs_type, const char *rhs_type, double lhs, double rhs)
{
    printf("assert_eq(%s %lf, %s %lf)\n", lhs_type, lhs, rhs_type, rhs);
}

void dll_ol_unsupported_assert_eq(const char *lhs_type, const char *rhs_type)
{
    printf("unsupported comparison: %s vs %s\n", lhs_type, rhs_type);
}

int run_test(void *handle, const char *name)
{
    test_fn f = dlsym(handle, name);
    if (f == NULL)
    {
        printf("test %s does not exist\n", name);
        return 1;
    }
    printf("%s...\n", name);
    f();
    return 0;
}

const char *tests[] = {
    "__ol_test_pass",
    "__ol_test_fail",
    "__ol_test_crash",
};

int main(void)
{
    void *handle = dlopen("./fixtures/mach-o-binary.dylib", RTLD_LAZY | RTLD_GLOBAL);
    if (handle == NULL)
    {
        printf("Failed to open dylib\n");
        char *err = dlerror();
        printf("dlerror = %s\n", err);
        return 1;
    }
    for (size_t i = 0; i < 3; i++)
    {
        const char *test = tests[i];
        if (run_test(handle, test))
        {
            return 1;
        }
    }
    return 0;
}
