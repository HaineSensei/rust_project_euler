use std::collections::HashMap;
use std::sync::LazyLock;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ProblemStatus {
    Complete,
    Inefficient,
    Incomplete,
}

#[derive(Clone)]
pub struct ProblemInfo {
    pub solver: fn() -> (),
    pub status: ProblemStatus,
}

pub mod p001;
pub mod p002;
pub mod p003;
pub mod p004;
pub mod p005;
pub mod p006;
pub mod p007;
pub mod p008;
pub mod p009;
pub mod p010;
pub mod p011;
pub mod p012;
pub mod p013;
pub mod p014;
pub mod p015;
pub mod p016;
pub mod p017;
pub mod p018;
pub mod p019;
pub mod p020;
pub mod p021;
pub mod p022;
pub mod p023;
pub mod p024;
pub mod p025;
pub mod p026;
pub mod p027;
pub mod p028;
pub mod p029;
pub mod p030;
pub mod p031;
pub mod p032;
pub mod p033;
pub mod p034;
pub mod p035;
pub mod p036;
pub mod p037;
pub mod p038;
pub mod p039;
pub mod p040;
pub mod p041;
pub mod p042;
pub mod p043;
pub mod p044;
pub mod p051;
pub mod p067;

include!(concat!(env!("OUT_DIR"), "/problems_generated.rs"));
