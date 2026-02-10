# bibl
rust-powered bibliomancy for communing with the cyberspirits

currently a quick and dirty implementation because my partner made a joke and I always take things to extremes. in Rust because only that language can truly commune with the other world.

usage:

`bibl process in.txt out.txt` - takes a text file and breaks it into individual sentences, separating on punctuation. saves the total number of lines as the first line.

`bibl reading filename context` - opens a processed text file, finds a random line, and returns `context` lines of context (before and after). I recommend 3, you do you.

licence: [CC BY-NC-SA 4.0 Intl](https://creativecommons.org/licenses/by-nc-sa/4.0/deed.en). Do what you like with it, just credit me, don't make money off it, and share your work on the same licence.

anti-ai statement: i do not use generative AI in any stage of production, and you are not to upload anything i create to any ai/ml system for training, repurposing/modification, or any other purpose.

...i mean seriously. do you want to anger the cybergods? that is how you anger the cybergods.

example:
```
~/bibl$ ./bibl process shrek.txt shrek.out
~/bibl$ ./bibl reading shrek.out 3
File has 1750 lines. Chosen line is number 1548. Viewing lines 1545 to 1551:
1545 -- -Well, you know.
1546 -- You were always me, me, me.
1547 -- Well, guess what?
1548 -- Now it's my turn!
1549 -- So you just shut up and pay attention!
1550 -- You are mean to me, you insult me, you don't appreciate anything that I do!
1551 -- You're always pushing me around or pushing me away.```
