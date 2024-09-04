LINUX_BIN=sniffer

build: clean
	GOOS=linux GOARCH=amd64 go build -v -o dist/$(LINUX_BIN) ./cmd/sniffer
	sudo setcap cap_net_raw,cap_net_admin=eip ./dist/$(LINUX_BIN)

dev:
	go build -v -o dist/$(LINUX_BIN) ./cmd/sniffer
	sudo setcap cap_net_raw,cap_net_admin=eip ./dist/$(LINUX_BIN)
	./dist/$(LINUX_BIN)

test:
	go test -v ./...

clean:
	rm -rf dist
