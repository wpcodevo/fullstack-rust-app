start-app:
	trunk serve --port 3000

start-server:
	cargo watch -q -c -w src/ -x run