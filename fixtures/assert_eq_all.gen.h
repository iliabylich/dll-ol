char char_ = -1;
assert_eq_char(char_, char_);
assert_ne_char(char_, -2);

unsigned char unsigned_char_ = 1;
assert_eq_unsigned_char(unsigned_char_, unsigned_char_);
assert_ne_unsigned_char(unsigned_char_, 2);

short short_ = -1;
assert_eq_short(short_, short_);
assert_ne_short(short_, -2);

unsigned short unsigned_short_ = 1;
assert_eq_unsigned_short(unsigned_short_, unsigned_short_);
assert_ne_unsigned_short(unsigned_short_, 2);

int int_ = -1;
assert_eq_int(int_, int_);
assert_ne_int(int_, -2);

unsigned int unsigned_int_ = 1;
assert_eq_unsigned_int(unsigned_int_, unsigned_int_);
assert_ne_unsigned_int(unsigned_int_, 2);

long long_ = -1;
assert_eq_long(long_, long_);
assert_ne_long(long_, -2);

unsigned long unsigned_long_ = 1;
assert_eq_unsigned_long(unsigned_long_, unsigned_long_);
assert_ne_unsigned_long(unsigned_long_, 2);

long long long_long_ = -1;
assert_eq_long_long(long_long_, long_long_);
assert_ne_long_long(long_long_, -2);

unsigned long long unsigned_long_long_ = 1;
assert_eq_unsigned_long_long(unsigned_long_long_, unsigned_long_long_);
assert_ne_unsigned_long_long(unsigned_long_long_, 2);

int8_t int8_t_ = -1;
assert_eq_int8_t(int8_t_, int8_t_);
assert_ne_int8_t(int8_t_, -2);

uint8_t uint8_t_ = 1;
assert_eq_uint8_t(uint8_t_, uint8_t_);
assert_ne_uint8_t(uint8_t_, 2);

int16_t int16_t_ = -1;
assert_eq_int16_t(int16_t_, int16_t_);
assert_ne_int16_t(int16_t_, -2);

uint16_t uint16_t_ = 1;
assert_eq_uint16_t(uint16_t_, uint16_t_);
assert_ne_uint16_t(uint16_t_, 2);

int32_t int32_t_ = -1;
assert_eq_int32_t(int32_t_, int32_t_);
assert_ne_int32_t(int32_t_, -2);

uint32_t uint32_t_ = 1;
assert_eq_uint32_t(uint32_t_, uint32_t_);
assert_ne_uint32_t(uint32_t_, 2);

int64_t int64_t_ = -1;
assert_eq_int64_t(int64_t_, int64_t_);
assert_ne_int64_t(int64_t_, -2);

uint64_t uint64_t_ = 1;
assert_eq_uint64_t(uint64_t_, uint64_t_);
assert_ne_uint64_t(uint64_t_, 2);

bool bool_ = 1;
assert_eq_bool(bool_, bool_);
assert_ne_bool(bool_, 0);

size_t size_t_ = 1;
assert_eq_size_t(size_t_, size_t_);
assert_ne_size_t(size_t_, 2);

float float_ = 42.0;
assert_eq_float(float_, float_);
assert_ne_float(float_, 43.0);

double double_ = 42.0;
assert_eq_double(double_, double_);
assert_ne_double(double_, 43.0);

char * char_ptr_ = "hello";
assert_eq_char_ptr(char_ptr_, char_ptr_);
assert_ne_char_ptr(char_ptr_, "world");

void * void_ptr_ = NULL;
assert_eq_void_ptr(void_ptr_, void_ptr_);
assert_ne_void_ptr(void_ptr_, (void *)42);

