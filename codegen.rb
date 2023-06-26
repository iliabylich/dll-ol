class Type < Struct.new(:c, :rs, :top_c_type, :format_string, :sample1, :sample2)
  def to_c_id
    c.gsub(' *', '_ptr').gsub(' ', '_')
  end
end

class SignedIntType < Type
  def initialize(c, rs)
    super(c, rs, 'int64_t', '%lld', '-1', '-2')
  end
end

class UnsignedIntType < Type
  def initialize(c, rs)
    super(c, rs, 'uint64_t', '%llu', '1', '2')
  end
end

class FloatType < Type
  def initialize(c, rs)
    super(c, rs, 'long double', '%Lf', '42.0', '43.0')
  end
end

class String
  def fix_ptrs
    gsub(' * ', ' *')
  end
end

INTEGER_TYPES = [
  SignedIntType.new('char', 'c_char'),
  UnsignedIntType.new('unsigned char', 'c_uchar'),
  SignedIntType.new('short', 'c_short'),
  UnsignedIntType.new('unsigned short', 'c_ushort'),
  SignedIntType.new('int', 'c_int'),
  UnsignedIntType.new('unsigned int', 'c_uint'),
  SignedIntType.new('long', 'c_long'),
  UnsignedIntType.new('unsigned long', 'c_ulong'),
  SignedIntType.new('long long', 'c_longlong'),
  UnsignedIntType.new('unsigned long long', 'c_ulonglong'),
  SignedIntType.new('int8_t', 'i8'),
  UnsignedIntType.new('uint8_t', 'u8'),
  SignedIntType.new('int16_t', 'u16'),
  UnsignedIntType.new('uint16_t', 'i16'),
  SignedIntType.new('int32_t', 'u32'),
  UnsignedIntType.new('uint32_t', 'i32'),
  SignedIntType.new('int64_t', 'u64'),
  UnsignedIntType.new('uint64_t', 'i64'),
  Type.new('bool', 'bool', 'uint64_t', '%llu', '1', '0'),
  UnsignedIntType.new('size_t', 'usize'),
]

FLOAT_TYPES = [
  FloatType.new('float', 'c_float'),
  FloatType.new('double', 'c_double'),
]

ALL_TYPES = [
  *INTEGER_TYPES,
  *FLOAT_TYPES,
  Type.new('char *', '*const c_char', 'char *', '%s', '"hello"', '"world"'),
  Type.new('void *', '*const c_void', 'void *', '%p', 'NULL', '(void *)42'),
]

# headers/assertions.gen.h
File.open('headers/assertions.gen.h', 'w') do |f|
  f.puts '#include <stdbool.h>'
  f.puts '#include <stddef.h>'
  f.puts '#include <stdint.h>'
  f.puts
  ALL_TYPES.each do |type|
    f.puts "void assert_eq_#{type.to_c_id}(#{type.c} lhs, #{type.c} rhs);"
    f.puts "void assert_ne_#{type.to_c_id}(#{type.c} lhs, #{type.c} rhs);"
    f.puts
  end
end

# fixtures/tests.gen.c
File.open('fixtures/tests.gen.c', 'w') do |f|
  f.puts '#include "../headers/dll-ol.h"'
  f.puts
  f.puts 'DEFINE_TEST(pass) {'
  ALL_TYPES.each do |type|
    f.puts "  #{type.c} #{type.to_c_id}_ = #{type.sample1};"
    f.puts "  assert_eq_#{type.to_c_id}(#{type.to_c_id}_, #{type.to_c_id}_);"
    f.puts "  assert_ne_#{type.to_c_id}(#{type.to_c_id}_, #{type.sample2});"
    f.puts
  end
  f.puts '}'
  f.puts
  f.puts 'DEFINE_TEST(fail) {'
  f.puts '  assert_eq_int(1, 2);'
  f.puts '}'
  f.puts
  f.puts 'DEFINE_TEST(crash) {'
  f.puts '  int *ptr = 0;'
  f.puts '  *ptr = 42;'
  f.puts '}'
end

# src/assertions/gen.rs
File.open('src/assertions/gen.rs', 'w') do |f|
  f.puts 'use crate::assertions::{AssertEq, AssertNe};'
  f.puts 'use std::ffi::{'
  f.puts '    c_char, c_double, c_float, c_int, c_long, c_longlong, c_short, c_uchar, c_uint, c_ulong,'
  f.puts '    c_ulonglong, c_ushort, c_void,'
  f.puts '};'
  f.puts
  ALL_TYPES.each do |type|
    f.puts '#[no_mangle]'
    f.puts "pub extern \"C\" fn assert_eq_#{type.to_c_id}(lhs: #{type.rs}, rhs: #{type.rs}) {"
    f.puts "    AssertEq::run(lhs, rhs);"
    f.puts '}'
    f.puts '#[no_mangle]'
    f.puts "pub extern \"C\" fn assert_ne_#{type.to_c_id}(lhs: #{type.rs}, rhs: #{type.rs}) {"
    f.puts "    AssertNe::run(lhs, rhs);"
    f.puts '}'
    f.puts
  end

  f.puts 'pub fn trigger_inclusion() -> usize {'
  f.puts '    ['
  ALL_TYPES.each do |type|
    f.puts "        assert_eq_#{type.to_c_id} as *mut std::ffi::c_void as usize,"
    f.puts "        assert_ne_#{type.to_c_id} as *mut std::ffi::c_void as usize,"
  end
  f.puts '    ]'
  f.puts '    .into_iter()'
  f.puts '    .fold(0, |acc, e| acc.wrapping_add(e))'
  f.puts '}'
end
