CC=cargo
APP=byakugan
TMP=./tmp

.PHONY: default help
default: help
help:
	@echo "Usage: make <target>"
	@echo ""
	@echo "Targets:"
	@echo "  codegen  - Generate entities code"
	@echo "  debug    - Run the debug project"
	@echo "  help     - Show this help"

.PHONY: codegen
codegen:
	@echo "[+] Generating entities code..."
	sea-orm-cli generate entity -u "sqlite://$(TMP)/$(APP).db" -o "$(TMP)/entities" 

.PHONY: debug
debug:
	@echo "[+] Running the debug project..."
	@$(CC) run -- console -D "sqlite://$(TMP)/$(APP).db" --log-file "$(TMP)/$(APP).log" -vv 