# Use the devenv image as the base
FROM ghcr.io/cachix/devenv/devenv:latest

# Set the working directory inside the container
WORKDIR /app

# Increase the download buffer size for Nix to prevent warnings
RUN mkdir -p /etc/nix && echo "download-buffer-size = 1004857600" >> /etc/nix/nix.conf

# Copy the current project context into the container
# This assumes the Dockerfile is in the root of your project
# COPY . /app

# Install all devenv dependencies
RUN devenv shell install:all

# This will execute when the container starts
CMD ["bash"]
