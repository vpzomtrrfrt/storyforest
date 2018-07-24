FROM rustlang/rust:nightly-slim

RUN apt-get -qq update && apt-get install -y libpq-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/storyforest_api_server
COPY . .
RUN cargo install --path .
CMD [ "storyforest_api_server" ]
