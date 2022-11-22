# 简单说明

## 依赖

代码仓库凭证

镜像仓库凭证

KUBECONFIG

## 容器相关命令

lima + nerdctl

```bash
lima nerdctl build -t yaoyaoio/go-sample:latest -f Dockerfile .
lima nerdctl run -p 8080:8080 yaoyaoio/go-sample:latest
lima nerdctl login
lima nerdctl push yaoyaoio/go-sample:latest
```

docker

```bash
docker build -t yaoyaoio/go-sample:latest -f Dockerfile .
docker run -p 8080:8080 yaoyaoio/go-sample:latest
docker login
docker push yaoyaoio/go-sample:latest
```

nerdctl

```bash 
nerdctl build -t yaoyaoio/go-sample:latest -f Dockerfile .
nerdctl run -p 8080:8080 yaoyaoio/go-sample:latest
nerdctl login
nerdctl push yaoyaoio/go-sample:latest
```

## environment 环境变量

`APP_NAME` 项目名称

`REGISTRY` 镜像仓库地址

`ORG_NAME` 组织名称

`BRANCH` 分支名称

```bash
export APP_NAME=go-sample
export REGISTRY=registry.cn-hangzhou.aliyuncs.com
export ORG_NAME=yaoyaoio
export BRANCH=main
```

## 发布到 Kubernetes

```bash
envsubst < deploy/prod/deployment.yaml | kubectl apply -f -
envsubst < deploy/prod/service.yaml | kubectl apply -f -
```

