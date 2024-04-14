# The Cats in Chat - v2

The Cats in Chat (TCIC or sometimes just Cats) is a multifunction chatbot focussed around providing a wholesome well-rounded set of functions to elevate a Twitch channel's chat.

## Features
 - Says HeyGuys when you turn it on

## How?

I'll Dockerize this all eventually but I'm developing this all in a VM anyway, so, until I need to deploy it somewhere, getting Docker working would just be less work being done on the bot -> this is contrary to many of my projects, trust me I've thought about it.

### Prerequisites
- Rust
- Python? 3.10+
- Postgres

The scripts in `_scripts` are all pretty straight-forward, it's easier for me than remembering the specific commands every time, even when there's nothing special about it.  I :

1) Create a `.env` at the top level of the workspace (where this README is) from `.env-example`. You'll need your own chatbot app (from [dev.twitch.tv](dev.twitch.tv)) with the correct redirects (which are shown in the env file)
1) Generate self-signed certificates using `create_new_certs.sh`, or (idk how yet, but), get real ones with LetsEncrypt.
1) Run `diesel_migrations.sh` to setup your database.
1) Run `run_admin.sh` to start running the Admin site.

    1) Visit https://$MY_LOCAL_DOMAIN:8443/
    1) Login, go through normal Twitch app approval process
    1) Open the "Bot Login" link: (When/if I make the website-activation method available again, those will all be run through TheCatsInChat but if you're running this on  your own, you're gonna be using your own bot account)

        a) in another browser/incognito window and do the login process with whatever account you want to use for your chatbot.  
        b) maybe, in the same window with the same account.  Not 100% sure this works.
    
1) Once you've got your credentials in place, you can shut down the Admin site (or not) and run `run_chatbot.sh`.

That's it?!

## About

The Cats in Chat started as a personal project for me, then became something much larger, then I abandoned that for like 2 years (I got a job).
So I'm rewriting the core of the chatbot using Rust instead of Python.

By rewriting the core in Rust I'm able to:  
-  Take advantage of the memory-safety, speed, and other benefits of Rust,  
-  Build the infrastructure in the right way instead of slapping parts of it together as I require it
-  Include all of the stand-alone modules from v1 of TCIC "easily", so I won't really have to rebuild that much of the functionality (just like db and admin)
    - And this means I/you can write new modules in Rust and/or Python, which ~~is~~ will be nice.

### Okay but who are you?

I'm Tasty, I stream on [Twitch](twitch.tv/tastyandthecats), so I wrote a bot for my channel.  People liked it, and now we're here.