use personnel::Candidate;

fn main() {
    let mut candidates: Vec<Candidate> = Candidate::load_candidate_file();

    candidates.sort_by_key(|c| std::cmp::Reverse(c.candidate_score()));
}

#[cfg(test)]
mod test;
