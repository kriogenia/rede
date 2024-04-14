# FAQ

**Why is datetime not a valid value in any type?**

Datetimes are easy to read, but not that easy to translate again into a string
for the HTTP communication based on all the different formats that are possible.
Accepting just a string representation with the expected format is the smoother
way of handling it.

**Does rede support GraphQL**?

Yes, but right now it would require you some manual crafting. Just use `POST`,
a `body.raw` and don't forget to set a `Content-Type: application/graphql` header.
