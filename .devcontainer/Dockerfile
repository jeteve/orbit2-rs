FROM rockylinux/rockylinux:9

ARG USERNAME=vscode
ARG USER_UID=1000
ARG USER_GID=1000


RUN dnf install -y git which epel-release which && dnf clean all
RUN dnf install -y ORBit2-devel clang-devel && dnf clean all


RUN groupadd --gid $USER_GID $USERNAME \
    && useradd --uid $USER_UID --gid $USER_GID -m $USERNAME

USER $USERNAME

RUN curl --proto '=https' https://sh.rustup.rs | sh  -s -- -y
