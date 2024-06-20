# Readme

This is a simplified version of how mio works. It handles just a simple event queue and HTTP on a given socket.

## Structure

The project is structured in 2 main parts:

- `delayserver`: This is the main server that listens on a given port and waits for incoming connections. It then waits for the requested duration specified in the param before responding. For more details on how to use it, please refer to the [README](./delayserver/README.md) in the delayserver folder.
- `equeue`: this is the simple event queue built on top of equeue (so mainly compatible with linux systems). It sends some messages to the delayserver and waits for the response. For more details on how to use it, please refer to the [README](./equeue/README.md) in the equeue folder.

Because this project is platform dependent, it is best to run it in a docker container using docker compose in order to allow to anybody to run it on their machine and try it.

## Commands

Before starting check if you have docker composed installed on your machine `docker-compose -v`, if not install it. To build and run both the services use:

```bash
# Run the following commands to start the server
docker-compose build

# Run the following command to start the server
docker-compose up
```

## Notes

If you are on Linux you can run the servers directly but you have to replace the `equeue` and `delay_server` in the urls of both services with `localhost` and then you should be able to run the servers directly.

## Greetings

A huge thanks to Carl Fredrik Samson for the Asynchronous Programming in Rust book that guided me through the creation of this project.
