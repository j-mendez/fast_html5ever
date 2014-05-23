/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::strbuf::StrBuf;
use util::atom::Atom;
use tokenizer::states;

// FIXME: already exists in Servo DOM
#[deriving(Eq, TotalEq, Clone, Show)]
pub struct Doctype {
    pub name: Option<StrBuf>,
    pub public_id: Option<StrBuf>,
    pub system_id: Option<StrBuf>,
    pub force_quirks: bool,
}

impl Doctype {
    pub fn new() -> Doctype {
        Doctype {
            name: None,
            public_id: None,
            system_id: None,
            force_quirks: false,
        }
    }
}

/// Attribute name; will eventually support namespaces.
#[deriving(Eq, TotalEq, Ord, TotalOrd, Clone, Show)]
pub struct AttrName {
    name: Atom,
}

impl AttrName {
    pub fn new(name: Atom) -> AttrName {
        AttrName {
            name: name,
        }
    }
}

impl Str for AttrName {
    fn as_slice<'t>(&'t self) -> &'t str {
        self.name.as_slice()
    }
}

#[deriving(Eq, TotalEq, Clone, Show)]
pub struct Attribute {
    pub name: AttrName,
    pub value: StrBuf,
}

#[deriving(Eq, TotalEq, Clone, Show)]
pub enum TagKind {
    StartTag,
    EndTag,
}

#[deriving(Eq, TotalEq, Clone, Show)]
pub struct Tag {
    pub kind: TagKind,
    pub name: Atom,
    pub self_closing: bool,
    pub attrs: Vec<Attribute>,
}

#[deriving(Eq, TotalEq, Clone, Show)]
pub enum Token {
    DoctypeToken(Doctype),
    TagToken(Tag),
    CommentToken(StrBuf),
    CharacterTokens(StrBuf),
    EOFToken,
    ParseError(~str),
}

pub trait TokenSink {
    /// Process a token.
    fn process_token(&mut self, token: Token);

    /// The tokenizer will call this after emitting any start tag.
    /// This allows the tree builder to change the tokenizer's state.
    fn query_state_change(&mut self) -> Option<states::State> {
        None
    }
}
