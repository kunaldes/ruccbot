# ruccbot: a dumb discord bot

ruccbot is a Discord bot meant to replace a handful of simple features from https://github.com/IanMitchell/aquarius.

## Overview

ruccbot is built on [Poise](https://github.com/serenity-rs/poise), so it's limited to handling single commands. This means that the `same` module from aquarius can't be reimplemented here without dropping into a lower level abstraction.

## Running ruccbot

There is no public instance of this bot, so if you want to run it, you need to [register with Discord as a developer](https://discord.com/developers/docs/intro).

Then set the `DISCORD_TOKEN` environment variable to your app's token. Note that ruccbot requires the privileged Message Content gateway intent.