FROM debian:buster

WORKDIR /root/

RUN apt-get update && \
	apt-get install -y make build-essential curl && \
	rm -rf /var/lib/apt/lists/*

RUN curl https://sh.rustup.rs -sSf | bash -s -- -y
ENV PATH=/root/.cargo/bin:$PATH

# Build a cache of deps
ADD ["Cargo.*", "./"]
# RUN cargo build && cargo build --release
RUN mkdir -p src && touch src/lib.rs && cargo fetch

ADD . ./

RUN make && make install
RUN sed -i '/dns/c\hosts:	files zerotier dns' /etc/nsswitch.conf

ENTRYPOINT ["getent", "-s", "zerotier", "hosts"]
