FROM adrianbrink/rust:nightly

VOLUME /source
ADD . /source
WORKDIR /source

CMD ["cargo", "run"]
