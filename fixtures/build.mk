BASE_FIXTURE_URL = https://github.com/iliabylich/dll-ol/releases/download/fixtures-latest

MACH_O_DYLIB = mach-o-binary.dylib
ELF_SO = elf.so
PE_DLL = pe.dll

ifdef MACH_O_BUILD_LOCALLY
GET_MACH_O = $(CC) -g -Wl,-undefined,dynamic_lookup -shared fixtures/tests.gen.c -o fixtures/$(MACH_O_DYLIB)
else
GET_MACH_O = wget -q $(BASE_FIXTURE_URL)/$(MACH_O_DYLIB) -O fixtures/$(MACH_O_DYLIB)
endif

ifdef SO_BUILD_LOCALLY
GET_SO = $(CC) -g fixtures/tests.gen.c -shared -o fixtures/$(ELF_SO)
else
GET_SO = wget -q $(BASE_FIXTURE_URL)/$(ELF_SO) -O fixtures/$(ELF_SO)
endif

ifdef PE_BUILD_LOCALLY
GET_PE = cl.exe /std:c11 /D_USRDLL /D_WINDLL fixtures/tests.gen.c /MT /link /FORCE:UNRESOLVED /DLL /OUT:fixtures/$(PE_DLL)
else
GET_PE = wget -q $(BASE_FIXTURE_URL)/$(PE_DLL) -O fixtures/$(PE_DLL)
endif

fixtures/$(MACH_O_DYLIB): fixtures/tests.gen.c headers/assertions.gen.h
	$(GET_MACH_O)
CLEAN += fixtures/$(MACH_O_DYLIB)

fixtures/$(ELF_SO): fixtures/tests.gen.c headers/assertions.gen.h
	$(GET_SO)
CLEAN += fixtures/$(ELF_SO)

fixtures/$(PE_DLL): fixtures/tests.gen.c headers/assertions.gen.h
	$(GET_PE)
CLEAN += fixtures/$(PE_DLL)

fixtures/tests.gen.c: codegen.rb
	ruby codegen.rb
CLEAN += fixtures/tests.gen.c

download-fixtures: fixtures/$(MACH_O_DYLIB) fixtures/$(ELF_SO) fixtures/$(PE_DLL)

.PHONY: download-fixtures
