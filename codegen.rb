INTEGER_TYPES = [
  'char',
  'unsigned char',
  'short',
  'unsigned short',
  'int',
  'unsigned int',
  'long',
  'unsigned long',
  'long long',
  'unsigned long long',
  'int8_t',
  'uint8_t',
  'int16_t',
  'uint16_t',
  'int32_t',
  'uint32_t',
  'int64_t',
  'uint64_t',
  'bool',
  'size_t',
]

FLOAT_TYPES = [
  'float',
  'double',
  'long double',
]

ALL_TYPES = [
  *INTEGER_TYPES,
  *FLOAT_TYPES,
  'char *',
  'void *',
]

# headers/assertions.gen.h
File.open('headers/assertions.gen.h', 'w') do |f|
  f.puts '#include <stdbool.h>'
  f.puts '#include <stddef.h>'
  f.puts '#include <stdint.h>'
  f.puts
  ALL_TYPES.each do |type|
    typename = type.gsub(' *', '_ptr').gsub(' ', '_')
    f.puts "void assert_eq_#{typename}(#{type} lhs, #{type} rhs);"
    f.puts "void assert_ne_#{typename}(#{type} lhs, #{type} rhs);"
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
  INTEGER_TYPES.each do |type|
    typename = type.gsub(' *', '_ptr').gsub(' ', '_')
    f.puts "void assert_eq_#{typename}(#{type} lhs, #{type} rhs)"
    f.puts "{"
    f.puts "    printf(\"assert_eq_#{typename}(%llu, %llu)\\n\", (uint64_t)lhs, (uint64_t)rhs);"
    f.puts "}"
    f.puts "void assert_ne_#{typename}(#{type} lhs, #{type} rhs)"
    f.puts "{"
    f.puts "    printf(\"assert_ne_#{typename}(%llu, %llu)\\n\", (uint64_t)lhs, (uint64_t)rhs);"
    f.puts "}"
    f.puts
  end
  FLOAT_TYPES.each do |type|
    typename = type.gsub(' *', '_ptr').gsub(' ', '_')
    f.puts "void assert_eq_#{typename}(#{type} lhs, #{type} rhs)"
    f.puts "{"
    f.puts "    printf(\"assert_eq_#{typename}(%Lf, %Lf)\\n\", (long double)lhs, (long double)rhs);"
    f.puts "}"
    f.puts "void assert_ne_#{typename}(#{type} lhs, #{type} rhs)"
    f.puts "{"
    f.puts "    printf(\"assert_ne_#{typename}(%Lf, %Lf)\\n\", (long double)lhs, (long double)rhs);"
    f.puts "}"
    f.puts
  end
  f.puts "void assert_eq_char_ptr(char *lhs, char *rhs)"
  f.puts "{"
  f.puts "    printf(\"assert_eq_char_ptr(%s, %s)\\n\", lhs, rhs);"
  f.puts "}"
  f.puts "void assert_ne_char_ptr(char *lhs, char *rhs)"
  f.puts "{"
  f.puts "    printf(\"assert_ne_char_ptr(%s, %s)\\n\", lhs, rhs);"
  f.puts "}"
  f.puts
end

# fixtures/assert_eq_all.gen.h
File.open('fixtures/assert_eq_all.gen.h', 'w') do |f|
  INTEGER_TYPES.each do |type|
    typename = type.gsub(' *', '_ptr').gsub(' ', '_')
    f.puts "#{type} #{typename}_ = 42;"
    f.puts "assert_eq_#{typename}(#{typename}_, #{typename}_);"
    f.puts "assert_ne_#{typename}(#{typename}_, 0);"
    f.puts
  end
  FLOAT_TYPES.each do |type|
    typename = type.gsub(' *', '_ptr').gsub(' ', '_')
    f.puts "#{type} #{typename}_ = 42.0;"
    f.puts "assert_eq_#{typename}(#{typename}_, #{typename}_);"
    f.puts "assert_ne_#{typename}(#{typename}_, 0.0);"
    f.puts
  end
  f.puts "char *char_ptr_ = \"hello\";"
  f.puts "assert_eq_char_ptr(char_ptr_, char_ptr_);"
  f.puts "assert_ne_char_ptr(char_ptr_, \"world\");"
end
