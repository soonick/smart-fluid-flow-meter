build:
	@docker build -f dockerfiles/dev -t smart-fluid-flow-meter-sketch-image .
.PHONY: build

upload:
	@docker run --rm --device /dev/ttyACM0:/dev/ttyACM0 -w /sketch/build/ smart-fluid-flow-meter-sketch-image sh -c "stty -F /dev/ttyACM0 1200 && /root/.arduino15/packages/arduino/tools/bossac/1.9.1-arduino5/bossac -a -d --port=ttyACM0 -U -e -w 'main.bin' -R"
.PHONY: upload

serial:
	@docker run --rm -it -v $(PWD)/src/:/sketch/ -w /sketch/ --device /dev/ttyACM0:/dev/ttyACM0 smart-fluid-flow-meter-sketch-image sh -c "stty 9600 -F /dev/ttyACM0 raw -echo && cat /dev/ttyACM0"
.PHONY: serial

ssh:
	@docker run --rm -it -v $(PWD)/src/:/sketch/src -w /sketch/ smart-fluid-flow-meter-sketch-image bash
.PHONY: ssh

format:
	@docker run --rm -it -v $(PWD)/src/:/sketch/ -w /sketch/ smart-fluid-flow-meter-sketch-image sh -c "find . -iname *.h -o -iname *.cpp | xargs clang-format --style=Chromium -i"
.PHONY: format

check-format:
	@docker run --rm -it -v $(PWD)/src/:/sketch/ -w /sketch/ smart-fluid-flow-meter-sketch-image sh -c "find . -iname *.h -o -iname *.cpp | xargs clang-format --style=Chromium --dry-run -Werror"
.PHONY: check-format

verify: build check-format
.PHONY: verify
