image:
	@docker build -f dockerfiles/dev -t smart-fluid-flow-meter-sketch-dev-image .
.PHONY: image

build: image
	@docker run -it --rm \
		-w /sketch/build/ \
		-v $(PWD)/build:/sketch/build \
		-v $(PWD)/CMakeLists.txt:/sketch/CMakeLists.txt \
		-v $(PWD)/src:/sketch/src \
		smart-fluid-flow-meter-sketch-dev-image \
		sh -c "\
			rm -f BoardOptions.cmake && \
			echo 'set(ARDUINO_BOARD \"Arduino UNO R4 WiFi [renesas_uno.unor4wifi]\")' >> BoardOptions.cmake && \
			cmake -D CMAKE_EXPORT_COMPILE_COMMANDS=1 \
				-D CMAKE_TOOLCHAIN_FILE=Arduino-CMake-Toolchain/Arduino-toolchain.cmake \
				-D ARDUINO_INSTALL_PATH=/root/.arduino15 \
				-D ARDUINO_ENABLE_PACKAGE_MANAGER=ON \
				-D ARDUINO_PLATFORM=arduino.renesas_uno .. && \
			rm -f /sketch/compile_commands.json && \
			make \
		"
.PHONY: build

upload: build
	@docker run --rm --device /dev/ttyACM0:/dev/ttyACM0 \
		-w /sketch/build/ \
		-v $(PWD)/build:/sketch/build \
		-v $(PWD)/CMakeLists.txt:/sketch/CMakeLists.txt \
		-v $(PWD)/src:/sketch/src \
		smart-fluid-flow-meter-sketch-dev-image \
		sh -c " \
			stty -F /dev/ttyACM0 1200 && \
			/root/.arduino15/packages/arduino/tools/bossac/1.9.1-arduino5/bossac \
			-a -d --port=ttyACM0 -U -e -w 'main.bin' -R \
		"
.PHONY: upload

serial:
	@docker run --rm -it -v $(PWD)/src/:/sketch/ -w /sketch/ \
		--device /dev/ttyACM0:/dev/ttyACM0 \
		smart-fluid-flow-meter-sketch-dev-image \
		sh -c "stty 9600 -F /dev/ttyACM0 raw -echo && cat /dev/ttyACM0"
.PHONY: serial

ssh:
	@docker run --rm -it -v $(PWD)/src/:/sketch/src -w /sketch/ smart-fluid-flow-meter-sketch-dev-image bash
.PHONY: ssh

format:
	@docker run --rm -it -v $(PWD)/src/:/sketch/ -w /sketch/ \
		smart-fluid-flow-meter-sketch-dev-image \
		sh -c "find . -iname *.h -o -iname *.cpp | xargs clang-format --style=Chromium -i"
.PHONY: format

check-format:
	@docker run --rm -it -v $(PWD)/src/:/sketch/ -w /sketch/ \
		smart-fluid-flow-meter-sketch-dev-image \
		sh -c "find . -iname *.h -o -iname *.cpp | xargs clang-format --style=Chromium --dry-run -Werror"
.PHONY: check-format

verify: image build check-format
.PHONY: verify

# Starts a container with a neovim development environment ready to use
vim: image
	@docker build -f dockerfiles/dev-vim -t smart-fluid-flow-meter-sketch-dev-vim-image .
	@docker run --rm -it \
		-v $(PWD)/build:/sketch/build \
		-v $(PWD)/CMakeLists.txt:/sketch/CMakeLists.txt \
		-v $(PWD)/README.md:/sketch/README.md \
		-v $(PWD)/Makefile:/sketch/Makefile \
		-v $(PWD)/src:/sketch/src \
		-v $(PWD)/dockerfiles:/sketch/dockerfiles \
		-v $(PWD)/dev-environments/vim/tmp:/root/.local/share/nvim \
		-w /sketch/ \
		smart-fluid-flow-meter-sketch-dev-vim-image \
		sh -c "ln -s /sketch/build/compile_commands.json /sketch/ && nvim"
.PHONY: vim
