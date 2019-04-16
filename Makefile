build-gh-pages:
	rm -rf docs
	mkdir docs
	cp static/index.html docs
	cp pkg/d0.js docs/pkg
	cp pkg/d0_bg.wasm docs/pkg

build-static:
	cp -r pkg static/pkg