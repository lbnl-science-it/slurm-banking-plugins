
# Table of Contents

1.  [Introduction](#orgef94fe4)
2.  [Limitations](#orgd59c1f0)
3.  [Build Requirements](#orgc3ca552)
4.  [Building](#orgff739b4)
    1.  [On Savio](#org0ab5a4c)
    2.  [NixOS](#orgb163390)
    3.  [Help](#orga40ba7f)
5.  [Usage](#orge1d3c56)
    1.  [Install, enable, and configure](#orgec4f217)
        1.  [Install the `.so` files](#org332cacf)
        2.  [/etc/slurm/slurm.conf](#org3bdbf55)
        3.  [/etc/slurm/plugstack.conf](#org1dca94c)
        4.  [/etc/slurm/bank-config.toml](#org94f03cb)
    2.  [Help/Debugging](#org733dc5a)
6.  [Developing](#org39498c0)
    1.  [Project Structure](#org2d9044e)
    2.  [myBRC API Codegen](#orgceec85a)
    3.  [Testing with myBRC](#orgae084d2)

<a href="https://travis-ci.org/ucb-rit/slurm-banking-plugins"><img src="https://travis-ci.org/ucb-rit/slurm-banking-plugins.svg?branch=master"></a> <a href="."><img src="https://img.shields.io/github/languages/top/ucb-rit/slurm-banking-plugins"></a> <a href="."><img src="https://img.shields.io/github/repo-size/ucb-rit/slurm-banking-plugins"></a>


<a id="orgef94fe4"></a>

# Introduction

Slurm banking plugins provide allocation management to Slurm. The plugins deduct service units for completed and running jobs and prevent jobs from running if there are insufficient service units available. The plugins interact with a REST API (provided by myBRC), documented in the [spec/swagger.json](./spec/swagger.json). The following three plugins are used:

-   [job\_submit\_plugin](./job_submit_plugin) (job submission stage): Estimate maximum job cost based on submission parameters, and reject job if the API reports that the user/account has insufficient service units available.
-   [spank\_plugin](./spank_plugin) (job running stage): Report job and estimated cost to the API.
-   [job\_completion\_plugin](./job_completion_plugin) (job completing stage): Modify job in API to reflect actual usage.

These plugins are written in [Rust](https://www.rust-lang.org) to help with safety. It uses [rust-bindgen](https://github.com/rust-lang/rust-bindgen) to automatically generate the Rust foreign function interface (FFI) bindings based on the Slurm C header files.


<a id="orgd59c1f0"></a>

# Limitations

-   Since the spank plugin cannot cancel a job, the user could overdraw their service unit allocation if they had enough service units at the time of submission, but not enough service units at the time the job starts running, since the units are only actually deducted from the balance when the job starts running.
-   If `--ntasks` or `--cpus-per-task` are unspecified, the job completion plugin will assume the value is 0 and will always allow the job, as long as the balance is non-negative. This can be improved in the future by checking whether the requested partition is exclusive and how many CPUs each node has, and then using that information to estimate the number of CPUs that will be used.
-   If the myBRC API is offline (or returns errors), the submit plugin will let the job go through.


<a id="orgc3ca552"></a>

# Build Requirements

-   [Rust](https://www.rust-lang.org/) (including [cargo](https://doc.rust-lang.org/cargo/))
-   [Slurm](https://github.com/SchedMD/slurm) header files and source code
-   [Clang](http://clang.llvm.org/get_started.html) (dependency for [rust-bindgen](https://rust-lang.github.io/rust-bindgen/requirements.html))


<a id="orgff739b4"></a>

# Building

Since the Slurm `jobcomp` plugins need access to the `src/common/slurm_jobcomp.h` header, we need access to the Slurm source code `src` directory in order to build (as well as the normal `<slurm/slurm.h>` headers on the `CPATH`). 

You will have to first run `./configure` on the Slurm source code, otherwise `<slurm/slurm.h>` will not exist. If you don't run `./configure`, the Makefile will try to do it for you.

1.  Edit the path at the top of the Makefile to point to the Slurm source code directory, or symlink `./slurm` in this repository to point to it.
2.  Once you have all the dependencies, just run `make` :)
3.  After building, you will find the `.so` files in the same directory as the Makefile.


<a id="org0ab5a4c"></a>

## On Savio

You will need the Rust and `clang` dependencies. 
Rust can be installed following the instructions on [rustup.rs](https://rustup.rs), and is easiest if installed locally for each user. 
`clang` can be loaded as a module (or by setting the environment variables).

The plugins can be built as an unprivileged user, as long as that user can read the Slurm source code.

    # After installing Rust (using rustup)...
    module load clang
    git clone https://github.com/ucb-rit/slurm-banking-plugins.git && cd slurm-banking-plugins
    rmdir slurm && ln -s /path/to/slurm/source slurm # Point to slurm source
    make

Then, follow the instructions in [Usage](#org2413015) to install, enable, and configure the plugins.

**When adding the .so binaries to the nodes with Warewulf, you must use "wwsh file import" instead of "wwsh file new". Make sure the format in "wwsh file print" is listed as binary.**


<a id="orgb163390"></a>

## NixOS

`shell.nix` provides the environment for development on [NixOS](https://nixos.org). I run the following:

    nix-shell 
    make


<a id="orga40ba7f"></a>

## Help

For additional reference on building, check [the build on travis-ci](https://travis-ci.org/ucb-rit/slurm-banking-plugins).


<a id="orge1d3c56"></a>

# <a id="org2413015"></a> Usage


<a id="orgec4f217"></a>

## Install, enable, and configure


<a id="org332cacf"></a>

### Install the `.so` files

The `job_submit_slurm_banking.so` and `jobcomp_slurm_banking.so` should be installed in `/usr/lib64/slurm/`. The `spank_slurm_banking.so` plugin should be installed in `/etc/slurm/spank/`.

    make install


<a id="org3bdbf55"></a>

### /etc/slurm/slurm.conf

Enable the submit and completion plugins:

    # other config options above...
    JobSubmitPlugins=job_submit/slurm_banking
    JobCompType=jobcomp/slurm_banking


<a id="org1dca94c"></a>

### /etc/slurm/plugstack.conf

Enable the spank plugin:

    required /etc/slurm/spank/spank_slurm_banking.so


<a id="org94f03cb"></a>

### /etc/slurm/bank-config.toml

Configure the plugin settings. Options that **must** be set properly include the API URL, API token, and partition names. You can use the example provided as a template.

    cp bank-config.toml.example /etc/slurm/bank-config.toml


<a id="org733dc5a"></a>

## Help/Debugging

-   The plugins log errors to the slurmd (spank plugin) and slurmctld (job submit and job completion plugins) logs. You can filter for their output by grepping for `_bank`.
-   For a working example installation, refer to [the Docker files](./docker)


<a id="org39498c0"></a>

# Developing

I use the [docker-centos7-slurm](https://github.com/giovtorres/docker-centos7-slurm) Docker container as a base, and build the plugins on top of it. 

`make docker-dev` builds the development container with Slurm plus all the other necessary dependencies for the plugins and drops you into a shell. The code is stored in `/slurm-banking-plugins` in the container. After making your changes, use `make && make install` to compile and install the plugins, copy the `plugstack.conf` and `bank-config.toml` config files to `/etc/slurm/`, and finally restart Slurm with `supervisorctl restart all`.


<a id="org2d9044e"></a>

## Project Structure

Each plugin is its own Rust project: [job\_completion\_plugin](./job_completion_plugin), [job\_submit\_plugin](./job_submit_plugin), and [spank\_plugin](./spank_plugin). Each of these uses the [slurm\_banking](./slurm_banking) project, which includes the job calculation functionality and helpers for calling the API. Communication with the myBRC API is done through [mybrc\_rest\_client](./mybrc_rest_client), described in the next section.


<a id="orgceec85a"></a>

## myBRC API Codegen

I use [swagger-codegen](https://github.com/swagger-api/swagger-codegen) to generate a library to abstract away access to the API. The API is described by a schema file in [spec/swagger.json](./spec/swagger.json). This file is automatically generated by the myBRC API, and can be obtained at `/swagger.json` on the myBRC API.

If the API spec changes and you need to update this plugin, just regenerate the API client. First, put the new `swagger.json` in [spec/swagger.json](./spec/swagger.json). To generate the API client based on this new schema, I use the Dockerized version of [swagger-codegen](https://github.com/swagger-api/swagger-codegen) like so:

    docker run --rm -v $(pwd):/local swaggerapi/swagger-codegen-cli generate \
      -i /local/spec/swagger.json \
      -l rust \
      -o /local/mybrc_rest_client

You may find the generated files are not owned by your user, so just run `chown -R $USER mybrc_rest_client`.


<a id="orgae084d2"></a>

## Testing with myBRC

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

