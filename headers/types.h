#include <stdint.h>
#include <stdbool.h>
#include <stddef.h>

#define FOREACH_INTEGER(action)                     \
    action(char, char);                             \
    action(unsigned char, unsigned_char);           \
    action(short, short);                           \
    action(unsigned short, unsigned_short);         \
    action(int, int);                               \
    action(unsigned int, unsigned_int);             \
    action(long, long);                             \
    action(unsigned long, unsigned_long);           \
    action(long long, long_long);                   \
    action(unsigned long long, unsigned_long_long); \
    action(int8_t, int8_t);                         \
    action(uint8_t, uint8_t);                       \
    action(int16_t, int16_t);                       \
    action(uint16_t, uint16_t);                     \
    action(int32_t, int32_t);                       \
    action(uint32_t, uint32_t);                     \
    action(int64_t, int64_t);                       \
    action(uint64_t, uint64_t);                     \
    action(bool, bool);                             \
    action(size_t, size_t);

#define FOREACH_FLOAT(action) \
    action(float, float);     \
    action(double, double);   \
    action(long double, long_double)

#define FOREACH_TYPE(action) \
    FOREACH_INTEGER(action); \
    FOREACH_FLOAT(action);   \
    action(bool, bool);      \
    action(char *, char_ptr);
