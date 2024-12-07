build:
	@docker build -f dockerfiles/dev -t smart-fluid-flow-meter-frontend-image .
.PHONY: build

start: build
	@docker run --rm -it -p 5173:5173 \
		-v $(PWD)/svelte-app:/svelte-app \
		-w /svelte-app/ \
		--name smart-fluid-flow-meter-frontend-dev \
		smart-fluid-flow-meter-frontend-image
.PHONY: start
