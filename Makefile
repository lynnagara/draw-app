build-gh-pages:
	wasm-pack build --target web
	rm -rf docs
	mkdir docs
	cp static/index.html docs
	cp -r pkg docs/pkg

build-static:
	wasm-pack build --target web
	cp -r pkg static/pkg