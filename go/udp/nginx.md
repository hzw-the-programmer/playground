stream {
        upstream udps {
            server localhost:3333;
        }

        server {
            listen 6666 udp;
            proxy_pass udps;
        }
}
