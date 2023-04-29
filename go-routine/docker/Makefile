
build-all: ubuntu-build rocky-build

rocky-build:
	docker build -t axemc-rocky -f ./rocky/Dockerfile ./rocky

rocky-run:
	docker run -itd --privileged=true --name axemc-rocky -p 22222:22 axemc-rocky:latest /sbin/init

ubuntu-build:
	docker build -t axemc-ubuntu:latest -f ./ubuntu/Dockerfile ./ubuntu

ubuntu-run:
	docker run -itd --privileged=true --name axemc-ubuntu -p 22222:22 axemc-ubuntu:latest /sbin/init
