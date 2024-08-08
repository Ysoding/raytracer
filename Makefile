
.PHONY: install
install:
	sudo apt-get install feh

.PHONY: feh
open:
	feh $(ARGS)


.PHONY: generate
generate:
	cargo run > image.ppm

