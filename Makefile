loc:
	find src tests mm-demo/src -name '*.rs' | xargs wc -l

demo:
	cargo run -p mm-demo

gitaddall:
	git add src protocols
