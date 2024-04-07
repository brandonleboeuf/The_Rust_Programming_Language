# minigrep
This searches a file for a spacific word and returns the lines that contain that word.

## Search for the word "to" (case insensitive)
`$ cargo run -- to poem.txt`

## Search for the word "to" (case sensitive)
`$ IGNORE_CASE=1 cargo run -- to poem.txt`

## Print the output to output.txt
`$ ❯ IGNORE_CASE=1 cargo run -- to pome.txt > output.txt`

## Run tests
`$ cargo test`