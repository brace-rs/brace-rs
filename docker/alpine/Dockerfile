FROM alpine:3.9

ARG BUILD_ID
ARG SOURCE_URL="https://dev.azure.com/brace-rs/brace/_apis/build/builds/${BUILD_ID}/artifacts?artifactName=brace-x86_64-unknown-linux-musl&api-version=5.1-preview.5&%24format=zip"

LABEL \
  build_id="$BUILD_ID" \
  maintainer="Daniel Balcomb <daniel.balcomb@gmail.com>" \
  source_url="$SOURCE_URL"

RUN \
  set -ex; \
  cd "$(mktemp -d)"; \
  wget -qO ./brace-x86_64-unknown-linux-musl.zip "$SOURCE_URL"; \
  unzip ./brace-x86_64-unknown-linux-musl.zip; \
  tar -xzf ./brace-x86_64-unknown-linux-musl/brace-x86_64-unknown-linux-musl.tar.gz; \
  mv brace /usr/local/bin/brace; \
  rm -rf /tmp/*; \
  :

EXPOSE 80

ENTRYPOINT ["/usr/local/bin/brace"]

CMD ["web", "run"]
