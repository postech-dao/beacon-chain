pub mod message;

use serde::{Deserialize, Serialize};

/// Tagged by its dept. and the join/admission year. (e.g., (CSED, `2022`))
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum AlumniType {
    Undergradute(Department, u16),
    Gradute(Department, u16),
    Professor(Department, u16),
}

/// Tagged by the join day. (e.g., `2022-09-02`)
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum CommitteeType {
    Committee(String),
    Observer(String),
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum Department {
    MATH,          // Department of Mathematics
    PHYS,          // Department of Physics
    CHEM,          // Department of Chemistry
    LIFE,          // Department of Life Sciences
    AMSE,          // Department of Metarials Science and Engineering
    MECH,          // Department of Mechanical Engineering
    IMEN,          // Department of Industrial and Management Engineering
    EECE,          // Department of Electrical Engineering
    CSED,          // Department of Computer Science and Engineering
    CHEB,          // Department of Chemical Engineering
    CITE,          // Department of Convergence IT Engineering
    SEMI,          // Department of Semiconductor Engineering
    MSUS,          // MUENJAE School of Undergraduate Studies
    Other(String), // Other Graduate Schools
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Member {
    pub name_kor: String,
    pub name_eng: String,
    pub alumni_type: Vec<AlumniType>,
    pub email: String,
    pub homepage: String,
    pub github_id: String,
    pub committee_type: CommitteeType,
    pub achievements: Vec<Achievement>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Achievement {
    pub name: String,
    pub description: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_sample1() {
        let member = Member {
            name_kor: "김철수".to_string(),
            name_eng: "Cheolsoo Kim".to_string(),
            alumni_type: vec![AlumniType::Undergradute(Department::CSED, 2022)],
            email: "ckim@dummy.co.kr".to_string(),
            homepage: "ckim.dummy.co.kr".to_string(),
            github_id: "ckim".to_string(),
            committee_type: CommitteeType::Committee("2022-09-02".to_string()),
            achievements: vec![Achievement {
                name: "Good Job".to_string(),
                description: "Did a great job".to_string(),
            }],
        };
        let x = serde_json::to_string(&member).unwrap();
        println!("{}", x);
    }
}
