use crate::features::settings::model::Language;

pub fn t<'a>(lang: &Language, key: &'a str) -> &'a str {
    match (lang, key) {
        // Tray menu
        (Language::Vi, "tray.skip") => "Bỏ qua nghỉ",
        (Language::Vi, "tray.pause") => "Tạm dừng",
        (Language::Vi, "tray.resume") => "Tiếp tục",
        (Language::Vi, "tray.settings") => "Cài đặt",
        (Language::Vi, "tray.quit") => "Thoát",
        (Language::Vi, "tray.tooltip") => "Sipping — Uống nước đi, giữ sức khỏe",
        (Language::Vi, "tooltip.mini") => "Nghỉ ngắn",
        (Language::Vi, "tooltip.long") => "Nghỉ dài",
        (Language::Vi, "break.default") => "Uống nước đi! Giữ đủ nước nhé.",

        // English defaults
        (_, "tray.skip") => "Skip Break",
        (_, "tray.pause") => "Pause Timer",
        (_, "tray.resume") => "Resume Timer",
        (_, "tray.settings") => "Settings",
        (_, "tray.quit") => "Quit",
        (_, "tray.tooltip") => "Sipping — Keep sipping, stay hydrated",
        (_, "tooltip.mini") => "Mini",
        (_, "tooltip.long") => "Long",
        (_, "break.default") => "Have a sip! Stay hydrated.",

        _ => key,
    }
}
