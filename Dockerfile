# Use the devenv image as the base
FROM nixos/nix:latest

# Set the working directory inside the container
WORKDIR /app

# Increase the download buffer size for Nix to prevent warnings
RUN mkdir -p /etc/nix && echo "download-buffer-size = 1004857600" >> /etc/nix/nix.conf
RUN echo "filter-syscalls = false" >> /etc/nix/nix.conf
RUN echo "extra-experimental-features = nix-command flakes" >> /etc/nix/nix.conf
RUN nix profile add --accept-flake-config nixpkgs#cachix
RUN cachix use devenv
RUN nix profile add nixpkgs#devenv

# Copy the current project context into the container
# This assumes the Dockerfile is in the root of your project
# COPY ./devenv.nix ./devenv.yaml ./dprint.json ./rust-toolchain.toml ./rustfmt.toml ./.envrc ./devenv.lock ./Cargo.toml ./Cargo.lock /app

# # Install all devenv dependencies
# RUN devenv test -v
