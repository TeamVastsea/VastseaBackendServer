use bson::{DateTime, doc};
use mongodb::Collection;
use crate::MONGODB;
use crate::survey::{Actions, Categories, Status, SurveyAnswer, SurveyLog};
use crate::utils::uuid::generate_uuid;

async fn start_survey(submitter: String, category: Categories) -> Result<(), String> {

    let (started, _) = is_started(submitter.clone()).await;
    if started {
        return Err("Already started.".to_string());
    }

    let aid = generate_uuid();

    let collection: &Collection<SurveyLog> = &unsafe { MONGODB.as_ref() }.unwrap().collection("survey_log");
    let log_doc = SurveyLog {
        aid: aid.clone(),
        modifier: submitter.clone(),
        category: category.clone(),
        actions: Actions::Start,
        time: DateTime::now().to_chrono(),
    };
    collection.insert_one(log_doc, None).await.expect("Cannot write mongodb");

    let collection: &Collection<SurveyAnswer> = &unsafe{ MONGODB.as_ref() }.unwrap().collection("survey");
    let answer = SurveyAnswer{
        _id: aid,
        submitter,
        status: Status::Completing,
        category,
        subjective: vec![],
        objective: vec![],
        points: vec![],
        judged: false,
        judges: vec![],
    };
    collection.insert_one(answer, None).await.expect("Cannot write mongodb");

    Ok(())
}

async fn is_started(submitter: String) -> (bool, Option<Categories>) {
    let collection: &Collection<SurveyAnswer> = &unsafe { MONGODB.as_ref() }.unwrap().collection("survey");
    let doc = collection.find_one(doc! {"submitter": submitter, "status": "completing"}, None).await.expect("Cannot read mongodb");

    return if let Some(result) = doc {
        (true, Some(result.category))
    } else {
        (false, None)
    }
}