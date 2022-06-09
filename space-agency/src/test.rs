use personnel::{AstronautJob, Candidate};

#[test]
fn test_candidate_score_two_jobs() {
    let c = Candidate {
        primary_job: AstronautJob::Biogeochemist,
        secondary_job: Some(AstronautJob::Medic),
        age: 25,
        health: 100,
    };
    assert_eq!(
        c.candidate_score(),
        ((((251 * 277) % 577) + 100) * 25) % 3929
    )
}

#[test]
fn test_candidate_score_one_job() {
    let c = Candidate {
        primary_job: AstronautJob::RoverOp,
        secondary_job: None,
        age: 20,
        health: 150,
    };
    assert_eq!(
        c.candidate_score(),
        ((((281 * 281) % 577) + 150) * 20) % 3929
    )
}

#[test]
fn test_candidate_score_zero_age() {
    let c = Candidate {
        primary_job: AstronautJob::Biogeochemist,
        secondary_job: Some(AstronautJob::Medic),
        age: 0,
        health: 100,
    };
    assert_eq!(c.candidate_score(), 0)
}

#[test]
fn test_candidate_ranking() {
    let c1 = Candidate {
        primary_job: AstronautJob::Biogeochemist,
        secondary_job: Some(AstronautJob::Medic),
        age: 0,
        health: 50,
    };
    let c2 = Candidate {
        primary_job: AstronautJob::Biogeochemist,
        secondary_job: Some(AstronautJob::Medic),
        age: 1,
        health: 100,
    };
    let c3 = Candidate {
        primary_job: AstronautJob::Biogeochemist,
        secondary_job: Some(AstronautJob::Medic),
        age: 1,
        health: 150,
    };

    let mut v: Vec<Candidate> = vec![c1, c2, c3];

    v.sort_by(|a, b| b.candidate_score().cmp(&a.candidate_score()));

    assert_eq!(v[0].health, 150);
    assert_eq!(v[1].health, 100);
    assert_eq!(v[2].age, 0);
}
