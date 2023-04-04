FIXTURES_RELEASE = fixtures-v0.1
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

download-fixtures: $(FIXTURES)

clean:
	rm -f $(FIXTURES)

.PHONY: clean download-fixtures
