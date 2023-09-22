# Timr
A handy timecard calculator plus command line application to keep track of time spent on tasks 


![MIT](https://img.shields.io/badge/license-MIT-blue.svg) 

## Another time tracker CLI?

Having to submit a timecard for my daily work activites, I wanted to make a little tool 
that would do the annyoying time conversions for me 
(because I kept forgetting the name of the website that does it).

Then I thought:

<img src="https://i.imgflip.com/7yiby5.jpg" title="made at imgflip.com"/>

so I am.


## License

This repository is licensed under [MIT](http://opensource.org/licenses/MIT) 

## Features
TODO!

## Usage
TODO!

## Dependencies
anyhow https://github.com/dtolnay/anyhow

chrono https://github.com/chronotope/chrono

clap https://github.com/clap-rs/clap

rand https://github.com/rust-random/rand

serde https://github.com/serde-rs/serde

serde_json https://github.com/serde-rs/json



## TODO

 - [x] timecard calculator
 - [x] read / writing tasks to a file.
 - [x] range based task lookup
 - [ ] name based task lookup
 - [ ] get task(s) without an end time (in progress)
 - [ ] amend task
 - [ ] do something about starting new task with existing running task name

 9/22: so I got super lost in the sauce, and have gone way off track.
 
* one thought, have two files: one for active tasks, other for archive. would make lookup much faster.

 - incorperating a [TUI](https://github.com/ratatui-org/ratatui/blob/main/examples/README.md#user-input) would be a very interesting premise once I get the actual thing working.