use personnel::Candidate;

fn main() {
    let mut candidates: Vec<Candidate> = Candidate::load_candidate_file();

    candidates.sort_by(|a, b| b.candidate_score().cmp(&a.candidate_score()));
}

#[cfg(test)]
mod test;
