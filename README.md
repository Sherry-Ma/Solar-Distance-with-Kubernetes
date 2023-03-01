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
First you need to sign up for an account at dockerhub!(Aquire a username and the password)
In the ternimal, login dockerhub:
```
docker login --username=<your-dockerhub-username>
```
You may need to input your password. Then push the docker image with command:
```
docker build . -t <your-dockerhub-username>/<your-application-name>
docker push <your-dockerhub-username>/<your-application-name>
```

## Deploy image to minikube
First, start your cluster:
```
minikube start
```
Create a deployment:
```
kubectl create deployment $NAME --image=registry.hub.docker.com/<your-dockerhub-username>/<your-application-name>
# Replace $NAME with any cluster name
```
Create service:
```
kubectl expose deployment $NAME --type=LoadBalancer --port=8080
```
You may view the service status:
```
kubectl get service $NAME
```
Finally, get a url that can run the application
```
minikube service $NAME  --url
```
This command will return a url like `http://192.168.49.2:32155` for you to run the app in a browser
