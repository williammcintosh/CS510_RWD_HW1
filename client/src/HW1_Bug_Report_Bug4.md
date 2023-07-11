# Bug 4

## Title

Deleting destroys all questions upto arg id

## Description

Calling POST multiple times would create the number
of questions in the list as there are times that the
POST/question was called. When we remove one of them
using the id= argument, we'd expect that only that
single question is removed from the list. Instead,
calling this api removes all questions in the list
upto the id that was entered as the argument.

## Steps To Reproduce

1. Restart the backend.
2. Start the client script.
3. Call POST
4. Call POST
5. Call POST
6. Call POST
7. Call DELETE/question$question_id=1
8. Call GET/questions
9. Read output

## Expected Behavior

When we delete id=1, we expect that only the id which
matches the same number as the argument input, being
the number 1, would be removed from the vector of questions. Instead,
what this call DELETE/questions$question_id=1 does is
remove all questions upto the id=1.

## Actual Output

At the final print out, we would expect to see ids 0,2,3,4 
with id 1 being omitted from the vector of questions, since we deleted it.

```
POST Question: {"id":0,"title":"Bug Content 0","content":"Bug Question 0","tags":["Bug Tag 0"]}
POST Question: {"id":1,"title":"Bug Content 1","content":"Bug Question 1","tags":["Bug Tag 1"]}
POST Question: {"id":2,"title":"Bug Content 2","content":"Bug Question 2","tags":["Bug Tag 2"]}
POST Question: {"id":3,"title":"Bug Content 3","content":"Bug Question 3","tags":["Bug Tag 3"]}
POST Question: {"id":4,"title":"Bug Content 4","content":"Bug Question 4","tags":["Bug Tag 4"]}
DELETE question id=1: 

GET All Questions #1: 
[{"id":1,"title":"Bug Content 1","content":"Bug Question 1","tags":["Bug Tag 1"]},{"id":1,"title":"Default Question","content":"Default Content","tags":["default"]}]
```