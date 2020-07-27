
all: files

files:
	$(MAKE) -C $@

clean:
	$(MAKE) -C ./files/ clean

.PHONY: all files clean


