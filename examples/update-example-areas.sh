for area in $(fd --type d --max-depth 1 .); do
	echo $area
	node ~/Projects/gobbldygook/modules/gob-hanson-format/cli.js \
		< "$area/source.yaml" \
		> "$area/source.json"
done
