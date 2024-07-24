FROM ubuntu:22.04 AS builder
WORKDIR /root/
RUN \
      apt-get update && \
      apt-get install -y llvm-15 llvm-15-tools clang-15 lld-15\
      build-essential libglib2.0-dev python3 python3-pip curl git\
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
RUN git clone https://github.com/UsQuake/wasi_sandbox_generator.git
RUN mkdir -p py-wasi-sandbox
RUN cp -r /root/cpython/Lib /root/py-wasi-sandbox
RUN cp -r /root/cpython/cross-build/build/lib.* /root/py-wasi-sandbox
RUN cp -r /root/cpython/cross-build/Modules /root/py-wasi-sandbox
RUN cp -r /root/cpython/cross-build/python.wasm /root/py-wasi-sandbox
RUN cp -r /root/mozilla-unified/obj/dist/bin/js /root/py-wasi-sandbox
WORKDIR /root/wasi_sandbox_generator
RUN cargo run py /root/cpython/native-build/python
FROM ubuntu:22.04
WORKDIR /root/py-wasi-sandbox
COPY --from=builder /root/py-wasi-sandbox .
CMD ["tail", "-f", "/dev/null"]
