//! Language enum and basic metadata.

use serde::{Deserialize, Serialize};

/// All languages supported by Time Warp Rusted.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Language {
    Basic,
    Pilot,
    Logo,
    C,
    Pascal,
    Prolog,
    Forth,
}

impl Language {
    /// Human-readable display name.
    pub fn friendly_name(&self) -> &'static str {
        match self {
            Language::Basic  => "BASIC",
            Language::Pilot  => "PILOT",
            Language::Logo   => "Logo",
            Language::C      => "C",
            Language::Pascal => "Pascal",
            Language::Prolog => "Prolog",
            Language::Forth  => "Forth",
        }
    }

    /// File extension (without leading dot).
    pub fn extension(&self) -> &'static str {
        match self {
            Language::Basic  => "bas",
            Language::Pilot  => "pilot",
            Language::Logo   => "logo",
            Language::C      => "c",
            Language::Pascal => "pas",
            Language::Prolog => "pl",
            Language::Forth  => "f",
        }
    }

    /// Derive language from file extension.
    pub fn from_extension(ext: &str) -> Option<Language> {
        match ext.to_lowercase().trim_start_matches('.') {
            "bas" | "basic" => Some(Language::Basic),
            "pil" | "pilot" => Some(Language::Pilot),
            "logo" | "lg"   => Some(Language::Logo),
            "c"  | "h" | "cpp" | "hpp" | "cc" | "cxx" => Some(Language::C),
            "pas" | "pascal" | "pp" | "dpr" | "lpr" | "p" => Some(Language::Pascal),
            "pl"  | "pro" | "prolog" => Some(Language::Prolog),
            "fth" | "4th" | "fs" | "forth" | "f" => Some(Language::Forth),
            _ => None,
        }
    }

    /// Return all variants.
    pub fn all() -> &'static [Language] {
        &[
            Language::Basic,
            Language::Pilot,
            Language::Logo,
            Language::C,
            Language::Pascal,
            Language::Prolog,
            Language::Forth,
        ]
    }

    /// Default sample program for each language.
    pub fn sample_program(&self) -> &'static str {
        match self {
            Language::Basic => r#"10 PRINT "Hello from BASIC!"
20 FOR I = 1 TO 5
30   PRINT "Number: "; I
40 NEXT I
50 END
"#,
            Language::Pilot => r#"T:Hello from PILOT!
T:What is your name?
A:NAME
T:Hello, #NAME!
E:
"#,
            Language::Logo => r#"REPEAT 4 [FORWARD 100 RIGHT 90]
"#,
            Language::C => r#"#include <stdio.h>

int main() {
    int i;
    printf("Hello from C!\n");
    for (i = 1; i <= 5; i++) {
        printf("Number: %d\n", i);
    }
    return 0;
}
"#,
            Language::Pascal => r#"program Hello;
var
  i : integer;
begin
  writeln('Hello from Pascal!');
  for i := 1 to 5 do
    writeln('Number: ', i);
end.
"#,
            Language::Prolog => r#"% Prolog facts
parent(tom, bob).
parent(bob, ann).
ancestor(X, Y) :- parent(X, Y).
ancestor(X, Y) :- parent(X, Z), ancestor(Z, Y).

?- ancestor(tom, ann).
"#,
            Language::Forth => r#": SQUARE DUP * ;
: CUBE DUP SQUARE * ;
5 SQUARE . CR
3 CUBE  . CR
"#,
        }
    }
}

impl std::fmt::Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.friendly_name())
    }
}
