FROM 42tokyo2407.azurecr.io/nginx:latest
RUN curl -L https://github.com/tkuchiki/alp/releases/download/v1.0.21/alp_linux_amd64.tar.gz -o alp_linux_amd64.tar.gz
RUN    gzip -d alp_linux_amd64.tar.gz
RUN tar -xf alp_linux_amd64.tar && \
    install alp /usr/local/bin
