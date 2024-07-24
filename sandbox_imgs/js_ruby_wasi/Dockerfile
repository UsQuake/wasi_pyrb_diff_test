FROM ubuntu:22.04 AS builder
WORKDIR /root/
RUN \
      apt-get update && \
      apt-get install -y llvm-15 llvm-15-tools clang-15 lld-15\
      build-essential libglib2.0-dev python3 python3-pip curl git ruby3.0\
      cmake autoconf wget make pkgconf libbz2-dev \
      libffi-dev liblzma-dev libsqlite3-dev libssl-dev lzma lzma-dev zlib1g-dev
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs >> install-rustup.sh
RUN sh install-rustup.sh -y
ENV PATH=/root/.cargo/bin:$PATH
RUN python3 -m pip install --user mercurial
RUN curl https://hg.mozilla.org/mozilla-central/raw-file/default/python/mozboot/bin/bootstrap.py -O
ENV PATH=/root/.local/bin:$PATH
RUN python3 bootstrap.py --no-interactive
WORKDIR /root/mozilla-unified
RUN hg up -C central
RUN ./mach bootstrap
RUN echo 'ac_add_options --enable-project=js' > mozconfig
RUN echo 'mk_add_options MOZ_OBJDIR=@TOPSRCDIR@/obj' >> mozconfig
RUN ./mach build
WORKDIR /root/
RUN git clone https://github.com/ruby/ruby.git
ENV WASI_VERSION=22
ENV WASI_VERSION_FULL=${WASI_VERSION}.0
ENV BINARYEN_VERSION=118
RUN wget https://github.com/WebAssembly/wasi-sdk/releases/download/wasi-sdk-${WASI_VERSION}/wasi-sdk-${WASI_VERSION_FULL}-linux.tar.gz
RUN tar xvf wasi-sdk-${WASI_VERSION_FULL}-linux.tar.gz
ENV WASI_SDK_PATH=/root/wasi-sdk-$WASI_VERSION_FULL
ENV WASI_SYSROOT=$WASI_SDK_PATH/share/wasi-sysroot
RUN wget https://github.com/WebAssembly/binaryen/releases/download/version_${BINARYEN_VERSION}/binaryen-version_${BINARYEN_VERSION}-x86_64-linux.tar.gz
RUN tar xvf binaryen-version_${BINARYEN_VERSION}-x86_64-linux.tar.gz
ENV PATH=/root/binaryen-version_${BINARYEN_VERSION}/bin:$PATH
WORKDIR /root/ruby
RUN ruby tool/downloader.rb -d tool -e gnu config.guess config.sub
RUN ./autogen.sh
RUN ./configure LDFLAGS="-Xlinker -zstack-size=16777216" \
  --host wasm32-unknown-wasi \
  --with-destdir=./ruby-wasm32-wasi \
  --with-static-linked-ext \
  --with-ext=ripper,monitor
RUN git clone https://github.com/UsQuake/ruby_patch.git
RUN mv ruby_patch/patch.patch .
RUN patch -p0 < patch.patch
RUN make install
RUN mkdir -p /root/rb-wasi-sandbox
RUN cp -r /root/mozilla-unified/obj/dist/bin/js /root/rb-wasi-sandbox
RUN cp -r ruby-wasm32-wasi/ /root/rb-wasi-sandbox
WORKDIR /root/
RUN git clone https://github.com/UsQuake/wasi_sandbox_generator.git
WORKDIR /root/wasi_sandbox_generator
RUN cargo run rb
FROM ubuntu:22.04
WORKDIR /root/rb-wasi-sandbox
COPY --from=builder /root/rb-wasi-sandbox .
CMD ["tail", "-f", "/dev/null"]
