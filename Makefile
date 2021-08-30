export UID=$(shell id -u)
export GID=$(shell id -g)

all:
	docker-compose up --build