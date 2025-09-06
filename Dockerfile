FROM ubuntu:latest AS BUILDER

# Install build dependencies and tools needed for vcpkg
RUN apt-get update && apt-get install -y \
    cmake \
    clang \
    git \
    curl \
    zip \
    unzip \
    tar \
    ninja-build \
    pkg-config \
    && apt-get clean

# Install vcpkg
RUN git clone https://github.com/Microsoft/vcpkg.git /vcpkg && \
    /vcpkg/bootstrap-vcpkg.sh && \
    /vcpkg/vcpkg integrate install

RUN mkdir /app
WORKDIR /app

# Copy only the files needed for dependencies first (for better caching)
COPY vcpkg.json CMakeLists.txt ./
COPY source ./source

# Build with vcpkg toolchain
RUN cmake -B build . -DCMAKE_TOOLCHAIN_FILE=/vcpkg/scripts/buildsystems/vcpkg.cmake && \
    cmake --build build

FROM ubuntu:latest

RUN apt-get update && apt-get install -y ca-certificates && apt-get clean
RUN mkdir -p /app/build
WORKDIR /app
COPY --from=BUILDER /app/build/api /app/build

EXPOSE 8080
ENTRYPOINT ["./build/api"]
