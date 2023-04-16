headers/assertions.gen.h: codegen.rb
	ruby codegen.rb
CLEAN += headers/assertions.gen.h
