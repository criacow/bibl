# bibl
rust-powered bibliomancy for communing with the cyberspirits

currently a quick and dirty implementation because my partner made a joke and I always take things to extremes. in Rust because only that language can truly commune with the other world.

usage:
`bibl process in.txt out.txt` - takes a text file and breaks it into individual sentences, separating on punctuation. saves the total number of lines as the first line.
`bibl reading filename context` - opens a processed text file, finds a random line, and returns `context` lines of context (before and after). I recommend 3, you do you.

licence: [CC BY-NC-SA 4.0 Intl](https://creativecommons.org/licenses/by-nc-sa/4.0/deed.en). Do what you like with it, just credit me, don't make money off it, and share your work on the same licence.
