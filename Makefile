
FORMAT=svg

DOT_FILES=$(wildcard *.dot)
IMAGES=$(DOT_FILES:%.dot=%.$(FORMAT))


all: $(IMAGES)


$(IMAGES): %.svg: %.dot
	fdp -T$(FORMAT) $^ > $@


.PHONY: clean
clean:
	rm -f *.svg
