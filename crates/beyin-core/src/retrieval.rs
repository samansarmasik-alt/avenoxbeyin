use std::collections::{HashMap, HashSet};

/// Lowercase and normalize Turkish specific characters: İ -> i, I -> ı
pub fn normalize_turkish(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for ch in text.chars() {
        match ch {
            'İ' => out.push('i'),
            'I' => out.push('ı'),
            other => {
                for lower in other.to_lowercase() {
                    out.push(lower);
                }
            }
        }
    }
    out
}

/// Tokenize text into words, removing punctuation
pub fn tokenize(text: &str) -> Vec<String> {
    let norm = normalize_turkish(text);
    norm.split(|c: char| !c.is_alphanumeric() && c != '_')
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect()
}

#[derive(Debug, Clone)]
pub struct ScoredItem<'a> {
    pub id: &'a str,
    pub score: f64,
}

/// Simplified BM25 / token matching scorer for memory retrieval
pub fn rank_documents<'a>(
    query: &str,
    documents: &'a [(&'a str, &'a str)], // (id, text)
) -> Vec<ScoredItem<'a>> {
    let q_tokens = tokenize(query);
    if q_tokens.is_empty() || documents.is_empty() {
        return Vec::new();
    }

    let q_set: HashSet<String> = q_tokens.into_iter().collect();
    let n = documents.len() as f64;

    // Document frequencies
    let mut df: HashMap<String, usize> = HashMap::new();
    let mut doc_tokens: Vec<Vec<String>> = Vec::with_capacity(documents.len());
    let mut total_len: usize = 0;

    for &(_id, text) in documents {
        let tokens = tokenize(text);
        total_len += tokens.len();
        let unique_tokens: HashSet<&String> = tokens.iter().collect();
        for &t in &unique_tokens {
            if q_set.contains(t) {
                *df.entry(t.clone()).or_insert(0) += 1;
            }
        }
        doc_tokens.push(tokens);
    }

    let avgdl = (total_len as f64) / (n.max(1.0));
    let k1 = 1.2;
    let b = 0.75;

    let mut scores: Vec<ScoredItem<'a>> = Vec::with_capacity(documents.len());

    for (idx, &(id, _)) in documents.iter().enumerate() {
        let tokens = &doc_tokens[idx];
        let doc_len = tokens.len() as f64;
        let mut tf_map: HashMap<&str, usize> = HashMap::new();
        for t in tokens {
            if q_set.contains(t) {
                *tf_map.entry(t.as_str()).or_insert(0) += 1;
            }
        }

        let mut score = 0.0;
        for q_word in &q_set {
            if let Some(&tf) = tf_map.get(q_word.as_str()) {
                let doc_freq = *df.get(q_word).unwrap_or(&0) as f64;
                // Standard BM25 IDF
                let idf = ((n - doc_freq + 0.5) / (doc_freq + 0.5) + 1.0).ln();
                let tf_norm = (tf as f64 * (k1 + 1.0))
                    / (tf as f64 + k1 * (1.0 - b + b * (doc_len / avgdl.max(1.0))));
                score += idf * tf_norm;
            }
        }

        if score > 0.0 {
            scores.push(ScoredItem { id, score });
        }
    }

    scores.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
    scores
}
