build:
	@mkdir -p build/target
	@mkdir -p build/registry
	@docker build -f dockerfiles/dev -t smart-fluid-flow-meter-backend-image .
.PHONY: build

build-prod: build
	@docker run --rm \
		-v $(PWD)/Cargo.lock:/smart-fluid-flow-meter/Cargo.lock \
		-v $(PWD)/Cargo.toml:/smart-fluid-flow-meter/Cargo.toml \
		-v $(PWD)/assets/:/assets \
		-v $(PWD)/build/target:/smart-fluid-flow-meter/target \
		-v $(PWD)/src:/smart-fluid-flow-meter/src \
		-w /smart-fluid-flow-meter/ \
		smart-fluid-flow-meter-backend-image \
		cargo build --release
	@docker build -f dockerfiles/prod -t smart-fluid-flow-meter-backend-image-prod .
.PHONY: build-prod

ssh: clean
	@docker run --rm -it -p 3000:3000 -v $(PWD)/:/smart-fluid-flow-meter/ --name smart-fluid-flow-meter-backend-dev smart-fluid-flow-meter-backend-image bash
.PHONY: ssh

ssh-prod: build-prod
	@docker run --rm -it -p 3000:3000 \
		--name smart-fluid-flow-meter-backend-prod-ssh \
		smart-fluid-flow-meter-backend-image-prod \
		bash
.PHONY: ssh-prod

start:
	@docker run --rm -it -p 3000:3000 \
		-v $(PWD)/Cargo.lock:/smart-fluid-flow-meter/Cargo.lock \
		-v $(PWD)/Cargo.toml:/smart-fluid-flow-meter/Cargo.toml \
		-v $(PWD)/assets/:/assets \
		-v $(PWD)/build/registry:/usr/local/cargo/registry \
		-v $(PWD)/build/target:/smart-fluid-flow-meter/target \
		-v $(PWD)/src:/smart-fluid-flow-meter/src \
		-w /smart-fluid-flow-meter/ \
		--name smart-fluid-flow-meter-backend-dev \
		smart-fluid-flow-meter-backend-image \
		cargo run
.PHONY: start

start-prod:
	@docker run --rm -it -p 3000:3000 \
		--name smart-fluid-flow-meter-backend-prod \
		smart-fluid-flow-meter-backend-image-prod \
		./smart_fluid_flow_meter_backend
.PHONY: start-prod

start-mysql:
	@docker compose -f dockerfiles/docker-compose-mysql.yaml down --remove-orphans
	@docker compose -f dockerfiles/docker-compose-mysql.yaml up --build --abort-on-container-exit
.PHONY: start-mysql

test:
	@docker compose -f dockerfiles/docker-compose-test.yaml down --remove-orphans
	@docker compose -f dockerfiles/docker-compose-test.yaml up --build \
		--abort-on-container-exit \
		--exit-code-from smart-fluid-flow-meter-backend
.PHONY: test

format:
	@docker run --rm -v $(PWD)/:/smart-fluid-flow-meter/ -w /smart-fluid-flow-meter/ smart-fluid-flow-meter-backend-image cargo fmt
.PHONY: format

check-format:
	@docker run --rm -v $(PWD)/:/smart-fluid-flow-meter/ -w /smart-fluid-flow-meter/ smart-fluid-flow-meter-backend-image cargo fmt --check
.PHONY: check-format

verify: clean build build-prod check-format test
.PHONY: verify

clean:
	-@docker kill smart-fluid-flow-meter-backend-dev 2>/dev/null ||:
	-@docker rm smart-fluid-flow-meter-backend-dev 2>/dev/null ||:
	-@docker kill smart-fluid-flow-meter-backend-prod 2>/dev/null ||:
	-@docker rm smart-fluid-flow-meter-backend-prod 2>/dev/null ||:
.PHONY: clean
