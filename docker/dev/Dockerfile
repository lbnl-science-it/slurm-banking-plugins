FROM giovtorres/docker-centos7-slurm:21.08.6
RUN yum clean all && yum install -y glibc clang llvm-devel # cargo
RUN curl -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable
# RUN echo "JobSubmitPlugins=job_submit/slurm_banking" >> /etc/slurm/slurm.conf \
#     && echo "JobCompType=jobcomp/slurm_banking" >> /etc/slurm/slurm.conf
# ADD . /slurm-banking-plugins
WORKDIR /slurm-banking-plugins
# RUN git submodule update --init --recursive
