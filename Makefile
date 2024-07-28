SHELL := /bin/bash

MYSQL_LOG_DIR := /var/log/mysql
MYSQL_LOG := $(MYSQL_LOG_DIR)/slow.log
HOST_LOG := ./webapp/mysql/logs/slow.log
RESULT_NGINX_LOG_DIR := ./webapp/nginx/results
DATE := $(shell date '+%T')
NGINX_ACCESS_LOG := /var/log/nginx/access.log

all: test

test:
	mkdir -p $(RESULT_NGINX_LOG_DIR)
	docker compose -f ./webapp/docker-compose.local.yml up -d
	docker compose -f ./webapp/docker-compose.local.yml exec db touch $(MYSQL_LOG)
	docker compose -f ./webapp/docker-compose.local.yml exec db mv $(MYSQL_LOG) $(MYSQL_LOG).$(DATE)
	docker compose -f ./webapp/docker-compose.local.yml exec nginx touch $(NGINX_ACCESS_LOG)
	docker compose -f ./webapp/docker-compose.local.yml exec nginx mv $(NGINX_ACCESS_LOG) $(NGINX_ACCESS_LOG).$(DATE)
	docker compose -f ./webapp/docker-compose.local.yml down
	docker compose -f ./webapp/docker-compose.local.yml up -d
	./run.sh
	mysqldumpslow -s t $(HOST_LOG) | \
	head -c 2000 | \
	jq -Rs '{username: "ろぐ子ちゃん", content: .}' | \
	curl -H "Content-Type: application/json" -X POST -d @- https://discord.com/api/webhooks/1266671844258349057/1pKcePHAIK1kC7NAqy_9N5ZLi2CkkDRRwsrKZwKOqbSPMx1o0Y9etZPvULl6150-g4UU
	mysqldumpslow -s t ./webapp/mysql/logs/slow.log > webapp/mysql/logs/slow_dbg.$(DATE).log
	docker compose -f ./webapp/docker-compose.local.yml exec nginx bash -c "cat /var/log/nginx/access.log | alp json -m '/api/user_image/[0-9]+,/orders/[0-9]+,/api/order/[0-9]+,/api/tow_truck/[0-9]+' --sort avg -r" > $(RESULT_NGINX_LOG_DIR)/$(DATE)
