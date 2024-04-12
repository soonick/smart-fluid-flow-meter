build:
	@docker build -f dockerfiles/dev -t smart-fluid-flow-meter-backend-image .
	@docker run --rm -v $(PWD)/:/smart-fluid-flow-meter/ -w /smart-fluid-flow-meter/ smart-fluid-flow-meter-backend-image cargo build
.PHONY: build

ssh: clean
	@docker run -it -p 3000:3000 -v $(PWD)/:/smart-fluid-flow-meter/ --name smart-fluid-flow-meter-backend-dev smart-fluid-flow-meter-backend-image bash
.PHONY: ssh

start: clean build
	@docker run -it -p 3000:3000 -v $(PWD)/:/smart-fluid-flow-meter/ -w /smart-fluid-flow-meter/ --name smart-fluid-flow-meter-backend-dev smart-fluid-flow-meter-backend-image cargo run
.PHONY: start

clean:
	-@docker kill smart-fluid-flow-meter-backend-dev 2>/dev/null ||:
	-@docker rm smart-fluid-flow-meter-backend-dev 2>/dev/null ||:
	-@docker kill smart-fluid-flow-meter-backend-prod 2>/dev/null ||:
	-@docker rm smart-fluid-flow-meter-backend-prod 2>/dev/null ||:
.PHONY: clean
