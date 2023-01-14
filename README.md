# SMFNotify

Announce new threads and posts in a [Simple Machines
Forum](http://simplemachines.org/) instance by sending HTTP requests to
a configurable webhook endpoint.


## Usage

SMFNotify is intended to be invoked regularly, for example as a cron job
executed every five minutes.

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

last_processed_id_filename = "last_processed_id"

webhook_text_template = "{author} posted to forum: \"{title}\" - <{url}>"
webhook_url = "http://127.0.0.1:8080/<your-webhook-path>"
```

To obtain the cookie value, log in to the forum, then use your web
browser's debugger or an extension to look up the cookie named
``SMFCookie10``.


## Background

SMFNotify is a port of a Python script I wrote in October 2015.


## License

SMFNotify is licensed under the MIT license.


## Copyright

Copyright 2022-2023 Jochen Kupperschmidt
