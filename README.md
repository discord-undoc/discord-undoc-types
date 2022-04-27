# Discord API Types

A repository full of manually generated hand curated JSON files, which
contain the API Types that the Discord API returns.

Also did I mention that everything in `./src` can be parsed easily!
That means you can write a parser to generate classes/structs/enums/etc
for your Discord API wrapper or bot!

## Primitive Types

- `null`
- `string`
- `integer`
- `float`
- `bool`
- `array[{type}]`

`{type}` can either be a primitive type, advanced type or a special type

## Advanced Types

Advanced Types are PascalCased and are scattered around `./src`,
"advanced types" are basically the objects the API returns

All keys are sorted alphabetically in an advanced type, but the *types*
itself might be sorted by relevance

## Special Types

These types are just to make life easier

- `union[{types}]` Used when the `type` can be one or more primitive
  and/or advanced objects
- `enum[{enum}]` Used when the `type` is from an enum
- `snowflake` Used as an alias to `string`, mainly exists because people
  might want to have their own custom implementation of sorts for a
  snowflake type

## The Schema

This is the base schema:

```json
{name}: {
    "type": {type},
    "required": {true|false},
    "nullable": {true|false},
}
```

**`type`:** can either be a primitive or an advanced type\
**`required`:** if this is true then the field is required to be sent
in the payload\
**`nullable`:** if this is true, then the feild can be sent as `null`
in the payload

### Examples

#### Payload with fields

```json
"Hello": {
    "_trace": {
        "type": "array[string]",
        "required": true,
        "nullable": false
    },
    "heartbeat_interval": {
        "type": "integer",
        "required": true,
        "nullable": false
    }
}
```

#### Payload with NO fields

```json
"Heartbeat": {
    "type": "integer",
    "required": true,
    "nullable": true
}
```

## Notes

The payload format is not mentioned in any of the files in `./src`\
But it is understood that you interpret it as this:

```json
"GatewayPayload": {
    "d": {
        "type": "union[Identify, ConnectionProperties, Resume, Heartbeat, RequestGuildMembers, UpdateVoiceState, UpdatePresence, Hello, Ready, Resumed, Reconnect, InvalidSession, ChannelCreate, ChannelUpdate, ChannelDelete, ChannelPinsUpdate, ThreadCreate, ThreadUpdate, ThreadDelete, ThreadListSync, ThreadMemberUpdate, ThreadMembersUpdate, GuildCreate, GuildUpdate, GuildDelete, GuildBanAdd, GuildBanRemove, GuildEmojisUpdate, GuildStickersUpdate, GuildIntegrationsUpdate, GuildMemberAdd, GuildMemberRemove, GuildMemberUpdate, GuildMembersChunk, GuildRoleCreate, GuildRoleUpdate, GuildRoleDelete, GuildScheduledEventCreate, GuildScheduledEventUpdate, GuildScheduledEventDelete, GuildScheduledEventUserAdd, GuildScheduledEventUserRemove, IntegrationCreate, IntegrationUpdate, IntegrationDelete, InteractionCreate, InviteCreate, InviteDelete, MessageCreate, MessageUpdate, MessageDelete, MessageDeleteBulk, MessageReactionAdd, MessageReactionRemove, MessageReactionRemoveAll, MessageReactionRemoveEmoji, PresenceUpdate, StageInstanceCreate, StageInstanceDelete, StageInstanceUpdate, TypingStart, UserUpdate, VoiceStateUpdate, VoiceServerUpdate, WebhooksUpdate]",
        "required": true,
        "nullable": true
    },
    "op": {
        "type": "integer",
        "required": true,
        "nullable": false
    },
    "s": {
        "type": "union[integer, null]",
        "required": false,
        "nullable": true
    },
    "t": {
        "type": "union[string, null]",
        "required": false,
        "nullable": true
    }
},
```
