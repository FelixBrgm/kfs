# Use an official lightweight base image
FROM --platform=linux/amd64 rust:1.70-bullseye

# Install necessary tools
RUN apt-get update && apt-get install -y \
	build-essential \
	curl \
	gcc \
	gdb \
	lldb \
	git \
	pkg-config \
	nasm \
	mtools \
	xorriso \
	grub-pc-bin \
	grub-efi-amd64-bin \
	qemu \
	qemu-system-x86 \
	&& rm -rf /var/lib/apt/lists/*

# RUN curl https://sh.rustup.rs -sSf | bash -s -- -y

# RUN echo 'source $HOME/.cargo/env' >> $HOME/.bashrc
# RUN /bin/bash -c "source $HOME/.cargo/env && rustup target add x86_64-unknown-none"
RUN rustup target add x86_64-unknown-none

RUN echo 'export PS1="\W> "' >> ~/.bashrc