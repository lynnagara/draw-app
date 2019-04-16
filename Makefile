build-gh-pages:
	wasm-pack build --target web
	rm -rf docs
	mkdir docs
	cp static/index.html docs
	mkdir docs/pkg
	cp pkg/d0.js docs/pkg
	cp pkg/d0_bg.wasm docs/pkg

build-static:
	wasm-pack build --target web
	cp -r pkg static/pkg