
.PHONY: install
install:
	sudo apt-get install feh

.PHONY: feh
open: generate
	feh image.ppm


.PHONY: generate
generate:
	cargo run > image.ppm
