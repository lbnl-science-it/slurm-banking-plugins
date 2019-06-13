# Slurm Banking Plugins

[![Travis Build Status](https://travis-ci.org/ucb-rit/slurm-banking-plugins.svg?branch=master)](https://travis-ci.org/ucb-rit/slurm-banking-plugins)

__Currently in development - Not ready for use__

Slurm banking plugins provide allocation management to Slurm. The plugins deduct service units for completed and running jobs and prevent jobs from running if there are insufficient service units available. There are two plugins, one running on job submission and the other on job completion. The job completion plugin will reimburse service units if the job ran for less time than was expected based on its submission options. The plugins interact with a REST API (provided by mybrc-rest) to keep track of the jobs and account balances.

These plugins are written in [Rust](https://www.rust-lang.org), an efficient and memory-safe programming language. It uses [rust-bindgen](https://github.com/rust-lang/rust-bindgen) to automatically generate the Rust foreign function interface (FFI) bindings based on the Slurm C header files.

## Build Requirements
- [Rust](https://www.rust-lang.org/) (including [cargo](https://doc.rust-lang.org/cargo/))
- [Slurm](https://github.com/SchedMD/slurm) header files and source code
- [Clang](http://clang.llvm.org/get_started.html) (dependency for [rust-bindgen](https://rust-lang.github.io/rust-bindgen/requirements.html))

## Building
Since the Slurm `jobcomp` plugins need access to the `"src/common/slurm_jobcomp.h"` header, we need access to the Slurm source code `src` directory in order to build (as well as the normal `<slurm/slurm.h>` headers on the `CPATH`). 

**You will have to first run `./configure` on the Slurm source code, otherwise `<slurm/slurm.h>` will not exist. If you don't run `./configure`, the Makefile will try to do it for you.**

1. Edit the path at the top of the Makefile to point to the Slurm source code directory, or symlink `./slurm` in this repository to point to it.
2. Once you have all the dependencies, just run `make` :)
3. After building, you will find the `.so` files in the same directory as the Makefile.

### On Savio
```bash
# After installing Rust (using rustup)...
module load clang
git clone https://github.com/ucb-rit/slurm-banking-plugins.git && cd slurm-banking-plugins
rmdir slurm && ln -s /path/to/slurm/source slurm # Point to slurm source
make
make install
vim /etc/slurm/slurm.conf # Edit slurm.conf
cp prices.toml /etc/slurm/prices.toml
```

### NixOS
`shell.nix` provides the environment for development on [NixOS](https://nixos.org). I run the following:

```bash
nix-shell 
make
```

### Help
For additional reference on building, check [the build on travis-ci](https://travis-ci.org/ucb-rit/slurm-banking-plugins).

## Usage
1. Move the `.so` files to `/usr/lib64/slurm`:
```bash
make install
```
2. Move `prices.toml` to `/etc/slurm/prices.toml` and update the partitions/prices accordingly.
```bash
cp prices.toml /etc/slurm/prices.toml
```
3. Include the plugins in the `/etc/slurm/slurm.conf`:
```bash
# other config options above...
JobSubmitPlugins=job_submit/bank
JobCompPlugins=jobcomp/bank
```

### Help
For additional reference on usage, refer to [the Docker files](docker).

## Developing
I use the [docker-centos7-slurm](https://github.com/giovtorres/docker-centos7-slurm) Docker container as a base, and build the plugins on top of it. 

`make docker-dev` builds the development container with Slurm plus all the other necessary dependencies for the plugins and drops you into a shell. The code is stored in `/slurm-banking-plugins` in the container. After making your changes, use `make && make install` to compile and install the plugins, then restart Slurm with `supervisorctl restart all`.

### myBRC API Codegen
I use [swagger-codegen](https://github.com/swagger-api/swagger-codegen) to generate a library to abstract away access to the API. The API is described by a schema file in [spec/swagger.json](spec/swagger.json). This file is automatically generated by the myBRC API, and can be obtained at `/swagger.json` on the myBRC API.

If the API spec changes and you need to update this plugin, just regenerate the API client. First, put the new `swagger.json` in [spec/swagger.json](spec/swagger.json). To generate the API client based on this new schema, I use the Dockerized version of [swagger-codegen](https://github.com/swagger-api/swagger-codegen) like so:

```bash
docker run --rm -v $(shell pwd):/local swaggerapi/swagger-codegen-cli generate \
  -i /local/spec/swagger.json \
  -l rust \
  -o /local/mybrc_rest_client
```

Current banking interaction is through two simple calls:
- `POST` to `/jobs/{slurmjobid}` will create a job and update the usage if the usage is within the allocated amount
- `PUT` to `/jobs/{slurmjobid}` will create/update a job no matter what, and update the usage even if that means the account is overdrawn

### Testing with myBRC

```bash
# Build mybrc-rest Docker image from scgup
docker build -f Dockerfile.mybrc-rest -t mybrc-rest

# Build slurm-banking-plugins-dev image
make docker-dev

# Launch containers
docker run --name=mybrc-rest -d -p 8181:8181 mybrc-rest
docker run \
  -v $(pwd)/job_submit_plugin/src:/slurm-banking-plugins/job_submit_plugin/src \
  -v $(pwd)/job_completion_plugin/src:/slurm-banking-plugins/job_completion_plugin/src \
  -v $(pwd)/slurm_banking/src:/slurm-banking-plugins/slurm_banking/src \
  --link mybrc-rest -it -h ernie slurm-banking-plugins-dev
```