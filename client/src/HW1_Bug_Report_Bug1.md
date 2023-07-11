# Title

Calling GET/questions Makes a Questions Post

## Description

Calling GET/questions creates a blank question with
the term "default" in each of its fields, when this
call should only return an empty JSON of posted
questions, not create new ones.

## Steps To Reproduce

1. Restart the backend.
2. Start the client script.
3. Call GET/questions
4. Call GET/questions
5. Read output

## Expected Behavior

Here, since we never added a question or post to the vector of question structs,
we expect that calling "GET/questions" would result in an
empty JSON list of questions each time it is called.

## Actual Output

A new question with the text "Default..." for each field.

```
GET All Questions #1:
[{"id":0,"title":"Default Question","content":"Default Content","tags":["default"]}]
GET All Questions #2:
[{"id":0,"title":"Default Question","content":"Default Content","tags":["default"]},{"id":1,"title":"Default Question","content":"Default Content","tags":["default"]}]
```