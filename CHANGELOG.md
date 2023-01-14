# Changelog


## 0.4.0 (2023-01-24)

- Added configuration property `feed_max_new_entries` to limit the
  number of feed items announced as new.

- Added missing Docker mount for file with last processed ID.

- Log success or failure of webhook calls instead of exiting.

- Updated clap to v4.1.1.


## 0.3.0 (2023-01-24)

- Added interval configuration property to repeatedly fetch and process
  new entries. Adds dependency on serde_with.

- Use proper logging. Output is written to stderr instead of stdout,
  timestamps and log levels are included.

- Added log level configuration property. Replaces command line option
  `-q`/`--quiet`.

- Added `Dockerfile` and Docker Compose file.

- Updated clap to v4.1.

- Updated feed-rs to v1.2.

- Updated ureq to v2.6.1.


## 0.2.0 (2022-07-17)

- Added option `-q`/`--quiet` to suppress output.

- Updated clap to v3.2.12.

- Updated feed-rs to v1.1.

- Updated ureq to v2.5.0.


## 0.1.0 (2022-05-08)

- First release
