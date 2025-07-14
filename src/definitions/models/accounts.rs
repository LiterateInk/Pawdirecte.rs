use crate::definitions::models::{Class, Profile};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Account {
  #[serde(rename = "idLogin")]
  pub id_login: u32,
  pub id: u32,
  pub uid: String,
  #[serde(rename = "identifiant")]
  pub username: String,
  #[serde(rename = "typeCompte")]
  pub kind: String,
  #[serde(rename = "codeOgec")]
  pub ogec_code: String,
  #[serde(rename = "main")]
  pub is_main: bool,
  #[serde(rename = "lastConnexion")]
  pub last_connection: String,
  #[serde(rename = "civilite")]
  pub civility: String,
  #[serde(rename = "prenom")]
  pub first_name: String,
  #[serde(rename = "particule")]
  pub prefix: String,
  #[serde(rename = "nom")]
  pub last_name: String,
  pub email: String,
  #[serde(rename = "isPrimaire")]
  pub is_primary_school: bool,
  #[serde(rename = "nomEtablissement")]
  pub school_name: String,
  #[serde(rename = "logoEtablissement")]
  pub school_logo_path: String,
  #[serde(rename = "couleurAgendaEtablissement")]
  pub school_agenda_color_hex: String,
  #[serde(rename = "dicoEnLigneLeRobert")]
  pub le_robert_online_dictionary: bool,
  #[serde(rename = "socketToken")]
  pub socket_token: String,
  #[serde(rename = "accessToken")]
  pub access_token: String,
  #[serde(skip)]
  pub modules: Vec<String>,
  #[serde(skip, rename = "parametresIndividuels")]
  pub individual_parameters: String,
  pub profile: Profile,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ChildAccount {
  pub id: u32,
  #[serde(rename = "prenom")]
  pub first_name: String,
  #[serde(rename = "nom")]
  pub last_name: String,
  #[serde(rename = "sexe")]
  pub gender: String,
  #[serde(rename = "infoEDT")]
  pub timetable_information: String,
  #[serde(rename = "photo")]
  pub profile_picture_path: String,
  #[serde(rename = "nomEtablissement")]
  pub school_name: String,
  #[serde(rename = "idEtablissement")]
  pub school_id: String,
  #[serde(rename = "idReelEtab")]
  pub school_real_id: String,
  #[serde(rename = "isPrimaire")]
  pub is_primary_school: bool,
  #[serde(skip)]
  pub modules: Vec<String>,
  #[serde(rename = "classe")]
  pub class: Class,
}
