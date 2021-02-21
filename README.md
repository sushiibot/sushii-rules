# sushii-rules

Rules engine for processing Discord events with user configurable logic.

## Rule Configuration

1. Trigger (Discord gateway event)
2. Conditions (Different condition types per event)
   * Messages
     * message content -> String conditions
     * message created -> DateTime conditions
     * channel_id -> Channel + Integer conditions
     * etc.
   * Member join
     * username -> String conditions
     * 
   * etc.
3. Actions

## Conditions

Conditions (boolean statements, e.g. x contains y) are grouped by data types to
make it easier for reuse and organization.

* Primitives-ish
  * String
    * ==
    * startsWith
    * contains word from word list
    * languageIs (language-api)
    * etc
  * Integers
  * DateTime
* Discord Types - should be just using the underlying ID / integer comparisons,
  but UI should show separately + a list of guild channels)
  * Channel
  * Role

## Word List

For logical separation and easier data store for large lists, lists of words are
stored separately and conditions just reference a word list ID / name. These are
mostly just used for searches like x contains a word from word list (e.g.
for blacklisted words).  Larget lists should be searched with `aho-corasik` for
linear executing large number of word searches in linear time. These Aho–Corasick
FSM's should also be cached.
