image:
	@docker build -f dockerfiles/dev -t smart-fluid-flow-meter-sketch-dev-image .
.PHONY: image

cmake: image
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
			rm -f /sketch/compile_commands.json \
		"
.PHONY: cmake

build: cmake
	@docker run -it --rm \
		-w /sketch/build/ \
		-v $(PWD)/build:/sketch/build \
		-v $(PWD)/CMakeLists.txt:/sketch/CMakeLists.txt \
		-v $(PWD)/src:/sketch/src \
		smart-fluid-flow-meter-sketch-dev-image \
		sh -c "\
			make \
		"
.PHONY: build

upload: build
	@docker run --rm --device /dev/ttyACM0:/dev/ttyACM0 \
		-w /sketch/build/ \
		-v $(PWD)/build:/sketch/build \
		-v $(PWD)/CMakeLists.txt:/sketch/CMakeLists.txt \
		-v $(PWD)/src:/sketch/src \
		-v $(PWD)/test:/sketch/test \
		smart-fluid-flow-meter-sketch-dev-image \
		sh -c " \
			stty -F /dev/ttyACM0 1200 && \
			/root/.arduino15/packages/arduino/tools/bossac/1.9.1-arduino5/bossac \
			-a -d --port=ttyACM0 -U -e -w 'main.bin' -R \
		"
.PHONY: upload

serial:
	@docker run --rm -it -v $(PWD)/src/:/sketch/src \
		-v $(PWD)/test/:/sketch/test \
		-w /sketch/ \
		--device /dev/ttyACM0:/dev/ttyACM0 \
		smart-fluid-flow-meter-sketch-dev-image \
		sh -c "stty 9600 -F /dev/ttyACM0 raw -echo && cat /dev/ttyACM0"
.PHONY: serial

ssh:
	@docker run --rm -it -v $(PWD)/src/:/sketch/src \
		-v $(PWD)/test/:/sketch/test \
		-w /sketch/ smart-fluid-flow-meter-sketch-dev-image bash
.PHONY: ssh

format:
	@docker run --rm -it -v $(PWD)/src/:/sketch/src \
		-v $(PWD)/test/:/sketch/test \
		-w /sketch/ \
		smart-fluid-flow-meter-sketch-dev-image \
		sh -c "find src/ test/src/ test/include/ -iname '*.h' -o -iname '*.cpp' | xargs clang-format --style=Chromium -i"
.PHONY: format

check-format:
	@docker run --rm -it \
		-v $(PWD)/src/:/sketch/src \
		-v $(PWD)/test/:/sketch/test \
		-w /sketch/ \
		smart-fluid-flow-meter-sketch-dev-image \
		sh -c "find src/ test/src/ test/include/ -iname '*.h' -o -iname '*.cpp' | xargs clang-format --style=Chromium --dry-run -Werror"
.PHONY: check-format

cmake_test: image
	@docker run -it --rm \
		-w /sketch/test/build/ \
		-v $(PWD)/test/build:/sketch/test/build \
		-v $(PWD)/test/CMakeLists.txt:/sketch/test/CMakeLists.txt \
		-v $(PWD)/src:/sketch/src \
		-v $(PWD)/test/src:/sketch/test/src \
		-v $(PWD)/test/include:/sketch/test/include \
		-v $(PWD)/test/external:/sketch/test/external \
		smart-fluid-flow-meter-sketch-dev-image \
		sh -c " \
			cmake -D CMAKE_EXPORT_COMPILE_COMMANDS=1 .. && \
			rm -f /sketch/compile_commands.json \
		"
.PHONY: cmake_test

test: cmake_test
	@docker run -it --rm \
		-w /sketch/test/build/ \
		-v $(PWD)/test/build:/sketch/test/build \
		-v $(PWD)/test/CMakeLists.txt:/sketch/test/CMakeLists.txt \
		-v $(PWD)/src:/sketch/src \
		-v $(PWD)/test/src:/sketch/test/src \
		-v $(PWD)/test/include:/sketch/test/include \
		-v $(PWD)/test/external:/sketch/test/external \
		smart-fluid-flow-meter-sketch-dev-image \
		sh -c " \
			make && \
			./test \
		"
.PHONY: test

verify: image build check-format test
.PHONY: verify

# Starts a container with a neovim development environment ready to use
vim: cmake cmake_test
	@docker build -f dockerfiles/dev-vim -t smart-fluid-flow-meter-sketch-dev-vim-image .
	@docker run --rm -it \
		-v $(PWD)/build:/sketch/build \
		-v $(PWD)/CMakeLists.txt:/sketch/CMakeLists.txt \
		-v $(PWD)/README.md:/sketch/README.md \
		-v $(PWD)/Makefile:/sketch/Makefile \
		-v $(PWD)/src:/sketch/src \
		-v $(PWD)/test:/sketch/test \
		-v $(PWD)/dockerfiles:/sketch/dockerfiles \
		-v $(PWD)/dev-environments/vim/tmp:/root/.local/share/nvim \
		-w /sketch/ \
		smart-fluid-flow-meter-sketch-dev-vim-image \
		sh -c "ln -s /sketch/build/compile_commands.json /sketch/ && ln -s /sketch/test/build/compile_commands.json /sketch/test/ && nvim"
.PHONY: vim
