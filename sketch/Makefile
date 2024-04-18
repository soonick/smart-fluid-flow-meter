build:
	@docker build -f dockerfiles/dev -t smart-fluid-flow-meter-sketch-image .
.PHONY: build

upload:
	@docker run --rm -v $(PWD)/:/sketch/ -w /sketch/ --device /dev/ttyACM0:/dev/ttyACM0 smart-fluid-flow-meter-sketch-image arduino-cli upload -p /dev/ttyACM0 --fqbn arduino:renesas_uno:unor4wifi .
.PHONY: upload

serial:
	@docker run --rm -it -v $(PWD)/:/sketch/ -w /sketch/ --device /dev/ttyACM0:/dev/ttyACM0 smart-fluid-flow-meter-sketch-image sh -c "stty 9600 -F /dev/ttyACM0 raw -echo && cat /dev/ttyACM0"
.PHONY: serial
