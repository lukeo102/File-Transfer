# File-Transfer
A simple file transfer program written in Rust

# TODO
## Common
- Protocols
  - Server - client messaging
  - File sending/receiving
    - Sending
      - Open file
      - Read bytes from file
        - Encode?
      - Send bytes
    - Receiving
      - Open file
      - Read bytes from buffer
      - Write to file
    - Resume interrupted downloads/uploads
    - Checksum validation
    - Check file exists
- Logging
  - Colours
  - Overwrite a line (progress bar(s))
- File syncing - maybe
  - Monitor file/directory changes
- Sockets
## Client
- Init connection with server
- GUI/TUI
  - Directory listing
    - Get folders
    - Get files
## Server
- Send info on available files/directories
- Trash folder? (14 days delete?)