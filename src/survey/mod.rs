mod start;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct SurveyAnswer {
    _id: String,
    submitter: String,
    status: Status,
    category: Categories,
    subjective: Vec<u8>,
    objective: Vec<String>,
    points: Vec<u8>,
    judged: bool,
    judges: Vec<String>
}

#[derive(Serialize, Deserialize)]
struct SurveyLog {
    aid: String,
    modifier: String,
    category: Categories,
    actions: Actions,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    time: DateTime<Utc>,
}

#[derive(Clone, Serialize, Deserialize)]
enum Categories {
    Builder,
    Redstoner,
    Technician,
    Modeler,
    Advertiser,
}

#[derive(Serialize, Deserialize)]
enum Actions {
    Start,
    Update,
    HandIn,
}

#[derive(Serialize, Deserialize)]
enum Status {
    Completing,
    Pending,
    Done
}