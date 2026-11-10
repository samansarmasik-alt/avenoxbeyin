use beyin_core::{normalize_turkish, rank_documents, tokenize, Record, Visibility};

#[test]
fn test_turkish_normalization() {
    assert_eq!(normalize_turkish("İSTANBUL IŞIK"), "istanbul ışık");
    assert_eq!(normalize_turkish("ÇIĞLIK"), "çığlık");
}

#[test]
fn test_tokenization() {
    let tokens = tokenize("Merhaba, Dünya! İkinci Beyin V3.");
    assert_eq!(tokens, vec!["merhaba", "dünya", "ikinci", "beyin", "v3"]);
}

#[test]
fn test_record_serialization() {
    let raw = "{\n  \"id\": \"md-test1234\",\n  \"source\": \"notes/proje.md\",\n  \"text\": \"# Proje Notu\\nRust cekirdek entegrasyonu.\",\n  \"source_sha256\": \"abcdef123456\",\n  \"revision\": 1,\n  \"visibility\": \"internal\"\n}";

    let record: Record = serde_json::from_str(raw).expect("deserialization failed");
    assert_eq!(record.id, "md-test1234");
    assert_eq!(record.visibility, Visibility::Internal);
    assert_eq!(record.revision, 1);
}

#[test]
fn test_bm25_retrieval() {
    let docs = vec![
        ("doc1", "Bu not yapay zeka ve hafıza sistemleri hakkındadır."),
        ("doc2", "Alışveriş listesi: elma, armut, ekmek."),
        ("doc3", "İkinci beyin notları yerel dosyalarda tutulur."),
    ];

    let results = rank_documents("yapay zeka", &docs);
    assert!(!results.is_empty());
    assert_eq!(results[0].id, "doc1");

    let results_beyin = rank_documents("ikinci beyin", &docs);
    assert!(!results_beyin.is_empty());
    assert_eq!(results_beyin[0].id, "doc3");
}
