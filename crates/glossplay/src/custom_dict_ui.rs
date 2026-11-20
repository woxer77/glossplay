//! The Custom Dictionary settings UI: connection probe and status.

use crate::i18n::{Tr, t, tf};
use crate::settings_ui::Form;
use crate::theme;
use eframe::egui::{self, Id, RichText};
use gp_ai::custom_dict::CustomDictClient;
use gp_core::config::Config;
use std::sync::{Arc, Mutex};

#[derive(Clone, Debug, PartialEq)]
enum Probe {
    Idle,
    Checking,
    Connected(u16),
    Error(String),
}

#[derive(Clone)]
struct Shared {
    probe: Arc<Mutex<Probe>>,
}

fn start_probe(ctx: &egui::Context, shared: &Shared, url: &str, token: &str) {
    let url = url.trim().to_owned();
    if url.is_empty() {
        *shared.probe.lock().unwrap() = Probe::Idle;
        return;
    }
    *shared.probe.lock().unwrap() = Probe::Checking;
    let slot = shared.probe.clone();
    let ctx = ctx.clone();
    let token = if token.trim().is_empty() {
        None
    } else {
        Some(token.trim().to_owned())
    };
    std::thread::spawn(move || {
        let client = CustomDictClient::new(&url, token.as_deref());
        let result = match client.probe() {
            Ok(code) => Probe::Connected(code),
            Err(e) => Probe::Error(e.to_string()),
        };
        *slot.lock().unwrap() = result;
        ctx.request_repaint();
    });
}

pub fn section(ui: &mut egui::Ui, form: &Form, draft: &Config) {
    let id = Id::unique("custom-dict-connection");
    let shared = ui.data_mut(|d| {
        d.get_temp_mut_or_insert_with(id, || Shared {
            probe: Arc::new(Mutex::new(Probe::Idle)),
        })
        .clone()
    });

    let probe = shared.probe.lock().unwrap().clone();
    let checking = probe == Probe::Checking;
    let configured = !draft.custom_dict.url.trim().is_empty();

    form.row(ui, t(Tr::CustomDictStatusRow), None, |ui| {
        if ui
            .add_enabled(
                configured && !checking,
                egui::Button::new(t(Tr::CustomDictCheckButton)),
            )
            .clicked()
        {
            start_probe(
                ui.ctx(),
                &shared,
                &draft.custom_dict.url,
                &draft.custom_dict.token,
            );
        }
    });

    if configured {
        let (text, color) = match &probe {
            Probe::Idle => (String::new(), theme::muted()),
            Probe::Checking => (t(Tr::CustomDictChecking).to_owned(), theme::muted()),
            Probe::Connected(code) => (tf(Tr::CustomDictConnected, &[code]), theme::accent()),
            Probe::Error(e) => (tf(Tr::CustomDictStatusError, &[e]), theme::danger()),
        };
        if !text.is_empty() {
            ui.add(egui::Label::new(RichText::new(text).color(color)).wrap());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_url_stays_idle() {
        let shared = Shared {
            probe: Arc::new(Mutex::new(Probe::Idle)),
        };
        let ctx = egui::Context::default();
        start_probe(&ctx, &shared, "", "");
        assert_eq!(*shared.probe.lock().unwrap(), Probe::Idle);
    }

    #[test]
    fn probe_state_transitions() {
        let p = Probe::Connected(200);
        assert_eq!(p, Probe::Connected(200));
        assert_ne!(p, Probe::Checking);
    }
}
