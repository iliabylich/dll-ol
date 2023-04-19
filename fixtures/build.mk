FIXTURES_RELEASE = fixtures-v0.3
BASE_FIXTURE_URL = https://github.com/iliabylich/dll-ol/releases/download/$(FIXTURES_RELEASE)

MACH_O_DYLIB = fixtures/mach-o-binary.dylib
ELF_SO = fixtures/elf.so
PE_DLL = fixtures/pe.dll

ifdef MACH_O_BUILD_LOCALLY
GET_MACH_O = $(CC) -Wl,-undefined,dynamic_lookup -shared fixtures/tests.c -o $(MACH_O_DYLIB)
else
GET_MACH_O = wget -q $(BASE_FIXTURE_URL)/mach-o-binary.dylib -O $(MACH_O_DYLIB)
endif

ifdef SO_BUILD_LOCALLY
GET_SO = $(CC) fixtures/tests.c -shared -o $(ELF_SO)
else
GET_SO = wget -q $(BASE_FIXTURE_URL)/elf.so -O $(ELF_SO)
endif

ifdef PE_BUILD_LOCALLY
GET_PE = cl.exe /std:c11 /D_USRDLL /D_WINDLL fixtures/tests.c /MT /link /FORCE:UNRESOLVED /DLL /OUT:$(PE_DLL)
else
GET_PE = wget -q $(BASE_FIXTURE_URL)/pe.dll -O $(PE_DLL)
endif

$(MACH_O_DYLIB): fixtures/tests.c headers/assertions.gen.h fixtures/assert_eq_all.gen.h
	$(GET_MACH_O)
CLEAN += $(MACH_O_DYLIB)

$(ELF_SO): fixtures/tests.c headers/assertions.gen.h fixtures/assert_eq_all.gen.h
	$(GET_SO)
CLEAN += $(ELF_SO)

$(PE_DLL): fixtures/tests.c headers/assertions.gen.h fixtures/assert_eq_all.gen.h
	$(GET_PE)
CLEAN += $(PE_DLL)

fixtures/assert_eq_all.gen.h: codegen.rb
	ruby codegen.rb
CLEAN += fixtures/assert_eq_all.gen.h

download-fixtures: $(MACH_O_DYLIB) $(ELF_SO) $(PE_DLL)

.PHONY: download-fixtures
