FROM python:3.9.15-slim

WORKDIR /usr/local/app

COPY requirements.txt /usr/local/app/
RUN pip install --no-cache-dir -r requirements.txt -i https://pypi.tuna.tsinghua.edu.cn/simple

COPY . .

CMD ["uvicorn", "main:app", "--host", "0.0.0.0","--port", "8080"]
EXPOSE 8080