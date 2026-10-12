//! Last-resort guard (craftrules `standards/never-crash.md`): a panic that escapes a command —
//! any edit, import or export, from the UI, the CLI, MCP or the control channel — becomes an
//! error, and the document is kept as it was before the command (and saved for crash recovery
//! when a recovery directory is set). It is a safety net: commands still must not panic.

use std::panic::{AssertUnwindSafe, catch_unwind};

use serde_json::Value;

use crate::{EngineError, Result, Session};

/// Text of a panic payload (`panic!` with a literal or a formatted message).
pub fn panic_message(payload: &(dyn std::any::Any + Send)) -> String {
    payload
        .downcast_ref::<&str>()
        .map(|s| s.to_string())
        .or_else(|| payload.downcast_ref::<String>().cloned())
        .unwrap_or_else(|| "unknown panic".into())
}

/// Log every panic (message and location) before it unwinds, then run the previous hook.
/// Installed once per process, when the first [`Session`] is made.
pub fn install_panic_hook() {
    static ONCE: std::sync::Once = std::sync::Once::new();
    ONCE.call_once(|| {
        let previous = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |info| {
            let at = info.location().map(|l| format!(" at {}:{}", l.file(), l.line())).unwrap_or_default();
            log::error!("panic{at}: {}", panic_message(info.payload()));
            previous(info);
        }));
    });
}

impl Session {
    /// Run command `id` (`f`) inside the guard.
    pub(crate) fn guarded(&mut self, id: &str, f: impl FnOnce(&mut Session) -> Result<Value>) -> Result<Value> {
        let before = self.active().map(|d| (d.uid, d.doc.clone(), d.selection.clone(), d.revision, d.history.clone(), d.interaction.clone()));
        let journal_len = self.journal.len();
        let requests_len = self.ui_requests.len();
        let depth = self.depth;
        match catch_unwind(AssertUnwindSafe(|| f(self))) {
            Ok(r) => r,
            Err(payload) => {
                self.depth = depth;
                let msg = panic_message(payload.as_ref());
                // Keep the document as it was before the command.
                if let (Some((uid, doc, selection, revision, history, interaction)), Some(st)) = (before, self.active_mut())
                    && st.uid == uid
                {
                    st.doc = doc;
                    st.selection = selection;
                    // A nested command can have pushed undo, cleared redo or started an
                    // interaction before a later step panics. Restore the checkpoint.
                    st.history = history;
                    st.interaction = interaction;
                    st.revision = revision.max(st.revision).saturating_add(1);
                }
                // A nested command may also have already appended an event or UI request.
                // Neither belongs to the failed outer command.
                self.journal.truncate(journal_len);
                self.ui_requests.truncate(requests_len);
                log::error!("command `{id}` panicked: {msg}");
                if let Some(dir) = self.recovery_dir.clone()
                    && let Err(e) = crate::recovery::save(self, &dir)
                {
                    log::error!("recovery save after `{id}` failed: {e}");
                }
                Err(EngineError::Internal(id.to_string(), msg))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::Session;

    #[test]
    fn panicking_nested_command_restores_undo_redo_and_side_effect_queues() {
        let mut s = Session::with_new();
        s.execute("slide.new", &json!({})).unwrap();
        s.execute("edit.undo", &json!({})).unwrap();
        let previous = s.doc().unwrap().doc.clone();
        let undo_len = s.doc().unwrap().history.undo.len();
        let redo_len = s.doc().unwrap().history.redo.len();
        let journal_len = s.journal.len();
        let requests_len = s.ui_requests.len();
        assert!(redo_len > 0, "test requires an existing redo entry");

        let error = s
            .guarded("test.nested-panic", |s| {
                s.execute("slide.new", &json!({}))?;
                s.ui_requests.push(crate::UiRequest::Pane { id: "unexpected".into() });
                panic!("rollback");
            })
            .unwrap_err();

        assert!(error.to_string().contains("rollback"));
        let state = s.doc().unwrap();
        assert!(std::sync::Arc::ptr_eq(&state.doc, &previous));
        assert_eq!(state.history.undo.len(), undo_len);
        assert_eq!(state.history.redo.len(), redo_len);
        assert_eq!(s.journal.len(), journal_len);
        assert_eq!(s.ui_requests.len(), requests_len);

        // The redo from before the panic must still be usable.
        s.execute("edit.redo", &json!({})).unwrap();
        assert_eq!(s.doc().unwrap().doc.slides.len(), 2);
    }

    #[test]
    fn a_panicking_command_is_an_error_and_keeps_the_document() {
        let mut s = Session::new();
        s.execute("file.new", &json!({})).unwrap();
        s.execute("shape.insert", &json!({"preset": "rect", "rect": [36, 36, 200, 200]})).unwrap();
        let before = s.active().unwrap().doc.clone();
        // A command that edits the document and then panics half way.
        let err = s
            .guarded("test.panic", |s| {
                s.execute("shape.insert", &json!({"preset": "ellipse", "rect": [36, 236, 200, 400]}))?;
                panic!("boom");
            })
            .unwrap_err();
        assert!(err.to_string().contains("test.panic") && err.to_string().contains("boom"), "{err}");
        let st = s.active().unwrap();
        assert!(std::sync::Arc::ptr_eq(&st.doc, &before), "the document is the one from before the command");
        // The session keeps working.
        s.execute("shape.insert", &json!({"preset": "ellipse", "rect": [36, 236, 200, 400]})).unwrap();
    }
}
