FROM ubuntu:22.04 AS builder
WORKDIR /root/
RUN \
      apt-get update && \
      apt-get install -y llvm-15 llvm-15-tools clang-15 lld-15\
      build-essential libglib2.0-dev python3 curl git cargo ruby3.0\
      cmake autoconf wget make pkgconf libbz2-dev \
      libffi-dev liblzma-dev libsqlite3-dev libssl-dev lzma lzma-dev zlib1g-dev 

RUN git clone https://chromium.googlesource.com/chromium/tools/depot_tools.git
ENV PATH=/root/depot_tools:$PATH
RUN mkdir -p standalone_v8
WORKDIR /root/standalone_v8
RUN fetch v8
WORKDIR /root/standalone_v8/v8
RUN gn gen out/d8 --args=""
WORKDIR /root/standalone_v8/v8/out/d8
RUN ninja
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
RUN cp -r ruby-wasm32-wasi/ /root/rb-wasi-sandbox
RUN cp /root/standalone_v8/v8/out/d8/d8 /root/rb-wasi-sandbox
RUN cp /root/standalone_v8/v8/out/d8/lib*.so /root/rb-wasi-sandbox
RUN cp /root/standalone_v8/v8/out/d8/*.bin /root/rb-wasi-sandbox
WORKDIR /root/
RUN git clone https://github.com/UsQuake/wasi_sandbox_generator.git
WORKDIR /root/wasi_sandbox_generator
RUN cargo run rb
FROM ubuntu:22.04
WORKDIR /root/rb-wasi-sandbox
COPY --from=builder /root/rb-wasi-sandbox .
CMD ["tail", "-f", "/dev/null"]
