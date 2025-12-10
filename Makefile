update:
	git pull
	cargo build --release
	cp ./target/release/minecraft-console .