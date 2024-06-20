## Structure

The client event queue is structured in 3 main parts:

- `ffi.rs`: This is the file that contains the FFI bindings to the C library.
- `poll.rs`: This is the file that contains the main logic of the event queue. Here are handled the poll and the event loop.
- `main.rs`: This is the part where we apply and check if the created event queue works, here we open 5 sockets and make 5 requests with a different delay then we open and event loop and poll until we receive a response that the server has finished processing the request.
