FROM redis:alpine

ARG REDIS_PASSWORD=Pass:123
RUN mkdir -p /usr/local/etc/redis
COPY redis.conf /usr/local/etc/redis/redis.conf
RUN sed -i "s/__REDIS_PASSWORD__/${REDIS_PASSWORD}/g" /usr/local/etc/redis/redis.conf

VOLUME /data

EXPOSE 6379

CMD ["redis-server"]
