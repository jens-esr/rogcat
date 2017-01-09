// Copyright © 2016 Felix Obenhuber
// This program is free software. It comes without any warranty, to the extent
// permitted by applicable law. You can redistribute it and/or modify it under
// the terms of the Do What The Fuck You Want To Public License, Version 2, as
// published by Sam Hocevar. See the COPYING file for more details.

use std::io::stdin;
use super::node::Node;
use super::record::Record;

pub struct StdinReader;

impl Node<Record, ()> for StdinReader {
    fn new(_: ()) -> Result<Box<Self>, String> {
        Ok(Box::new(StdinReader {}))
    }

    fn start(&self, send: &Fn(Record), done: &Fn()) -> Result<(), String> {
        loop {
            let mut buffer = String::new();
            match stdin().read_line(&mut buffer) {
                Ok(len) => {
                    if len == 0 {
                        done();
                        return Ok(());
                    } else {
                        send(Record {
                            timestamp: Some(::time::now()),
                            raw: buffer.trim().to_string(),
                            ..Default::default()
                        })
                    }
                }
                Err(e) => return Err(format!("{}", e)),
            }
        }
    }
}