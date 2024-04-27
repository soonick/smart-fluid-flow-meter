build:
	@docker build -f dockerfiles/dev -t smart-fluid-flow-meter-sketch-image .
.PHONY: build

upload:
	@docker run --rm -v $(PWD)/:/sketch/ -w /sketch/ --device /dev/ttyACM0:/dev/ttyACM0 smart-fluid-flow-meter-sketch-image make -f ArduinoMakefile upload
.PHONY: upload

serial:
	@docker run --rm -it -v $(PWD)/:/sketch/ -w /sketch/ --device /dev/ttyACM0:/dev/ttyACM0 smart-fluid-flow-meter-sketch-image sh -c "stty 9600 -F /dev/ttyACM0 raw -echo && cat /dev/ttyACM0"
.PHONY: serial

ssh:
	@docker run --rm -it -v $(PWD)/:/sketch/ -w /sketch/ smart-fluid-flow-meter-sketch-image bash
.PHONY: ssh

format:
	@docker run --rm -it -v $(PWD)/:/sketch/ -w /sketch/ smart-fluid-flow-meter-sketch-image clang-format *.ino -style=Chromium -i
.PHONY: format

check-format:
	@docker run --rm -it -v $(PWD)/:/sketch/ -w /sketch/ smart-fluid-flow-meter-sketch-image clang-format *.cpp -style=Chromium --dry-run -Werror
.PHONY: check-format

verify: build check-format
.PHONY: verify
