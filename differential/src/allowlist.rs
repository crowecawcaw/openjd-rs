// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// Copyright by contributors to this project.
// SPDX-License-Identifier: (Apache-2.0 OR MIT)

//! The intentional-divergence allowlist.
//!
//! "Match Python" is not absolute: some divergences are deliberate and
//! desirable — the clearest example is turning Python's *silent integer
//! saturation* into an explicit Rust error, which is a fix, not a regression.
//! Without a record of those decisions, the harness would either be a nuisance
//! (flagging intended behavior forever) or unsafe (someone `#[allow]`-ing the
//! whole class and letting real bugs through with it).
//!
//! This allowlist is that record: a small, reviewed, in-repo table keyed by the
//! exact input, each entry carrying a human reason. A divergence is downgraded
//! to a pass *only if its input is listed*. Every entry is auditable in review;
//! adding one is a deliberate act with a written justification, exactly like a
//! tracked `#[allow]`.
//!
//! The table is loaded from `allowlist.json` next to the crate so it can be
//! edited without recompiling and diffed cleanly in a PR.

use std::collections::HashMap;
use std::path::Path;

use serde::Deserialize;

/// One reviewed, intentional divergence.
#[derive(Debug, Clone, Deserialize)]
pub struct AllowedDivergence {
    /// The exact expression source this entry excuses.
    pub input: String,
    /// Why the two implementations are allowed to differ here. Required — an
    /// entry without a reason is a smell, so the field is non-optional.
    pub reason: String,
    /// Optional tracking reference (ticket, spec section, PR) for the decision.
    #[serde(default)]
    pub reference: Option<String>,
}

/// The loaded allowlist, indexed by input for O(1) lookup.
#[derive(Debug, Default)]
pub struct Allowlist {
    by_input: HashMap<String, AllowedDivergence>,
}

#[derive(Debug, Deserialize)]
struct AllowlistFile {
    #[serde(default)]
    allowed: Vec<AllowedDivergence>,
}

impl Allowlist {
    /// Load from a JSON file. A missing file is treated as an empty allowlist —
    /// the safe default (nothing is excused).
    pub fn load(path: impl AsRef<Path>) -> Result<Self, String> {
        let path = path.as_ref();
        let text = match std::fs::read_to_string(path) {
            Ok(t) => t,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(Self::default()),
            Err(e) => return Err(format!("reading {}: {e}", path.display())),
        };
        let file: AllowlistFile =
            serde_json::from_str(&text).map_err(|e| format!("parsing {}: {e}", path.display()))?;
        let by_input = file
            .allowed
            .into_iter()
            .map(|a| (a.input.clone(), a))
            .collect();
        Ok(Self { by_input })
    }

    /// The conventional path: `allowlist.json` next to the crate manifest.
    pub fn load_default() -> Result<Self, String> {
        Self::load(Path::new(env!("CARGO_MANIFEST_DIR")).join("allowlist.json"))
    }

    /// Is this input a reviewed, intentional divergence?
    pub fn is_allowed(&self, input: &str) -> Option<&AllowedDivergence> {
        self.by_input.get(input)
    }

    pub fn len(&self) -> usize {
        self.by_input.len()
    }

    pub fn is_empty(&self) -> bool {
        self.by_input.is_empty()
    }
}
