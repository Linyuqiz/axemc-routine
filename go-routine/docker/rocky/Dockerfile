FROM rockylinux:9

RUN yum -y install epel-release && yum repolist && yum clean all && yum makecache
RUN yum update -y && yum install vim openssh-server -y
RUN echo "PermitRootLogin yes" >> /etc/ssh/sshd_config
RUN echo "root:1234" | chpasswd

CMD ["/usr/sbin/sshd", "-D"]
