FROM clux/muslrust:stable

RUN curl -L 'https://github.com/rust-lang-nursery/mdBook/releases/download/v0.2.1/mdbook-v0.2.1-x86_64-unknown-linux-musl.tar.gz' | tar zxC /tmp && \
	curl -L 'https://github.com/casey/just/releases/download/v0.4.2/just-v0.4.2-x86_64-unknown-linux-musl.tar.gz' | tar zxC /tmp && \
	mv /tmp/just /usr/local/bin/just && \
	mv /tmp/mdbook /usr/local/bin/mdbook && \
	apt-get update && \
	apt-get install -y --no-install-recommends cmake rsync && \
	rm -rf /var/lib/apt/lists/*
