FROM rust:latest

RUN curl -L 'https://github.com/casey/just/releases/download/v0.4.2/just-v0.4.2-x86_64-unknown-linux-musl.tar.gz' | tar zxC /tmp && \
	mv /tmp/just /usr/local/bin/just && \
	cd /tmp && \
	git clone https://github.com/ehuss/mdBook.git --branch update-pulldown-cmark && \
	cd mdBook && \
	cargo build --release && \
	cp target/release/mdbook /usr/local/bin/mdbook && \
	apt-get update && \
	apt-get install -y --no-install-recommends cmake rsync && \
	rm -rf /var/lib/apt/lists/*
VOLUME /code
WORKDIR /code
