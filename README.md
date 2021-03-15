# sushii-rules

Rules engine for processing Discord events with user configurable logic. In very
early development and mostly experimentation.

## Rule Configuration

1. Trigger
   * Discord gateway events
   * Quota exceeded
2. Conditions (Different condition types per event)
   * Messages
     * message content -> String conditions
     * message created -> DateTime conditions
     * channel_id -> Channel + Integer conditions
     * etc.
   * Member join
     * username -> String conditions
     * isBot -> bool conditions
     * isVerifiedBot -> bool conditions
     * previousNumberOfJoins -> Integer conditions
   * etc.
3. Actions
   * Discord actions
     * Message
     * Add role
     * Ban
     * Kick
     * etc
   * sushii actions
     * Warn
     * Mute
   * Other actions
     * Sleep (sleeps longer than x minutes store in db and poll?)
     * trigger another rule? (prevent infinite looping, disallow recursive rules or add a TTL)
     * add to quota

## Rule Persistence

guild_rule_groups

* set of multiple rules
* a certain "feature" might contain a set of multiple rules

| id   | guild_id | name |
| ---- | -------- | ---- |
| uuid | bigint   | text |

guild_rules

* rules can only have 1 trigger

| id   | rule_group_id           | rule_name | trigger_event |
| ---- | ----------------------- | --------- | ------------- |
| uuid | fk guild_rule_groups.id | text      | text          |

guild_rule_conditions

* json data of all conditions for given rule

| id   | rule_id           | condition data |
| ---- | ----------------- | -------------- |
| uuid | fk guild_rules.id | json           |

guild_rule_actions

* json data of all action steps

| id   | condition_id                | actions data |
| ---- | --------------------------- | ------------ |
| uuid | fk guild_rule_conditions.id | json         |

## Conditions

Conditions (boolean statements, e.g. x contains y) are grouped by data types to
make it easier for reuse and organization.

* Primitives-ish
  * String
    * ==
    * startsWith
    * contains word from word list
    * languageIs/IsNot/IsIn/IsNotIn (language-api)
    * % uppercase
    * % non-alphanumeric letters
    * number of lines
    * length
    * etc
  * Integers
  * DateTime
  * bool
* Discord Types - should be just using the underlying ID / integer comparisons,
  but UI should show separately + a list of guild channels)
  * Channel
  * Role
* sushii types
  * Warns
    * number of warns
  * Mutes
  * Quotas (rate limiting with governor crate)
    * number in last x minutes
    * number in a row
  * Level

## Word List

For logical separation and easier data store for large lists, lists of words are
stored separately and conditions just reference a word list ID / name. These are
mostly just used for searches like x contains a word from word list (e.g.
for blacklisted words).  Larget lists should be searched with `aho-corasik` for
linear executing large number of word searches in linear time. These Aho–Corasick
FSM's should also be cached.
