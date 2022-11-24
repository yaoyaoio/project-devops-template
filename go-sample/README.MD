# go-sample

## 前提准备

```bash  
# 设置环境变量  
export APP_NAME=go-sample
export VERSION=0.0.1  
export IMAGE_REGISTRY="{仓库地址}"  
export IMAGE_NAMESPACE="{命名空间/组织空间}"  
export IMAGE_USER="{用户名}"  
export IMAGE_PASSWORD="{密码}"  
export IMAGE_NAME=${IMAGE_REGISTRY}/${IMAGE_NAMESPACE}/${APP_NAME}:${VERSION}  
```  

## 快速构建（多阶段构建）

```bash  
# 构建镜像  
echo $IMAGE_PASSWD | docker login $IMAGE_REGISTRY -u $IMAGE_USER --password-stdin  
# docker 构建  
docker build -t ${IMAGE_NAME} -f Dockerfile-builder .  
docker push ${IMAGE_NAME}  
# nerdctl 构建  
nerdctl build -t ${IMAGE_NAME} -f Dockerfile-builder .  
nerdctl push ${IMAGE_NAME}  
# lima + nerdctl 构建  
limactl shell default -- nerdctl build -t ${IMAGE_NAME} -f Dockerfile-builder .  
limactl shell default -- nerdctl push ${IMAGE_NAME}  
```  

## 普通构建 (使用当前 Golang 环境)

```bash
# 编译项目
make build
# docker 构建  
docker build -t ${IMAGE_NAME} -f Dockerfile .  
docker push ${IMAGE_NAME}  
# nerdctl 构建  
nerdctl build -t ${IMAGE_NAME} -f Dockerfile .  
nerdctl push ${IMAGE_NAME}  
# lima + nerdctl 构建  
limactl shell default -- nerdctl build -t ${IMAGE_NAME} -f Dockerfile .  
limactl shell default -- nerdctl push ${IMAGE_NAME}  
```

## 快速部署

```bash  
# kubernetes 部署 (envsubst 命令可以读取 env 环境变量并且把变量替换到文件中)  
envsubst < deploy/prod/deployment.yaml | kubectl apply -f -  
envsubst < deploy/prod/service.yaml | kubectl apply -f -  
# docker部署  
docker run -d -p 8080:8080 ${IMAGE_NAME}  
# nerdctl部署
nerdctl run -d -p 8080:8080 ${IMAGE_NAME}
# lima + nerdctl 部署
limactl shell default -- nerdctl run -d -p 8080:8080 ${IMAGE_NAME}  
```  

**注**
如果使用私有镜像仓库 kubernetes 需要配置镜像仓库的 secret

```yaml
spec:
  imagePullSecrets:
    - name: { secret-name }
```

## 使用 CICD

返回上一级查看 [README.MD](../README.MD)
