build-gh-pages:
	rm -rf docs
	mkdir docs
	cp static/index.html docs
	cp static/bootstrap.js docs
	cp static/main.js docs
	cp pkg/d0.js docs
	cp pkg/d0_bg.wasm docs
	sed -i '' 's/..\/pkg\//.\//' docs/main.js
