include .env
export

.PHONY: debug-env
debug-env:
	@echo "--- Variables imported from .env ---"
	@$(foreach V,$(shell sed -n 's/^\([^#=]*\)=.*/\1/p' .env), \
		echo "$(V)=$($(V))"; \
	)

show-config:
	docker compose config


build:
	docker compose build

stop:
	docker compose down

deploy:
	docker compose up --build -d

distclean:
	docker volume rm -f \
		axum-web-server_storage_grafana \
		axum-web-server_storage_prometheus


