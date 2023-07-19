d:
	cargo doc

t:
	cargo test 

tf:
	cargo test --features unchecked_mut,into_precompiled

a:
	-- --show-output

b:
	cargo build
