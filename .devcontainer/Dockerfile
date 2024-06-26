FROM rockylinux/rockylinux:9

ARG USERNAME=vscode
ARG USER_UID=1000
ARG USER_GID=1000


RUN dnf install -y git sudo which patch epel-release which && dnf clean all
RUN dnf install -y ORBit2-devel python3 make  \
    clang-devel cmake llvm llvm-devel perl-FindBin \
    perl-IPC-Cmd && dnf clean all


RUN groupadd --gid $USER_GID $USERNAME \
    && useradd --uid $USER_UID --gid $USER_GID -m $USERNAME \
    && echo $USERNAME ALL=\(root\) NOPASSWD:ALL > /etc/sudoers.d/$USERNAME \
    && chmod 0440 /etc/sudoers.d/$USERNAME

USER $USERNAME

RUN curl --proto '=https' https://sh.rustup.rs | sh  -s -- -y
RUN source "$HOME/.cargo/env" && cargo install cargo-release
