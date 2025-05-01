# ---| Compile medusa

FROM golang:latest AS medusa
WORKDIR /src
RUN /usr/bin/git clone https://github.com/crytic/medusa.git . && \
    /usr/bin/git checkout "$(/usr/bin/git describe --tags | /usr/bin/sed 's/-[0-9]\+-g\w\+$//')" && \
    /usr/local/go/bin/go build -trimpath -o=/usr/local/bin/medusa -ldflags="-s -w" && \
    /usr/bin/chmod 755 /usr/local/bin/medusa

# ---| Final system image

FROM ubuntu:latest AS system

# Metadata
LABEL org.opencontainers.image.title="EthSecurity"
LABEL org.opencontainers.image.description="Docker container preconfigured with tools for Ethereum security"
LABEL org.opencontainers.image.source="https://github.com/cosasdepuma/totes/"
LABEL org.opencontainers.image.url="https://github.com/cosasdepuma/totes/blob/main/ethsecurity"
LABEL org.opencontainers.image.version="1.0.0"
LABEL org.opencontainers.image.created="2024-08-19"
LABEL org.opencontainers.image.licenses="GNU GPL v3.0"

# Install system dependencies
ENV DEBIAN_FRONTEND=noninteractive
RUN /usr/bin/apt-get update && /usr/bin/apt-get install -y --no-install-recommends \
        build-essential \
        bash-completion curl git \
        python3-dev python3-pip python3-venv \
        sudo unzip wget \
        libc6-amd64-cross libc6-dev-amd64-cross && \
    /usr/bin/apt-get clean && /usr/bin/rm -rf /var/lib/apt/lists/*

# Install n (node version manager), lts node, npm, and yarn
RUN /usr/bin/curl -fsSL https://raw.githubusercontent.com/tj/n/master/bin/n | /bin/bash -s lts && \
    /usr/local/bin/npm install -g n yarn && /usr/local/bin/npm --force cache clean

# Install medusa
COPY --from=medusa --chown=root:root /usr/local/bin/medusa /usr/local/bin/medusa
RUN /usr/local/bin/medusa completion bash > /etc/bash_completion.d/medusa

# Create a user with passwordless sudo
RUN /usr/sbin/useradd -mUG sudo -s /bin/bash ethsec && \
    /bin/echo 'ethsec ALL=(ALL) NOPASSWD: ALL' >> /etc/sudoers
USER ethsec
ENV HOME="/home/ethsec"
ENV PATH="${PATH}:${HOME}/.local/bin:${HOME}/.cargo/bin:${HOME}/.local/ethereum/bin:${HOME}/.foundry/bin:${HOME}/.vyper/bin"

# Install rust
RUN /usr/bin/curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | /bin/bash -s -- -y

# Install foundry (forge)
RUN /usr/bin/curl -L https://foundry.paradigm.xyz | /bin/bash && \
    "${HOME}"/.foundry/bin/foundryup
USER root
RUN for tool in anvil cast forge; do \
        echo $HOME && \
        "${HOME}"/.foundry/bin/"$tool" completions bash > /etc/bash_completion.d/"${tool}"; \
    done
USER ethsec

# Create a virtual environment for python tools
RUN /usr/bin/python3 -m venv ${HOME}/.local/ethereum

# Install solc-select (also install some solc releases, and select the latest version)
RUN "${HOME}"/.local/ethereum/bin/pip3 install --no-cache-dir solc-select && \
    solc-select install 0.4.26 0.5.17 0.6.12 0.7.6 latest && \
    solc-select use latest

# Install vyper compiler
RUN "${HOME}"/.local/ethereum/bin/pip3 install --no-cache-dir vyper

# Install cryptic compiler
RUN "${HOME}"/.local/ethereum/bin/pip3 install --no-cache-dir crytic-compile

# Install pyevmasm (dis)assembler
RUN "${HOME}"/.local/ethereum/bin/pip3 install --no-cache-dir pyevmasm

# Install slither static analyzer
RUN "${HOME}"/.local/ethereum/bin/pip3 install --no-cache-dir slither-analyzer

# Install mythril static analyzer
RUN "${HOME}"/.local/ethereum/bin/pip3 install --no-cache-dir mythril

# Install tealer static analyzer
RUN "${HOME}"/.local/ethereum/bin/pip3 install --no-cache-dir tealer

# Change the message of the day
COPY --link --chown=root:root ./motd /etc/motd
RUN /usr/bin/touch "${HOME}"/.sudo_as_admin_successful && \
    /usr/bin/echo '\n/usr/bin/cat /etc/motd\n' >> "${HOME}"/.bashrc

# Entrypoint
WORKDIR /code
CMD ["/bin/bash"]