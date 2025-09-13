# ===== Voting DApp Makefile =====
# usage:
#   make deploy               # default ke ENV=sandbox
#   make deploy ENV=production
#   make run                  # jalanin backend + frontend lokal

SHELL := /bin/bash
ENV ?= sandbox

# -------- Paths --------
WS_DIR          := onchain
ARTIFACTS_DIR   := $(WS_DIR)/artifacts
WASM_FILE       := $(ARTIFACTS_DIR)/voting_cw20.wasm

DEPLOY_TS_DIR   := deploy/ts
UPLOAD_SCRIPT   := $(DEPLOY_TS_DIR)/scripts/upload.ts
INSTANT_SCRIPT  := $(DEPLOY_TS_DIR)/scripts/instantiate.ts
SETENV_SCRIPT   := $(DEPLOY_TS_DIR)/scripts/set_env_from_deploy.ts

FE_DIR          := frontend
BE_DIR          := backend

# -------- Helpers --------
define load_env
	@if [ -f config/$(ENV)/chain.env ]; then set -a; source config/$(ENV)/chain.env; set +a; fi; \
	if [ -f config/$(ENV)/contracts.env ]; then set -a; source config/$(ENV)/contracts.env; set +a; fi; \
	if [ -f config/$(ENV)/backend.env ]; then set -a; source config/$(ENV)/backend.env; set +a; fi; \
	if [ -f config/$(ENV)/frontend.env ]; then set -a; source config/$(ENV)/frontend.env; set +a; fi;
endef

# -------- Phony --------
.PHONY: all build-wasm upload instantiate set-env deploy migrate run run-backend run-frontend fe-dev be-dev clean

all: deploy

## 1) Build CosmWasm (butuh rust-optimized toolchain di folder onchain/)
build-wasm:
	@echo "==> Building CosmWasm contract"
	@cd $(WS_DIR) && make optimize
	@mkdir -p $(ARTIFACTS_DIR)
	@# pastikan file wasm berada di $(WASM_FILE)
	@if [ ! -f "$(WASM_FILE)" ]; then echo "Note: rename/copy wasm ke $(WASM_FILE)"; fi

## 2) Upload wasm → dapat code_id (disimpan di onchain/artifacts/code_id.txt)
upload:
	@echo "==> Uploading WASM to $(ENV)"
	@$(load_env)
	@cd $(DEPLOY_TS_DIR) && npx ts-node $(UPLOAD_SCRIPT) ../../$(WASM_FILE)

## 3) Instantiate code_id → dapat contract address (disimpan artifacts/contract_address.txt)
instantiate:
	@echo "==> Instantiating contract on $(ENV)"
	@$(load_env)
	@cd $(DEPLOY_TS_DIR) && npx ts-node $(INSTANT_SCRIPT)

## 4) Sinkronisasi .env frontend/backend + config/contracts.env
set-env:
	@echo "==> Writing contract address to env files ($(ENV))"
	@cd $(DEPLOY_TS_DIR) && npx ts-node $(SETENV_SCRIPT)

## Shortcut: 2–4
deploy: upload instantiate set-env
	@echo "✅ Deploy done for $(ENV)"

## (Opsional) SQLX migrations (MySQL)
migrate:
	@echo "==> Running sqlx migrations (backend)"
	@cd $(BE_DIR) && sqlx migrate run

## Jalankan backend & frontend (dev)
run: run-backend run-frontend

run-backend:
	@echo "==> Backend (dev)"
	@cd $(BE_DIR) && RUST_LOG=info cargo run

run-frontend:
	@echo "==> Frontend (dev)"
	@cd $(FE_DIR) && npm run dev

fe-dev: run-frontend
be-dev: run-backend

clean:
	@echo "==> Clean artifacts"
	@rm -f $(ARTIFACTS_DIR)/code_id.txt $(ARTIFACTS_DIR)/contract_address.txt $(ARTIFACTS_DIR)/upload_result.json
