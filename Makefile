.PHONY: problem
problem:
	@if [ -z "$(DAY)" ]; then \
		echo "usage: make problem DAY=dXX"; exit 1; \
	fi
	mkdir -p $(DAY)
	cd $(DAY); cargo init --vcs none
