use egui::Color32;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Theme {
    pub hyperlink: Color32,
    pub error_fg: Color32,
    pub warn_fg: Color32,
    pub bg_fill: Color32,
    pub text: Color32,
    pub stroke: Color32,
    pub hovered: Color32,
    pub active: Color32,
    pub open: Color32,
    pub base: Color32,
    pub code_bg: Color32,
    pub extreme_bg: Color32,
}

//Note: For now this is using Catppuccin Mocha colours as a placeholder.
pub const ROSEMARY_DARK: Theme = Theme {
    hyperlink: Color32::from_rgb(245, 224, 220),
    error_fg: Color32::from_rgb(235, 160, 172),
    text: Color32::from_rgb(205, 214, 244),
    base: Color32::from_rgb(30, 30, 46),
    warn_fg: Color32::from_rgb(250, 179, 135),
    bg_fill: Color32::from_rgb(137, 180, 250),
    stroke: Color32::from_rgb(127, 132, 156),
    hovered: Color32::from_rgb(88, 91, 112),
    active: Color32::from_rgb(69, 71, 90),
    open: Color32::from_rgb(49, 50, 68),
    code_bg: Color32::from_rgb(24, 24, 37),
    extreme_bg: Color32::from_rgb(17, 17, 27),
};

use egui::{epaint, style};

pub fn set_theme(ctx: &egui::Context, theme: Theme) {
    let old = ctx.style().visuals.clone();
    ctx.set_visuals(theme.visuals(old));
}

pub fn set_style_theme(style: &mut egui::Style, theme: Theme) {
    let old = style.visuals.clone();
    style.visuals = theme.visuals(old);
}

fn make_widget_visual(
    old: style::WidgetVisuals,
    theme: &Theme,
    bg_fill: egui::Color32,
) -> style::WidgetVisuals {
    style::WidgetVisuals {
        bg_fill,
        weak_bg_fill: bg_fill,
        bg_stroke: egui::Stroke {
            color: theme.stroke,
            ..old.bg_stroke
        },
        fg_stroke: egui::Stroke {
            color: theme.text,
            ..old.fg_stroke
        },
        ..old
    }
}

impl Theme {
    fn visuals(&self, old: egui::Visuals) -> egui::Visuals {
        egui::Visuals {
            override_text_color: Some(self.text),
            hyperlink_color: self.hyperlink,
            faint_bg_color: self.open,
            extreme_bg_color: self.extreme_bg,
            code_bg_color: self.code_bg,
            warn_fg_color: self.warn_fg,
            error_fg_color: self.error_fg,
            window_fill: self.base,
            panel_fill: self.base,
            window_stroke: egui::Stroke {
                color: self.stroke,
                ..old.window_stroke
            },
            widgets: style::Widgets {
                noninteractive: make_widget_visual(old.widgets.noninteractive, self, self.base),
                inactive: make_widget_visual(old.widgets.inactive, self, self.open),
                hovered: make_widget_visual(old.widgets.hovered, self, self.hovered),
                active: make_widget_visual(old.widgets.active, self, self.active),
                open: make_widget_visual(old.widgets.open, self, self.open),
            },
            selection: style::Selection {
                bg_fill: self.bg_fill.linear_multiply(0.2),
                stroke: egui::Stroke {
                    color: self.stroke,
                    ..old.selection.stroke
                },
            },

            window_shadow: epaint::Shadow {
                color: egui::Color32::from_black_alpha(96),
                ..old.window_shadow
            },
            popup_shadow: epaint::Shadow {
                color: egui::Color32::from_black_alpha(96),
                ..old.popup_shadow
            },
            dark_mode: true,
            ..old
        }
    }
}
