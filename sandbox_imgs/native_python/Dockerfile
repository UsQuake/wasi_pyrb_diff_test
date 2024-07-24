FROM ubuntu:22.04 AS builder
WORKDIR /root/
RUN \
      apt-get update && \
      apt-get install -y llvm-15 llvm-15-tools clang-15 lld-15\
      build-essential libglib2.0-dev python3 python3-pip curl git\
      cmake autoconf wget make pkgconf libbz2-dev \
      libffi-dev liblzma-dev libsqlite3-dev libssl-dev lzma lzma-dev zlib1g-dev
RUN git clone https://github.com/python/cpython.git
WORKDIR /root/cpython
RUN mkdir -p native-build
RUN mkdir -p /root/py-native-sandbox
WORKDIR /root/cpython/native-build
RUN ../configure --prefix /root/py-native-sandbox
RUN make
RUN make install
FROM ubuntu:22.04
WORKDIR /root/py-native-sandbox
COPY --from=builder /root/py-native-sandbox .
CMD ["sh", "-c", "while :; do sleep 1; done"]
