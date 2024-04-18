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

test:
	@docker-compose -f dockerfiles/docker-compose-test.yaml down --remove-orphans
	@docker-compose -f dockerfiles/docker-compose-test.yaml up --build --abort-on-container-exit --exit-code-from smart-fluid-flow-meter-backend
.PHONY: test

format:
	@docker run --rm -v $(PWD)/:/smart-fluid-flow-meter/ -w /smart-fluid-flow-meter/ smart-fluid-flow-meter-backend-image cargo fmt
.PHONY: format

check-format:
	@docker run --rm -v $(PWD)/:/smart-fluid-flow-meter/ -w /smart-fluid-flow-meter/ smart-fluid-flow-meter-backend-image cargo fmt --check
.PHONY: check-format

verify: clean build check-format test
.PHONY: verify

clean:
	-@docker kill smart-fluid-flow-meter-backend-dev 2>/dev/null ||:
	-@docker rm smart-fluid-flow-meter-backend-dev 2>/dev/null ||:
.PHONY: clean
