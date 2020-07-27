
FORMAT=svg

FDP_FILES=clustered.dot
FDP_IMAGES=$(FDP_FILES:%.dot=%.$(FORMAT))

DOT_FILES=linked.dot
DOT_IMAGES=$(DOT_FILES:%.dot=%.$(FORMAT))


all: $(FDP_IMAGES) $(DOT_IMAGES)


$(FDP_IMAGES): %.svg: %.dot
	fdp -T$(FORMAT) $^ > $@

$(DOT_IMAGES): %.svg: %.dot
	dot -T$(FORMAT) $^ > $@


.PHONY: clean
clean:
	rm -f *.svg
