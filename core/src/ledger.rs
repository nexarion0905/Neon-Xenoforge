// ledger.rs

// LogEntry structure to represent a single entry in the ledger.
#[derive(Debug, Clone)]
pub struct LogEntry {
    pub timestamp: String,
    pub data: String,
    pub hash: String,
}

// TemporalGraph structure to represent a temporal graph of log entries.
#[derive(Debug, Clone)]
pub struct TemporalGraph {
    pub entries: Vec<LogEntry>,
}

impl LogEntry {
    pub fn new(timestamp: String, data: String) -> Self {
        let hash = Self::calculate_hash(&timestamp, &data);
        LogEntry { timestamp, data, hash }
    }

    pub fn calculate_hash(timestamp: &str, data: &str) -> String {
        // Implement your hash function here, for example, using SHA-256
        // This is a placeholder for actual hash calculation.
        format!("{}-{}", timestamp, data)
    }
}

impl TemporalGraph {
    pub fn new() -> Self {
        TemporalGraph { entries: Vec::new() }
    }

    pub fn add_entry(&mut self, entry: LogEntry) {
        self.entries.push(entry);
    }
}
