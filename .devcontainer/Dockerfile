FROM rockylinux/rockylinux:9

RUN dnf install -y git which epel-release which && dnf clean all
RUN dnf install -y ORBit2-devel clang-devel && dnf clean all

RUN curl --proto '=https' https://sh.rustup.rs | sh  -s -- -y
