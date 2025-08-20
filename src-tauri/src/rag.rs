// RAG with simple BM25
// Use tantivy crate for indexing

use tantivy::{Index, collector::TopDocs, query::QueryParser, schema::*};

pub fn index_kb() -> Result<(), String> {
    // Create index in kb/
    Ok(())
}

pub fn search(query: &str) -> Result<Vec<(f64, String)>, String> {
    // Search index
    Ok(vec![])
}
