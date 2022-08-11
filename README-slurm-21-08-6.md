This branch was created in order to port the Slurm banking plugins to Slurm 21.08.6. To clone this branch, run

```
$ git clone -b slurm-21-08-6 git@github.com:lbnl-science-it/slurm-banking-plugins.git
$ cd slurm-banking-plugins
```

## Source Code Changes

There are some API changes between Slurm v20.11 and v21.08. I only needed to make some minor changes to 2 files in order to successfully build the Slurm banking plugins with Slurm 21.08.6. On my Mac, I did:

```
$ sed -i '' -e 's/ESLURM/slurm_err_t_ESLURM/g' job_completion_plugin/src/jobcomp_bank.rs
$ sed -i '' -e 's/ESLURM/slurm_err_t_ESLURM/g' job_submit_plugin/src/job_submit_bank.rs
```

Note that BSD sed syntaxt is slightly different from GNU sed syntaxt. On a Linux box, I would have run:

```
$ sed -i 's/ESLURM/slurm_err_t_ESLURM/g' job_completion_plugin/src/jobcomp_bank.rs
$ sed -i 's/ESLURM/slurm_err_t_ESLURM/g' job_submit_plugin/src/job_submit_bank.rs
```

Additionally, there are API changes for the jobcomp plugin (see <https://github.com/SchedMD/slurm/blob/slurm-21.08/src/plugins/jobcomp/none/jobcomp_none.c> for an example). I made the following changes to `job_completion_plugin/src/jobcomp_bank.rs`:

* replaced `slurm_jobcomp_set_location` with `jobcomp_p_set_location`
* replaced `slurm_jobcomp_log_record` with `jobcomp_p_log_record`
* replaced `slurm_jobcomp_get_jobs` with `jobcomp_p_get_jobs`
* replaced `slurm_jobcomp_archive` with `jobcomp_p_archive`
* deleted function `slurm_jobcomp_strerror`
* deleted function `slurm_jobcomp_strerror`

## Developing Environment

I modified the [Dockerfile](https://github.com/lbnl-science-it/slurm-banking-plugins/blob/slurm-21-08-6/docker/dev/Dockerfile) for the dev container, which now uses [giovtorres/docker-centos7-slurm:21.08.6](https://github.com/giovtorres/docker-centos7-slurm) as the base. I also lightly modified the `docker-dev` stanza of the [Makefile](https://github.com/lbnl-science-it/slurm-banking-plugins/blob/slurm-21-08-6/Makefile)

```
	docker build -f docker/dev/Dockerfile -t slurm-banking-plugins-centos7-dev:21.08.6 .
	docker run \
		-v $(shell pwd):/slurm-banking-plugins \
		-it -h slurmctl slurm-banking-plugins-centos7-dev:21.08.6
```

Now if you run

```
$ make docker-dev
```

It will build a docker image for `slurm-banking-plugins-centos7-dev:21.08.6`, launch a container based on that image, and drop you into a shell on the container. From there, you can download the source code for Slurm, then build the Slurm bankding plugins:

```
SLURM_TAG=slurm-21-08-6-1
git clone -b ${SLURM_TAG} --single-branch --depth=1 https://github.com/SchedMD/slurm.git
cd slurm
./configure --prefix=/usr --sysconfdir=/etc/slurm --with-mysql_config=/usr/bin --libdir=/usr/lib64
cd ..
make
```
