# Title

Deleting Causes Duplicate Unique IDs with POST

## Description

Calling POST twice, and then deleting post with
id=1, and then calling GET/questions,
makes it to where two posts have the same unique id.

## Steps To Reproduce

1. Restart the backend.
2. Start the client script.
3. Call POST
4. Call POST
5. Call DELETE/question$question_id=1
6. Call GET/questions
7. Read output

## Expected Behavior

This Bug #3 bypasses the understood Bug #1, that calling
GET/questions creates a post, we know that when a new
post is created that each id of each post should be unique
and should be simply the length of the vector of questions.
Here, with Bug #3 we see that when deleting post #1 and
then calling GET/questions, that a new post is created
both containing id=1.

## Actual Output

At the final print out, each of the two posts in the
database of posts has id=1.

```
GET All Questions #1: 
[{"id":0,"title":"Default Question","content":"Default Content","tags":["default"]},{"id":1,"title":"Default Question","content":"Default Content","tags":["default"]},{"id":2,"title":"Default Question","content":"Default Content","tags":["default"]}]
DELETE question id=1: 

GET All Questions #2: 
[{"id":1,"title":"Default Question","content":"Default Content","tags":["default"]},{"id":1,"title":"Default Question","content":"Default Content","tags":["default"]}]
```