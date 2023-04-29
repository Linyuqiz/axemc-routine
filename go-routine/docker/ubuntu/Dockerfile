FROM ubuntu:latest

RUN apt update -y && apt install vim openssh-server -y
RUN echo "PermitRootLogin yes" >> /etc/ssh/sshd_config
RUN echo "root:1234" | chpasswd

CMD ["/usr/sbin/sshd", "-D"]
