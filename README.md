# pbddns

pbddns is a simple client for the updating of dynamic IPs on subdomains using the Porkbun API. It uses a config file to store the API keys and for securities sake can not create or remove subdomains.

The files in releases are for Linux amd64. I'm exceptionally bad at figuring out cross-compiling, so that's about as far as things are going to go for now.

## Usage

Running it is as simple as running the actual program file. By default it looks for a config file in `./data` relative to the binary and creates a *blank* `config.toml` within it. This location can be changed with an argument (ex: `pbddns -d /etc/pbddns`). The config file should be filled out with the Porkbun API info and the subdomain entries. The subdomain entries other than the TLD use the same syntax as Porkbun and do not need to include the TLD.

Do note that the client is one-shot by nature. It is expected that it will be run with a cron job or a systemd timer.
