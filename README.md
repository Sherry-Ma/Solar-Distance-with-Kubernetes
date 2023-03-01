# Solar-Distance-with-Kubernetes

## Overview

This is a application that get solar distance of planets in solar system. In this project, I learned/did:
1. Use hashmap to store and get data pairs in Rust
2. Containerize the application with Docker
3. Push image to DockerHub
4. Deploy image to minikube

## Run the docker image locally
Type `make build` and `make rundocker`, which will run the corresponding commands in Makefile

Build docker image:
```
    build:
	    docker build -t solar_distance .
```

run:
```
    rundocker:
	    docker run -it --rm -p 8080:8080 solar_distance
```
then you can visit http://127.0.0.1:8080/ in your browser and interact with the microservice






## Push image to Dockerhub



## Deploy image to minikube
