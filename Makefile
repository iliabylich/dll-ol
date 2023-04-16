include fixtures/build.mk
include headers/build.mk
include runner/build.mk

clean:
	rm -f $(CLEAN)

.PHONY: clean
