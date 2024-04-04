# The Cats in Chat - v2

The Cats in Chat (TCIC or sometimes just Cats) is a multifunction chatbot focussed around providing a wholesome well-rounded set of functions to elevate a Twitch channel's chat.

## Features
 - Nothing yet, I'm just setting things up.

### About

The Cats in Chat started as a personal project for me, then became something much larger, then I abandoned that for like 2 years (I got a job).
So I'm rewriting the core of the chatbot using Rust instead of Python.

By rewriting the core in Rust I'm able to:  
-  Take advantage of the memory-safety, speed, and other benefits of Rust,  
-  Build the infrastructure in the right way instead of slapping parts of it together as I require it
-  Include all of the stand-alone modules from v1 of TCIC "easily", so I won't really have to rebuild that much of the functionality (just like db and admin)
    - And this means I/you can write new modules in Rust and/or Python, which ~~is~~ will be nice.

