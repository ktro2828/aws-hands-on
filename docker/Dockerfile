FROM node:19
LABEL maintainer="Kotaro Uetake"

RUN apt-get update \
    && apt-get install nano less

RUN cd /opt \
    RUN cd /opt \
    && curl -q "https://www.python.org/ftp/python/3.10.8/Python-3.10.8.tgz" -o Python-3.10.8.tgz \
    && tar -xzf Python-3.10.8.tgz \
    && cd Python-3.10.8 \
    && ./configure --enable-optimizations \
    && make install

RUN cd /opt \
    && curl "https://awscli.amazonaws.com/awscli-exe-linux-x86_64.zip" -o "awscliv2.zip" \
    && unzip awscliv2.zip \
    && ./aws/install

RUN npm install -g aws-cdk@1.100

# clean up unnecessary files
RUN rm -rf /opt/*

# copy hands-on source code in /root/
COPY src/ /root/src

COPY .aws/ /root/.aws

# copy .aws if it exists in /root/
RUN if [ -e "/root/.aws"]; then \
        echo "\e[32m>> [INFO] Found .aws directory\e[m"; \
    else \
        echo "\e[33m>> [WARN] Can not find .aws directory, please make it in your container!!\e[m"; \
    fi

# Make command line prettier...
RUN echo "alias ls='ls --color=auto'" >> /root/.bashrc
RUN echo "PS1='${debian_chroot:+($debian_chroot)}\[\033[01;32m\]\u@aws-hands-on\[\033[00m\]:\[\033[01;34m\]\w\[\033[00m\]\$ '" >> /root/.bashrc

RUN mkdir -p /root/.ssh
WORKDIR /root
CMD ["/bin/bash"]