.PHONY: problem
problem:
	@if [ -z "$(DAY)" ]; then \
		echo "usage: make problem DAY=dXX"; exit 1; \
	fi
	mkdir -p $(DAY)
	cd $(DAY); cargo init --vcs none
	@printf '\n[lints.rust]\nwarnings = "deny"\n\n[lints.clippy]\nall = "deny"\npedantic = "deny"\nnursery = "warn"\n' >> $(DAY)/Cargo.toml
