FROM ubuntu:22.04 AS builder
WORKDIR /root/
RUN \
      apt-get update && \
      apt-get install -y llvm-15 llvm-15-tools clang-15 lld-15\
      build-essential libglib2.0-dev curl git ruby3.0\
      cmake autoconf make pkgconf libbz2-dev \
      libffi-dev liblzma-dev libsqlite3-dev libssl-dev \ 
      lzma lzma-dev zlib1g-dev libyaml-dev
RUN git clone https://github.com/ruby/ruby.git
WORKDIR /root/ruby
RUN git clone https://github.com/UsQuake/ruby_patch.git
RUN mv ruby_patch/patch.patch .
RUN patch -p0 < patch.patch
RUN ./autogen.sh
RUN mkdir -p native-build
WORKDIR /root/ruby/native-build
RUN mkdir -p /root/rb-native-sandbox
RUN ../configure --prefix /root/rb-native-sandbox
RUN make
RUN make install
FROM ubuntu:22.04
WORKDIR /root/rb-native-sandbox
COPY --from=builder /root/rb-native-sandbox .
CMD ["sh", "-c", "while :; do sleep 1; done"]
