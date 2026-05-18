use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use tracing::info;

/// Conversation memory manager
///
/// Stores conversation history for multiple sessions.
/// Each session has its own message history with a maximum length limit.
#[derive(Clone)]
pub struct ConversationMemory {
    /// Session ID -> List of conversation messages
    sessions: Arc<RwLock<HashMap<String, Vec<String>>>>,
    /// Maximum number of messages to keep per session (default: 20)
    max_history_per_session: usize,
}

impl Default for ConversationMemory {
    fn default() -> Self {
        Self {
            sessions: Arc::new(RwLock::new(HashMap::new())),
            max_history_per_session: 20,
        }
    }
}

impl ConversationMemory {
    /// Create a new conversation memory with default settings
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a new conversation memory with custom max history length
    pub fn with_max_history(max_messages: usize) -> Self {
        Self {
            sessions: Arc::new(RwLock::new(HashMap::new())),
            max_history_per_session: max_messages,
        }
    }

    /// Get conversation history for a session as a joined string
    ///
    /// # Arguments
    /// * `session_id` - The session identifier
    ///
    /// # Returns
    /// A string containing all messages joined by newlines, or empty string if session doesn't exist
    pub fn get_history(&self, session_id: &str) -> String {
        let sessions = self.sessions.read().unwrap();
        sessions
            .get(session_id)
            .map(|h| h.join("\n"))
            .unwrap_or_default()
    }

    /// Get conversation history as a vector of messages
    ///
    /// # Arguments
    /// * `session_id` - The session identifier
    ///
    /// # Returns
    /// A vector of message strings, or empty vector if session doesn't exist
    pub fn get_history_vec(&self, session_id: &str) -> Vec<String> {
        let sessions = self.sessions.read().unwrap();
        sessions.get(session_id).cloned().unwrap_or_default()
    }

    /// Add a message to a session's history
    ///
    /// # Arguments
    /// * `session_id` - The session identifier
    /// * `message` - The message to add (format: "Role: content")
    pub fn add_message(&self, session_id: &str, message: String) {
        let mut sessions = self.sessions.write().unwrap();
        let hist = sessions.entry(session_id.to_string()).or_default();
        hist.push(message);
        self.trim_history(hist);
    }

    /// Add user and assistant messages together
    ///
    /// # Arguments
    /// * `session_id` - The session identifier
    /// * `user_message` - The user's input message
    /// * `assistant_message` - The assistant's response message
    pub fn add_exchange(&self, session_id: &str, user_message: &str, assistant_message: &str) {
        let mut sessions = self.sessions.write().unwrap();
        let hist = sessions.entry(session_id.to_string()).or_default();
        hist.push(format!("User: {}", user_message));
        hist.push(format!("Assistant: {}", assistant_message));
        self.trim_history(hist);
    }

    /// Trim history to max length
    fn trim_history(&self, history: &mut Vec<String>) {
        if history.len() > self.max_history_per_session {
            let drain_count = history.len() - self.max_history_per_session;
            history.drain(0..drain_count);
        }
    }

    /// Clear conversation history for a specific session
    ///
    /// # Arguments
    /// * `session_id` - The session identifier
    pub fn clear_session(&self, session_id: &str) {
        let mut sessions = self.sessions.write().unwrap();
        sessions.remove(session_id);
        info!("Cleared conversation history for session: {}", session_id);
    }

    /// Clear all conversation histories for all sessions
    pub fn clear_all(&self) {
        let mut sessions = self.sessions.write().unwrap();
        sessions.clear();
        info!("Cleared all conversation histories");
    }

    /// Check if a session has any history
    ///
    /// # Arguments
    /// * `session_id` - The session identifier
    ///
    /// # Returns
    /// True if the session has at least one message
    pub fn has_history(&self, session_id: &str) -> bool {
        let sessions = self.sessions.read().unwrap();
        sessions
            .get(session_id)
            .map(|h| !h.is_empty())
            .unwrap_or(false)
    }

