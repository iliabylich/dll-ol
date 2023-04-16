runner/assertions.gen.h: codegen.rb
	ruby codegen.rb
CLEAN += runner/assertions.gen.h

runner/main: runner/main.c runner/assertions.gen.h
	$(CC) $< -o $@
CLEAN += runner/main

run: runner/main fixtures/mach-o-binary.dylib
	./runner/main
