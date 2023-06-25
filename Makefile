build-worker:
	worker-build --release

publish:
	wrangler publish

run-worker: build-worker
	npx workerd serve my-config.capnp
