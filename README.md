This tool will iterate over all of the plugin manifests found at https://github.com/logseq/marketplace and
clone the repositories they refer to, locally. If the repositories have already been cloned, they will be updated.

I was motivated to do this so that I could have a corpus of examples to search through when making my own plugins, as the
logseq plugin API documentation is not very helpful at the moment. 

Here's a motivating screenshot, to illustrate the use of this:

![Example Usage in VSCode](example_usage.png)

I am searching the source code for every logseq plugin that is published on the marketplace for "find block". This will find
code usage and comments matching this phrase. I can then look through these results to get an idea on how to tackle whatever my
partiular problem about "finding blocks" is, with the benefit of seeing how others have solved similar problems. 

I think it's a fantastic way to leverage open source to make more open source, personally!