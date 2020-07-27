
FORMAT=svg

FDP_FILES=clustered.dot apriori_relations.dot

FDP_IMAGES=$(FDP_FILES:%.dot=%.$(FORMAT))

DOT_FILES=linked.dot link_type.dot link_label.dot link_type_2.dot \
multiple_link_types.dot single_attribute_constraint.dot \
inference_square.dot inference_triangle.dot

DOT_IMAGES=$(DOT_FILES:%.dot=%.$(FORMAT))


all: $(FDP_IMAGES) $(DOT_IMAGES)


$(FDP_IMAGES): %.svg: %.dot
	fdp -T$(FORMAT) $^ > $@

$(DOT_IMAGES): %.svg: %.dot
	dot -T$(FORMAT) $^ > $@


.PHONY: clean
clean:
	rm -f *.svg
