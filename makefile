start:
	docker compose -f backend/docker-compose.yml up


build_back:
	docker compose -f backend/docker-compose.yml up santa_labyrinth_api --build

stop:
	docker compose -f backend/docker-compose.yml down
