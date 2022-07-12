FROM rust:1.60.0

ENV PROTOC_VERSION 3.8.0

RUN set -x; \
    export PROTOC_ZIP=$(echo "protoc-$PROTOC_VERSION-linux-$(uname -m).zip" | sed 's/aarch/aarch_/g'); \
    apt update \
      && apt install -y \
      clang \
      cmake \
      lcov \
      libudev-dev \
      mscgen \
      net-tools \
      rsync \
      sudo \
      golang \
      unzip \
      \
     && apt remove -y libcurl4-openssl-dev \
     && rm -rf /var/lib/apt/lists/* \
     && cargo --version \
     && curl -OL https://github.com/google/protobuf/releases/download/v$PROTOC_VERSION/$PROTOC_ZIP \
     && unzip -o $PROTOC_ZIP -d /usr/local bin/protoc \
     && unzip -o $PROTOC_ZIP -d /usr/local include/* \
     && rm -f $PROTOC_ZIP \
     && cargo install protobuf-codegen --locked