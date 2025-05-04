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
pub mod p067;
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
pub mod p032;
pub mod p031;
pub mod p033;
pub mod p034;
pub mod p035;
pub mod p036;
pub mod p051;

// Problem registry with status
pub static PROBLEMS: LazyLock<HashMap<usize, ProblemInfo>> = LazyLock::new(|| {
    let mut problems = HashMap::new();
    
    // Add all problems with their status
    problems.insert(1, ProblemInfo { solver: p001::main, status: ProblemStatus::Complete });
    problems.insert(2, ProblemInfo { solver: p002::main, status: ProblemStatus::Complete });
    problems.insert(3, ProblemInfo { solver: p003::main, status: ProblemStatus::Complete });
    problems.insert(4, ProblemInfo { solver: p004::main, status: ProblemStatus::Complete });
    problems.insert(5, ProblemInfo { solver: p005::main, status: ProblemStatus::Complete });
    problems.insert(6, ProblemInfo { solver: p006::main, status: ProblemStatus::Complete });
    problems.insert(7, ProblemInfo { solver: p007::main, status: ProblemStatus::Complete });
    problems.insert(8, ProblemInfo { solver: p008::main, status: ProblemStatus::Complete });
    problems.insert(9, ProblemInfo { solver: p009::main, status: ProblemStatus::Complete });
    problems.insert(10, ProblemInfo { solver: p010::main, status: ProblemStatus::Complete });
    problems.insert(11, ProblemInfo { solver: p011::main, status: ProblemStatus::Complete });
    problems.insert(12, ProblemInfo { solver: p012::main, status: ProblemStatus::Complete });
    problems.insert(13, ProblemInfo { solver: p013::main, status: ProblemStatus::Complete });
    problems.insert(14, ProblemInfo { solver: p014::main, status: ProblemStatus::Complete });
    problems.insert(15, ProblemInfo { solver: p015::main, status: ProblemStatus::Complete });
    problems.insert(16, ProblemInfo { solver: p016::main, status: ProblemStatus::Complete });
    problems.insert(17, ProblemInfo { solver: p017::main, status: ProblemStatus::Complete });
    problems.insert(18, ProblemInfo { solver: p018::main, status: ProblemStatus::Complete });
    problems.insert(19, ProblemInfo { solver: p019::main, status: ProblemStatus::Complete });
    problems.insert(20, ProblemInfo { solver: p020::main, status: ProblemStatus::Complete });
    problems.insert(21, ProblemInfo { solver: p021::main, status: ProblemStatus::Complete });
    problems.insert(22, ProblemInfo { solver: p022::main, status: ProblemStatus::Complete });
    problems.insert(23, ProblemInfo { solver: p023::main, status: ProblemStatus::Complete });
    problems.insert(24, ProblemInfo { solver: p024::main, status: ProblemStatus::Complete });
    problems.insert(25, ProblemInfo { solver: p025::main, status: ProblemStatus::Complete });
    problems.insert(26, ProblemInfo { solver: p026::main, status: ProblemStatus::Complete });
    problems.insert(27, ProblemInfo { solver: p027::main, status: ProblemStatus::Inefficient });
    problems.insert(28, ProblemInfo { solver: p028::main, status: ProblemStatus::Complete });
    problems.insert(29, ProblemInfo { solver: p029::main, status: ProblemStatus::Complete });
    problems.insert(30, ProblemInfo { solver: p030::main, status: ProblemStatus::Complete });
    problems.insert(31, ProblemInfo { solver: p031::main, status: ProblemStatus::Complete });
    problems.insert(32, ProblemInfo { solver: p032::main, status: ProblemStatus::Inefficient });
    problems.insert(33, ProblemInfo { solver: p033::main, status: ProblemStatus::Complete });
    problems.insert(34, ProblemInfo { solver: p034::main, status: ProblemStatus::Inefficient });
    problems.insert(35, ProblemInfo { solver: p035::main, status: ProblemStatus::Inefficient });
    problems.insert(36, ProblemInfo { solver: p036::main, status: ProblemStatus::Complete });
    problems.insert(51, ProblemInfo { solver: p051::main, status: ProblemStatus::Incomplete});
    problems.insert(67, ProblemInfo { solver: p067::main, status: ProblemStatus::Complete });
    
    problems
});