FROM python:2.7-slim

WORKDIR /app

COPY . /app

RUN pip install --trusted-host pypi.python.org -r requirements.txt

EXPOSE 80

ENV NAME Zhiwen He

CMD ["python", "app.py"]

# docker build --tag=friendlyhello .
# docker run -p 4000:80 friendlyhello
# CTRL + C
# docker run -d -p 4000:80 friendlyhello
# docker container stop 51c71a54ab22
