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
  SignedIntType.new('char', 'i8'),
  UnsignedIntType.new('unsigned char', 'u8'),
  SignedIntType.new('short', 'i8'),
  UnsignedIntType.new('unsigned short', 'u8'),
  SignedIntType.new('int', 'i8'),
  UnsignedIntType.new('unsigned int', 'u8'),
  SignedIntType.new('long', 'i8'),
  UnsignedIntType.new('unsigned long', 'u8'),
  SignedIntType.new('long long', 'i8'),
  UnsignedIntType.new('unsigned long long', 'u8'),
  SignedIntType.new('int8_t', 'u8'),
  UnsignedIntType.new('uint8_t', 'u8'),
  SignedIntType.new('int16_t', 'u8'),
  UnsignedIntType.new('uint16_t', 'u8'),
  SignedIntType.new('int32_t', 'u8'),
  UnsignedIntType.new('uint32_t', 'u8'),
  SignedIntType.new('int64_t', 'u8'),
  UnsignedIntType.new('uint64_t', 'u8'),
  UnsignedIntType.new('bool', 'u8'),
  UnsignedIntType.new('size_t', 'u8'),
]

FLOAT_TYPES = [
  FloatType.new('float', 'f32'),
  FloatType.new('double', 'f32'),
  FloatType.new('long double', 'f32'),
]

ALL_TYPES = [
  *INTEGER_TYPES,
  *FLOAT_TYPES,
  Type.new('char *', '*const std::ffi::c_char', 'char *', '%s', '"hello"', '"world"'),
  Type.new('void *', '*const std::ffi::c_void', 'void *', '%p', 'NULL', '(void *)42'),
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

# runner/assertions.gen.h
File.open('runner/assertions.gen.h', 'w') do |f|
  f.puts '#include <stdbool.h>'
  f.puts '#include <stddef.h>'
  f.puts '#include <stdint.h>'
  f.puts '#include <stdio.h>'
  f.puts
  ALL_TYPES.each do |type|
    f.puts "void assert_eq_#{type.to_c_id}(#{type.c} lhs, #{type.c} rhs)".fix_ptrs
    f.puts "{"
    f.puts "    printf(\"assert_eq_#{type.to_c_id}(#{type.format_string}, #{type.format_string})\\n\", (#{type.top_c_type})lhs, (#{type.top_c_type})rhs);"
    f.puts "}"
    f.puts "void assert_ne_#{type.to_c_id}(#{type.c} lhs, #{type.c} rhs)".fix_ptrs
    f.puts "{"
    f.puts "    printf(\"assert_ne_#{type.to_c_id}(#{type.format_string}, #{type.format_string})\\n\", (#{type.top_c_type})lhs, (#{type.top_c_type})rhs);"
    f.puts "}"
    f.puts
  end
end

# fixtures/assert_eq_all.gen.h
File.open('fixtures/assert_eq_all.gen.h', 'w') do |f|
  ALL_TYPES.each do |type|
    f.puts "#{type.c} #{type.to_c_id}_ = #{type.sample1};"
    f.puts "assert_eq_#{type.to_c_id}(#{type.to_c_id}_, #{type.to_c_id}_);"
    f.puts "assert_ne_#{type.to_c_id}(#{type.to_c_id}_, #{type.sample2});"
    f.puts
  end
end

# src/assertions/gen.rs
File.open('src/assertions/gen.rs', 'w') do |f|
  ALL_TYPES.each do |type|
    f.puts '#[no_mangle]'
    f.puts "pub extern \"C\" fn assert_eq_#{type.to_c_id}(_lhs: #{type.rs}, _rhs: #{type.rs}) {"
    f.puts '    todo!()'
    f.puts '}'
    f.puts '#[no_mangle]'
    f.puts "pub extern \"C\" fn assert_ne_#{type.to_c_id}(_lhs: #{type.rs}, _rhs: #{type.rs}) {"
    f.puts '    todo!()'
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
