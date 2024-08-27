FROM ubuntu

RUN apt-get update
RUN apt install -y make build-essential libssl-dev zlib1g-dev libbz2-dev libreadline-dev libsqlite3-dev wget curl llvm libncursesw5-dev xz-utils tk-dev libxml2-dev libxmlsec1-dev libffi-dev liblzma-dev

RUN apt install -y git

ENV HOME=/home/ubuntu
ENV PYENV_ROOT=$HOME/.pyenv
ENV PATH=$PYENV_ROOT/shims:$PYENV_ROOT/bin:$PATH

RUN curl https://pyenv.run | bash

RUN mkdir /usr/local/nvm
ENV NVM_DIR /usr/local/nvm

# nvm environment variables
ENV NODE_VERSION=20.0.0

# install nvm
# https://github.com/creationix/nvm#install-script
RUN curl https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.1/install.sh | bash \
    && . $NVM_DIR/nvm.sh \
    && nvm install $NODE_VERSION \
    && nvm alias default $NODE_VERSION \
    && nvm use default

ENV NODE_PATH $NVM_DIR/v$NODE_VERSION/lib/node_modules
ENV PATH $NVM_DIR/versions/node/v$NODE_VERSION/bin:$PATH

# install python
RUN pyenv install 3.9.19

COPY . .

