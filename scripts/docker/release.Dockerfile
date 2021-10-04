FROM docker.io/library/ubuntu:20.04

# metadata
ARG VCS_REF
ARG BUILD_DATE
ARG diamond_VERSION

LABEL io.parity.image.authors="devops-team@parity.io" \
	io.parity.image.vendor="Parity Technologies" \
	io.parity.image.title="parity/diamond" \
	io.parity.image.description="diamond: a platform for web3" \
	io.parity.image.source="https://github.com/paritytech/Polkadot/blob/${VCS_REF}/scripts/docker/Dockerfile" \
	io.parity.image.revision="${VCS_REF}" \
	io.parity.image.created="${BUILD_DATE}" \
	io.parity.image.documentation="https://github.com/paritytech/Polkadot/"

# show backtraces
ENV RUST_BACKTRACE 1

# install tools and dependencies
RUN apt-get update && \
	DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends \
		libssl1.1 \
		ca-certificates \
		curl \
		gnupg && \
	useradd -m -u 1000 -U -s /bin/sh -d /diamond diamond && \
# add repo's gpg keys and install the published diamond binary
	gpg --recv-keys --keyserver hkps://keys.mailvelope.com 9D4B2B6EB8F97156D19669A9FF0812D491B96798 && \
	gpg --export 9D4B2B6EB8F97156D19669A9FF0812D491B96798 > /usr/share/keyrings/parity.gpg && \
	echo 'deb [signed-by=/usr/share/keyrings/parity.gpg] https://releases.parity.io/deb release main' > /etc/apt/sources.list.d/parity.list && \
	apt-get update && \
	apt-get install -y --no-install-recommends diamond=${diamond_VERSION#?} && \
# apt cleanup
	apt-get autoremove -y && \
	apt-get clean && \
	rm -rf /var/lib/apt/lists/* ; \
	mkdir -p /data /diamond/.local/share && \
	chown -R diamond:diamond /data && \
	ln -s /data /diamond/.local/share/diamond

USER diamond

# check if executable works in this container
RUN /usr/bin/diamond --version

EXPOSE 30333 9933 9944
VOLUME ["/diamond"]

ENTRYPOINT ["/usr/bin/diamond"]
