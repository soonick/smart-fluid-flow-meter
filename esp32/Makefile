image:
	@docker build -f dockerfiles/dev -t smart-fluid-flow-meter-esp32-image .
.PHONY: image

build: image
	@docker run -it --rm \
		-v $(PWD)/src:/esp32 \
		-w /esp32 \
		smart-fluid-flow-meter-esp32-image \
		idf.py build
.PHONY: build

upload: build
	@docker run --rm --device /dev/ttyUSB0:/dev/ttyUSB0 \
		-v $(PWD)/src:/esp32 \
		-w /esp32 \
		smart-fluid-flow-meter-esp32-image \
		idf.py -p /dev/ttyUSB0 flash
.PHONY: upload

serial:
	@docker run -it --rm --device /dev/ttyUSB0:/dev/ttyUSB0 \
		-v $(PWD)/src:/esp32 \
		-w /esp32 \
		smart-fluid-flow-meter-esp32-image \
		idf.py -p /dev/ttyUSB0 monitor
.PHONY: serial

ssh:
	@docker run -it --rm \
		--device /dev/ttyUSB0:/dev/ttyUSB0 \
		-v $(PWD)/src:/esp32 \
		-w /esp32 \
		smart-fluid-flow-meter-esp32-image \
		bash
.PHONY: ssh

format: build
	@docker run -it --rm \
		-v $(PWD)/src:/esp32 \
		-w /esp32 \
		smart-fluid-flow-meter-esp32-image \
		sh -c "find main/ -iname '*.hpp' -o -iname '*.cpp' | xargs clang-format --style=Chromium -i"
.PHONY: format

check-format: build
	@docker run -it --rm \
		-v $(PWD)/src:/esp32 \
		-w /esp32 \
		smart-fluid-flow-meter-esp32-image \
		sh -c "find main/ -iname '*.hpp' -o -iname '*.cpp' | xargs clang-format --style=Chromium --dry-run -Werror"
.PHONY: check-format

verify: build check-format
.PHONY: verify

# Starts a container with a neovim development environment ready to use
vim: build
	@docker build -f dockerfiles/dev-vim -t smart-fluid-flow-meter-esp32-vim-image .
	@docker run --rm -it \
		-v $(PWD)/dev-environments/vim/tmp:/root/.local/share/nvim \
		-v $(PWD)/src:/esp32 \
		-w /esp32 \
		smart-fluid-flow-meter-esp32-vim-image \
		nvim
.PHONY: vim
