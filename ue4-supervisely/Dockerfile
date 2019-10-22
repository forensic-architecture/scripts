FROM ubuntu:18.04
MAINTAINER Lachlan Kermode <lk@forensic-architecture.org>

RUN apt-get update \
    && apt-get install -y python3-pip python3-dev \
    && cd /usr/local/bin \
    && ln -s /usr/bin/python3 python \
    && pip3 install --upgrade pip

# install apt-get packages
RUN apt-get update && \
    apt-get install -y \
    build-essential \
    libsm6 \
    libxrender1 \
    libfontconfig1 \
    wget \
    curl \
    git \
    htop \
    vim \
    zip \
    lsof \
    tmux \
	rename

RUN apt update && \
    apt install net-tools

# install pip packages
ARG requirements_file=docker.requirements.txt
COPY $requirements_file /requirements.txt
RUN pip install --upgrade pip && \
    pip install -r /requirements.txt \
    && rm -r ~/.cache/pip

COPY . /safariland
WORKDIR /safariland

CMD ["/bin/bash"]
