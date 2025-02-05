# This will build the default docker client

# <image>:<tag>
FROM rust:latest

# Copy from $PWD/. to the container's $PWD/.
COPY ./ ./

# Build & install the program
RUN cargo install --path .

EXPOSE 6113
EXPOSE 5000

CMD ["/bin/sh", "-c", "tic_tac_toe"]
