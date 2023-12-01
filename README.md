# kudos api

# Local Development

## Docker

### Pre-requisites

This Dockerfile requires BuildKit and buildx. BuildKit is an improved backend to replace the legacy builder. BuildKit is the default builder for users on Docker Desktop, and Docker Engine as of version 23.0.

### Build

To build the image, use:

```docker build . -t kudos-api```

### Run

```docker run -e DATABASE_URL=... -e HTTP_SERVER_HOST=... -e HTTP_SERVER_PORT=... kudos-api```

### Docker-compose

### Run

It builds the image if it's the first time, otherwise, it uses the latest built image.

```docker-compose up```

### Build and run

```docker-compose up --build```