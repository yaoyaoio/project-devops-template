# 简单说明

本项目 CICD 只在 Kubesphere 上测试过，其他平台未测试。

## 依赖 Kubernetes Secret

代码仓库凭证
如果你使用 Gitlab 作为代码仓库 则需要把 Gitlab 的凭证放到 Kubernetes 的 Secret 中

```bash
kubectl create secret yaoyao.io code-credentials --from-literal=username=username --from-literal=password=password
```

镜像仓库凭证
如果你使用 Harbor 作为镜像仓库 或者使用其他镜像仓库 则需要把相关的凭证放到 Kubernetes 的 Secret 中

```bash

```

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

## 发布到 Kubernetes (依赖环境变量)

```bash
envsubst < deploy/prod/deployment.yaml | kubectl apply -f -
envsubst < deploy/prod/service.yaml | kubectl apply -f -
```

## 使用Jenkinsfile

1. 把 Jenkinsfile 复制到 Kubesphere 的 Devops 项目中即可使用