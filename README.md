# SMFNotify

Announce new threads and posts in a [Simple Machines
Forum](http://simplemachines.org/) instance by sending HTTP requests to
a configurable webhook endpoint.


## Usage

SMFNotify is intended to be invoked regularly, for example every few
minutes (depending on the activity of the monitored forum and how
quickly people want to be informed about new posts).

Each invocation fetches the feed, looks for new entries, and calls the
webhook for each one before exiting.

A configuration file is mandatory to run it:

```sh
$ smfnotify --config config_example.toml
```

An example configuration file is included as ``config_example.toml`` and
looks like this:

```toml
feed_url = "https://forum.example.com/index.php?action=.xml;type=atom"
feed_cookie_value = "<insert value of your 'SMFCookie10' cookie>"
feed_max_new_entries = 5

last_processed_id_filename = "last_processed_id"

webhook_text_template = "{author} posted to forum: \"{title}\" - <{url}>"
webhook_url = "http://127.0.0.1:8080/<your-webhook-path>"

interval_in_seconds = 120
```

To obtain the cookie value, log in to the forum, then use your web
browser's debugger or an extension to look up the cookie named
``SMFCookie10``.

The interval is optional. If it is not set, SMFNotify will only fetch
and potentially notify about new entries just once, then exit.


## Docker

Both a ``Dockerfile`` and a ``docker-compose.yaml`` are included to
support running SMFNotify with Docker and Docker Compose.

Be sure to provide:

- a configuration file named ``config.toml`` (and configure an interval)
  and
- a ``last_processed_id`` file (can be empty; create with ``touch
  last_processed_id``)

to avoid issues.


## Background

SMFNotify is a port of a Python script I wrote in October 2015.


## License

SMFNotify is licensed under the MIT license.


## Copyright

Copyright 2022-2023 Jochen Kupperschmidt
