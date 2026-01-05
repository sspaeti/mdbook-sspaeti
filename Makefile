.DEFAULT_GOAL := build

# Version to upgrade to (override with: make upgrade VERSION=v0.5.3)
VERSION ?= v0.5.2

build:
	cargo build --release
	mv target/release/mdbook-sspaeti /usr/local/bin/mdbook-sspaeti


build-linux:
	cargo build --release
	mv target/release/mdbook-sspaeti ~/.local/bin/mdbook-sspaeti

upgrade-dry: ## Dry run to see what would conflict when merging upstream version
	@echo "Fetching upstream mdBook repository..."
	git fetch mdbook --tags
	@echo "\nAttempting dry-run merge of $(VERSION)..."
	@echo "This will NOT commit anything, just show you what would change/conflict.\n"
	git merge $(VERSION) --no-commit --no-ff || true
	@echo "\n--- Merge preview complete ---"
	@echo "Check 'git status' and 'git diff --cached' to review changes"
	@echo "Run 'git merge --abort' to cancel this dry run"

upgrade: ## Fetch latest stable release and merge into a new branch
	@echo "Fetching upstream mdBook repository..."
	git fetch mdbook --tags
	@echo "\nEnsuring we're on main branch..."
	git checkout main
	@echo "\nCreating merge branch: merge-$(VERSION)..."
	git checkout -b merge-$(VERSION)
	@echo "\nMerging $(VERSION) from upstream..."
	git merge $(VERSION) --no-ff -m "Merge upstream mdBook $(VERSION)"
	@echo "\n--- Merge complete ---"
	@echo "Review the changes with: git log --oneline --graph -20"
	@echo "If everything looks good:"
	@echo "  git checkout main"
	@echo "  git merge merge-$(VERSION)"
	@echo "  git push origin main"
	@echo "\nIf you need to abort:"
	@echo "  git checkout main"
	@echo "  git branch -D merge-$(VERSION)"

# build: prepare run

