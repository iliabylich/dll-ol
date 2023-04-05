FIXTURES_RELEASE = fixtures-v0.2
BASE_FIXTURE_URL = https://github.com/iliabylich/dll-ol/releases/download/$(FIXTURES_RELEASE)

fixtures/elf.so:
	wget -q $(BASE_FIXTURE_URL)/elf.so -O $@
FIXTURES += fixtures/elf.so

fixtures/mach-o-binary.dylib:
	wget -q $(BASE_FIXTURE_URL)/mach-o-binary.dylib -O $@
FIXTURES += fixtures/mach-o-binary.dylib

fixtures/pe.dll:
	wget -q $(BASE_FIXTURE_URL)/pe.dll -O $@
FIXTURES += fixtures/pe.dll

CLEAN = $(FIXTURES)

download-fixtures: $(FIXTURES)

mach-o.dylib: fixtures/tests.c headers/dll-ol.h
	$(CC) -Wl,-undefined,dynamic_lookup -shared fixtures/tests.c -o $@
CLEAN += mach-o.dylib

runner: runner.c
	$(CC) $< -o $@
CLEAN += runner

run: runner mach-o.dylib
	./runner

clean:
	rm -f $(CLEAN)

.PHONY: clean download-fixtures