    /// Get the number of messages in a session
    ///
    /// # Arguments
    /// * `session_id` - The session identifier
    ///
    /// # Returns
    /// The number of messages, or 0 if session doesn't exist
    pub fn message_count(&self, session_id: &str) -> usize {
        let sessions = self.sessions.read().unwrap();
        sessions.get(session_id).map(|h| h.len()).unwrap_or(0)
    }

    /// Get all active session IDs
    ///
    /// # Returns
    /// A vector of all session IDs that have history
    pub fn get_all_sessions(&self) -> Vec<String> {
        let sessions = self.sessions.read().unwrap();
        sessions.keys().cloned().collect()
    }

    /// Get the maximum history length per session
    pub fn max_history_per_session(&self) -> usize {
        self.max_history_per_session
    }

    /// Set a new maximum history length (affects future trimming)
    pub fn set_max_history(&mut self, max_messages: usize) {
        self.max_history_per_session = max_messages;
        let mut sessions = self.sessions.write().unwrap();
        for history in sessions.values_mut() {
            if history.len() > max_messages {
                let drain_count = history.len() - max_messages;
                history.drain(0..drain_count);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_memory() {
        let memory = ConversationMemory::new();
        assert_eq!(memory.max_history_per_session(), 20);
        assert!(memory.get_all_sessions().is_empty());
    }

    #[test]
    fn test_add_and_get_history() {
        let memory = ConversationMemory::new();
        let session = "test-session";
        memory.add_message(session, "User: Hello".to_string());
        memory.add_message(session, "Assistant: Hi there!".to_string());
        let history = memory.get_history(session);
        assert!(history.contains("User: Hello"));
        assert!(history.contains("Assistant: Hi there!"));
        assert_eq!(memory.message_count(session), 2);
    }

    #[test]
    fn test_add_exchange() {
        let memory = ConversationMemory::new();
        let session = "test-session";
        memory.add_exchange(session, "What's 2+2?", "It's 4");
        let history = memory.get_history_vec(session);
        assert_eq!(history.len(), 2);
        assert_eq!(history[0], "User: What's 2+2?");
        assert_eq!(history[1], "Assistant: It's 4");
    }

    #[test]
    fn test_clear_session() {
        let memory = ConversationMemory::new();
        let session = "test-session";
        memory.add_message(session, "User: Hello".to_string());
        assert!(memory.has_history(session));
        memory.clear_session(session);
        assert!(!memory.has_history(session));
    }

    #[test]
    fn test_clear_all() {
        let memory = ConversationMemory::new();
        memory.add_message("session1", "User: Hi".to_string());
        memory.add_message("session2", "User: Hey".to_string());
        assert_eq!(memory.get_all_sessions().len(), 2);
        memory.clear_all();
        assert!(memory.get_all_sessions().is_empty());
    }

    #[test]
    fn test_max_history_limit() {
        let memory = ConversationMemory::with_max_history(3);
        let session = "test-session";
        for i in 0..10 {
            memory.add_message(session, format!("Message {}", i));
        }
        assert_eq!(memory.message_count(session), 3);
        let history = memory.get_history_vec(session);
        assert_eq!(history[0], "Message 7");
        assert_eq!(history[1], "Message 8");
        assert_eq!(history[2], "Message 9");
    }

    #[test]
    fn test_set_max_history() {
        let mut memory = ConversationMemory::with_max_history(10);
        let session = "test-session";
        for i in 0..10 {
            memory.add_message(session, format!("Message {}", i));
        }
        assert_eq!(memory.message_count(session), 10);
        memory.set_max_history(3);
        assert_eq!(memory.message_count(session), 3);
    }

    #[test]
    fn test_get_all_sessions() {
        let memory = ConversationMemory::new();
        memory.add_message("session-a", "Hello".to_string());
        memory.add_message("session-b", "World".to_string());
        memory.add_message("session-a", "Again".to_string());
        let sessions = memory.get_all_sessions();
        assert_eq!(sessions.len(), 2);
        assert!(sessions.contains(&"session-a".to_string()));
        assert!(sessions.contains(&"session-b".to_string()));
    }
}
