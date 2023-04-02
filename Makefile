fixtures/elf.so:
	wget -q https://github.com/iliabylich/dll-ol/releases/download/dlib-fixtures/elf.so -O $@
FIXTURES += fixtures/elf.so

fixtures/mach-o-binary.dylib:
	wget -q https://github.com/iliabylich/dll-ol/releases/download/dlib-fixtures/mach-o-binary.dylib -O $@
FIXTURES += fixtures/mach-o-binary.dylib

fixtures/pe.dll:
	wget -q https://github.com/iliabylich/dll-ol/releases/download/dlib-fixtures/pe.dll -O $@
FIXTURES += fixtures/pe.dll

download-fixtures: $(FIXTURES)

clean:
	rm -f $(FIXTURES)

.PHONY: clean download-fixtures
