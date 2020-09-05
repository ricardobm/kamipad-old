.PHONY: run serve

run:
	@cd kamipad-app; npm start

serve:
	@cd kamipad-server; cargo run
