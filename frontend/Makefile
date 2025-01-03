image:
	@docker build -f dockerfiles/dev -t smart-fluid-flow-meter-frontend-image .
.PHONY: image

image-prod: build
	@docker build -f dockerfiles/prod -t console-prod .
.PHONY: image-prod

install: image
	@docker run --rm -it \
		-v $(PWD)/svelte-app:/svelte-app \
		-w /svelte-app/ \
		smart-fluid-flow-meter-frontend-image \
		npm install
.PHONY: install

build: install
	@docker run --rm -it \
		--env-file=.env \
		-v $(PWD)/svelte-app:/svelte-app \
		-w /svelte-app/ \
		smart-fluid-flow-meter-frontend-image \
		npm run build
.PHONY: build

start: install
	@docker run --rm -it -p 5173:5173 \
		--env-file=.env \
		-v $(PWD)/svelte-app:/svelte-app \
		-w /svelte-app/ \
		smart-fluid-flow-meter-frontend-image
.PHONY: start

start-prod: image-prod
	@docker run --rm -it -p 5173:5173 \
		--env-file=.env \
		--name smart-fluid-flow-meter-frontend-prod \
		console-prod
.PHONY: start-prod

ssh: image
	@docker run --rm -it -p 5173:5173 \
		--env-file=.env \
		-v $(PWD)/svelte-app:/svelte-app \
		-w /svelte-app/ \
		smart-fluid-flow-meter-frontend-image \
		bash
.PHONY: ssh

check-format: install
	@docker run --rm -it \
		--env-file=.env \
		-v $(PWD)/svelte-app:/svelte-app \
		-w /svelte-app/ \
		smart-fluid-flow-meter-frontend-image \
		npm run lint
.PHONY: check-format

format: install
	@docker run --rm -it \
		--env-file=.env \
		-v $(PWD)/svelte-app:/svelte-app \
		-w /svelte-app/ \
		smart-fluid-flow-meter-frontend-image \
		npm run format
.PHONY: format

check-svelte: install
	@docker run --rm -it \
		--env-file=.env \
		-v $(PWD)/svelte-app:/svelte-app \
		-w /svelte-app/ \
		smart-fluid-flow-meter-frontend-image \
		npm run check
.PHONY: svelte-check

verify: build check-format check-svelte
.PHONY: verify

# Starts a container with a neovim development environment ready to use
vim: image
	@docker build -f dockerfiles/dev-vim -t smart-fluid-flow-meter-frontend-vim-image .
	@docker run --rm -it \
		-v $(PWD)/svelte-app:/svelte-app \
		-v $(PWD)/dev-environments/vim/tmp:/root/.local/share/nvim \
		-w /svelte-app/ \
		smart-fluid-flow-meter-frontend-vim-image \
		sh -c "nvim"
.PHONY: vim
