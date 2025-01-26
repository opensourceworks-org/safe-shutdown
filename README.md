# safe-shutdown

## introduction
This service manages the safe shutdown of the machine it's running on.

It takes a sentinel path as env var and immediately creates the sentinel file in that path.

A systemctl daemon should block shutdown if the sentinel file exists.


It's part of a bigger picture:

- interface: a button on a raspberry pi zero w with LCD display
- controller: a raspberry pi connected to the pc via ipmi/bmc and/or motherboard power/reset headers.
- manager: api running on the controller, handling power through the controller and a tuya smart socket.

This allows the manager to check if the pc is on, and if it's safe to turn it off, and if it's not, to notify the user to turn it off manually.

Runs inside a docker container which starts with the host.