# py-sample

## 前提准备

```bash
# 设置环境变量
export APP_NAME=py-sample
export VERSION=0.0.1
export IMAGE_REGISTRY=registry.cn-hangzhou.aliyuncs.com
export IMAGE_NAMESPACE=yaoyaoio
export IMAGE_USER=""
export IMAGE_PASSWORD=""
export IMAGE_NAME=${IMAGE_REGISTRY}/${IMAGE_NAMESPACE}/${APP_NAME}:${VERSION}
```

## 快速构建

```bash
# 构建镜像
echo $IMAGE_PASSWD | docker login $IMAGE_REGISTRY-u $IMAGE_USER --password-stdin
docker build -t ${IMAGE_NAME} -f Dockerfile .
docker push ${IMAGE_NAME}
```

## 快速部署

```bash
envsubst < deploy/prod/deployment.yaml | kubectl apply -f -
envsubst < deploy/prod/service.yaml | kubectl apply -f -
```