FROM docker.io/paritytech/ci-linux:production as builder
LABEL description="This is the build stage for diamond. Here we create the binary."

ARG PROFILE=release
WORKDIR /diamond

COPY . /diamond

RUN cargo build --$PROFILE

# ===== SECOND STAGE ======

FROM docker.io/library/ubuntu:20.04
LABEL description="This is the 2nd stage: a very small image where we copy the diamond binary."
ARG PROFILE=release
COPY --from=builder /diamond/target/$PROFILE/diamond /usr/local/bin

RUN useradd -m -u 1000 -U -s /bin/sh -d /diamond diamond && \
	mkdir -p /diamond/.local/share && \
	mkdir /data && \
	chown -R diamond:diamond /data && \
	ln -s /data /diamond/.local/share/diamond && \
	rm -rf /usr/bin /usr/sbin

USER diamond
EXPOSE 30333 9933 9944
VOLUME ["/data"]

CMD ["/usr/local/bin/diamond"]
