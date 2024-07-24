FROM ubuntu:22.04 AS builder
WORKDIR /root/
RUN \
      apt-get update && \
      apt-get install -y llvm-15 llvm-15-tools clang-15 lld-15 \
      build-essential libglib2.0-dev python3 ruby3.0 git cargo \
      cmake ninja-build python-is-python3 curl git\
      autoconf wget make pkgconf libbz2-dev \
      libffi-dev liblzma-dev libsqlite3-dev libssl-dev lzma lzma-dev zlib1g-dev
RUN git clone https://github.com/WebKit/webkit.git
WORKDIR /root/webkit
RUN ./Tools/Scripts/build-webkit --jsc-only
WORKDIR /root/
RUN git clone https://github.com/python/cpython.git
ENV WASI_VERSION=22
ENV WASI_VERSION_FULL=${WASI_VERSION}.0
RUN wget https://github.com/WebAssembly/wasi-sdk/releases/download/wasi-sdk-${WASI_VERSION}/wasi-sdk-${WASI_VERSION_FULL}-linux.tar.gz
RUN tar xvf wasi-sdk-${WASI_VERSION_FULL}-linux.tar.gz
ENV WASI_SDK_PATH=/root/wasi-sdk-$WASI_VERSION_FULL
ENV WASI_SYSROOT=$WASI_SDK_PATH/share/wasi-sysroot
WORKDIR /root/cpython
RUN mkdir -p native-build
WORKDIR /root/cpython/native-build
RUN ../configure
RUN make
ENV PATH=$WASI_SDK_PATH/bin:$PATH
ENV CC=$WASI_SDK_PATH/bin/clang
ENV CPP=$WASI_SDK_PATH/bin/clang-cpp
ENV CXX=$WASI_SDK_PATH/bin/clang++
ENV CFLAGS="--sysroot=$WASI_SYSROOT"
ENV LDFLAGS="--sysroot=$WASI_SYSROOT"
ENV CONFIG_SITE=/root/cpython/Tools/wasm/config.site-wasm32-wasi
ENV AR=$WASI_SDK_PATH/bin/llvm-ar
ENV RANLIB=$WASI_SDK_PATH/bin/llvm-ranlib
ENV PKG_CONFIG_PATH=""
ENV PKG_CONFIG_LIBDIR=$WASI_SYSROOT/lib/pkgconfig:$WASI_SYSROOT/share/pkgconfig
ENV PKG_CONFIG_SYSROOT_DIR=$WASI_SYSROOT
WORKDIR /root/cpython
RUN mkdir -p cross-build
WORKDIR /root/cpython/cross-build
RUN ../configure --host=wasm32-wasi --build=$(../config.guess) --with-build-python=../native-build/python
RUN make
WORKDIR /root/
RUN mkdir -p py-wasi-sandbox
RUN cp -r /root/cpython/Lib /root/py-wasi-sandbox
RUN cp -r /root/cpython/cross-build/build/lib.* /root/py-wasi-sandbox
RUN cp -r /root/cpython/cross-build/Modules /root/py-wasi-sandbox
RUN cp -r /root/cpython/cross-build/python.wasm /root/py-wasi-sandbox
RUN cp -r /root/webkit/WebKitBuild/JSCOnly/Release/bin/jsc /root/py-wasi-sandbox
RUN cp -r /root/webkit/WebKitBuild/JSCOnly/Release/lib /root/py-wasi-sandbox
RUN git clone https://github.com/UsQuake/wasi_sandbox_generator.git
WORKDIR /root/wasi_sandbox_generator
RUN cargo run py /root/cpython/native-build/python
FROM ubuntu:22.04
RUN \
      apt-get update && \
      apt-get install -y clang-15
WORKDIR /root/py-wasi-sandbox
COPY --from=builder /root/py-wasi-sandbox .
RUN mkdir -p /root/webkit
RUN mkdir -p /root/webkit/WebKitBuild
RUN mkdir -p /root/webkit/WebKitBuild/JSCOnly
RUN mkdir -p /root/webkit/WebKitBuild/JSCOnly/Release
RUN mv /root/py-wasi-sandbox/lib /root/webkit/WebKitBuild/JSCOnly/Release/
CMD ["tail", "-f", "/dev/null"]
