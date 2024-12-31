image:
	@docker build -f dockerfiles/dev -t smart-fluid-flow-meter-frontend-image .
.PHONY: image

install: image
	@docker run --rm -it \
		-v $(PWD)/svelte-app:/svelte-app \
		-w /svelte-app/ \
		smart-fluid-flow-meter-frontend-image \
		npm install
.PHONY: install


build: install
	@docker run --rm -it \
		-v $(PWD)/svelte-app:/svelte-app \
		-w /svelte-app/ \
		smart-fluid-flow-meter-frontend-image \
		npm run build
.PHONY: build

start: install
	@docker run --rm -it -p 5173:5173 \
		-v $(PWD)/svelte-app:/svelte-app \
		-w /svelte-app/ \
		smart-fluid-flow-meter-frontend-image
.PHONY: start

ssh: image
	@docker run --rm -it -p 5173:5173 \
		-v $(PWD)/svelte-app:/svelte-app \
		-w /svelte-app/ \
		smart-fluid-flow-meter-frontend-image \
		sh
.PHONY: ssh

check-format: install
	@docker run --rm -it \
		-v $(PWD)/svelte-app:/svelte-app \
		-w /svelte-app/ \
		smart-fluid-flow-meter-frontend-image \
		npm run lint
.PHONY: check-format

format: install
	@docker run --rm -it \
		-v $(PWD)/svelte-app:/svelte-app \
		-w /svelte-app/ \
		smart-fluid-flow-meter-frontend-image \
		npm run format
.PHONY: format

check-svelte: install
	@docker run --rm -it \
		-v $(PWD)/svelte-app:/svelte-app \
		-w /svelte-app/ \
		smart-fluid-flow-meter-frontend-image \
		npm run check
.PHONY: svelte-check

verify: build check-format check-svelte
.PHONY: verify
