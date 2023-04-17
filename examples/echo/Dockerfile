FROM docker.io/golang:1.20 as build-stage

# The build image is split into parts to make adjustments to the files
#   have a faster turn-around for the developer.  Normally, this would
#   lead to more layers in the final docker image.  However, as a build
#   stage image, many layers doesn't matter for the final image.

# Install base dependencies.
RUN apt-get update \
    && apt-get install -y python3-pip

# Setup the native-shell for use.
#   In a production environment, this would copy in a tarball and extract it.
COPY pyproject.toml /opt/native-shell/pyproject.toml
COPY src/ /opt/native-shell
RUN cd /opt/native-shell \
    && python3 -m pip install .

# Build our script
COPY examples/echo/ /opt/source
RUN python3 -m native_shell --out /opt/source /opt/source/hello-world.v1.yaml \
    && cd /opt/source \
    && make

# Final image.  Just the final go file.
FROM scratch

COPY --from=build-stage /go-echo /go-echo

ENTRYPOINT ["/go-echo"]
