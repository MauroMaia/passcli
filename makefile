GREEN=\033[1;32m
NC=\033[0m

all: create_rpm create_deb
	@echo "Done"

build-release: test
	@echo -e "$(GREEN)$$(date --iso=s) - Next step -> Building the release version $(NC)"
	cargo build --release

test:
	@echo -e "$(GREEN)$$(date --iso=s) - Next step -> Let's run some tests $(NC)"
	cargo test

create_rpm: clean test doc
	@echo -e "$(GREEN)$$(date --iso=s) - Next step -> build rpm for x86_64-unknown-linux-gnu $(NC)"
	#cargo rpm build -v --target aarch64-unknown-linux-gnu
	cargo rpm build --target x86_64-unknown-linux-gnu

create_deb: clean test doc
	@echo -e "$(GREEN)$$(date --iso=s) - Next step -> build deb $(NC)"
	cargo deb

local_x86-64_rpm_install: create_rpm
	@echo -e "$(GREEN)$$(date --iso=s) - Next step -> install... $(NC)"
	find ./target -name "*pass*x86_64.rpm" -exec sudo dnf install {} \;

local_x86-64_rpm_reinstall: create_rpm
	@echo -e "$(GREEN)$$(date --iso=s) - Next step -> reinstall... $(NC)"
	find ./target -name "*pass*x86_64.rpm" -exec sudo dnf reinstall {} \;

publish:
	@echo -e "$(GREEN)$$(date --iso=s) - Next step -> publishing... $(NC)"
	basename=$(find ./target -name "*pass*x86_64.rpm" -exec basename {} \;)
	find ./target -name "*pass*x86_64.rpm" -exec scp {} maurofilipemaia.dev:~/domains/passcli.maurofilipemaia.dev/public_html/rpms \;
	find ./target -name "*pass*x86_64.rpm" -exec ssh maurofilipemaia.dev ln -nsf ~/domains/passcli.maurofilipemaia.dev/public_html/rpms/$(basename {} ) ~/domains/passcli.maurofilipemaia.dev/public_html/rpms/latest
	@echo -e "$(GREEN)$$(date --iso=s) - Next step -> publishing done $(NC)"

clean:
	@echo -e "$(GREEN)$$(date --iso=s) - Next step -> cleaning... $(NC)"
	cargo clean
	[ -d target ] && rm -rf target || true

doc:
	@echo -e "$(GREEN)$$(date --iso=s) - Next step -> creating docs.... $(NC)"
	cargo doc
