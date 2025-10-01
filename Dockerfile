# Use the devenv image as the base
FROM nixos/nix:latest

# Set the working directory inside the container
WORKDIR /app

# Increase the download buffer size for Nix to prevent warnings
RUN mkdir -p /etc/nix && echo "download-buffer-size = 1004857600" >> /etc/nix/nix.conf
RUN echo "filter-syscalls = false" >> /etc/nix/nix.conf
RUN echo "extra-experimental-features = nix-command flakes" >> /etc/nix/nix.conf
# Use the provided `GITHUB_TOKEN` if available
RUN --mount=type=secret,id=GITHUB_TOKEN,env=GITHUB_TOKEN [ -n $GITHUB_TOKEN ] && echo "access-tokens = github.com:$GITHUB_TOKEN" >> /etc/nix/nix.conf
# Always allow substituting from the cache, even if the derivation has `allowSubstitutes = false`.
# This is a CI optimisation to avoid having to download the inputs for already-cached derivations
# to rebuild trivial text files.
RUN echo "always-allow-substitutes = true" >> /etc/nix/nix.conf

# Install nix dependencies
RUN nix profile add --accept-flake-config nixpkgs#cachix
RUN cachix use devenv
RUN nix profile add nixpkgs#devenv

# Copy the current project context into the container
# This assumes the Dockerfile is in the root of your project
# ADD ./devenv.nix ./devenv.yaml ./dprint.json ./rust-toolchain.toml ./rustfmt.toml ./.envrc ./devenv.lock ./Cargo.toml ./Cargo.lock /app

# # Install all devenv dependencies
# RUN devenv test -v
