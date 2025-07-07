use crate::definitions::models::{ChildAccount, Class};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct ParentProfile {
  pub email: String,
  #[serde(rename = "telPortable")]
  pub phone_number: String,
  #[serde(rename = "telPortableConjoint")]
  pub phone_number_partner: String,
  #[serde(rename = "eleves")]
  pub child: Vec<ChildAccount>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct StudentProfile {
  #[serde(rename = "sexe")]
  pub gender: String,
  #[serde(rename = "infoEDT")]
  pub timetable_information: String,
  #[serde(rename = "nomEtablissement")]
  pub school_name: String,
  #[serde(rename = "idEtablissement")]
  pub school_id: String,
  #[serde(rename = "rneEtablissement")]
  pub school_rne: String,
  #[serde(rename = "telPortable")]
  pub phone_number: String,
  #[serde(rename = "idReelEtab")]
  pub school_real_id: String,
  #[serde(rename = "photo")]
  pub profile_picture_path: String,
  #[serde(rename = "estApprenant")]
  pub is_apprentice: bool,
  #[serde(rename = "classe")]
  pub class: Class,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum Profile {
  Parent(ParentProfile),
  Student(StudentProfile),
}
