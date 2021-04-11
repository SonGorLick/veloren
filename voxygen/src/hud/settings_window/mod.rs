mod controls;
mod gameplay;
mod interface;
mod language;
mod sound;
mod video;

use crate::{
    hud::{
        img_ids::Imgs, BarNumbers, BuffPosition, CrosshairType, PressBehavior, ShortcutNumbers,
        Show, TEXT_COLOR, UI_HIGHLIGHT_0, UI_MAIN,
    },
    i18n::{LanguageMetadata, Localization},
    render::RenderMode,
    settings::Fps,
    ui::fonts::Fonts,
    window::{FullScreenSettings, GameInput},
    GlobalState,
};
use conrod_core::{
    color,
    widget::{self, Button, Image, Rectangle, Text},
    widget_ids, Colorable, Labelable, Positionable, Sizeable, Widget, WidgetCommon,
};

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

widget_ids! {
    struct Ids {
        frame,
        settings_bg,
        tabs_align,
        icon,
        settings_close,
        settings_title,
        settings_content_align,

        tabs[],
        interface,
        gameplay,
        controls,
        video,
        sound,
        language,
    }
}

const RESET_BUTTONS_HEIGHT: f64 = 34.0;
const RESET_BUTTONS_WIDTH: f64 = 155.0;

#[derive(Debug, EnumIter, PartialEq)]
pub enum SettingsTab {
    Interface,
    Video,
    Sound,
    Gameplay,
    Controls,
    Lang,
}
impl SettingsTab {
    fn name_key(&self) -> &str {
        match self {
            SettingsTab::Interface => "common.interface",
            SettingsTab::Gameplay => "common.gameplay",
            SettingsTab::Controls => "common.controls",
            SettingsTab::Video => "common.video",
            SettingsTab::Sound => "common.sound",
            SettingsTab::Lang => "common.languages",
        }
    }

    fn title_key(&self) -> &str {
        match self {
            SettingsTab::Interface => "common.interface_settings",
            SettingsTab::Gameplay => "common.gameplay_settings",
            SettingsTab::Controls => "common.controls_settings",
            SettingsTab::Video => "common.video_settings",
            SettingsTab::Sound => "common.sound_settings",
            SettingsTab::Lang => "common.language_settings",
        }
    }
}

#[derive(WidgetCommon)]
pub struct SettingsWindow<'a> {
    global_state: &'a GlobalState,
    show: &'a Show,
    imgs: &'a Imgs,
    fonts: &'a Fonts,
    localized_strings: &'a Localization,
    fps: f32,
    #[conrod(common_builder)]
    common: widget::CommonBuilder,
}

impl<'a> SettingsWindow<'a> {
    pub fn new(
        global_state: &'a GlobalState,
        show: &'a Show,
        imgs: &'a Imgs,
        fonts: &'a Fonts,
        localized_strings: &'a Localization,
        fps: f32,
    ) -> Self {
        Self {
            global_state,
            show,
            imgs,
            fonts,
            localized_strings,
            fps,
            common: widget::CommonBuilder::default(),
        }
    }
}

pub struct State {
    ids: Ids,
}

pub enum Event {
    ToggleHelp,
    ToggleDebug,
    ToggleTips(bool),
    ToggleBarNumbers(BarNumbers),
    ToggleShortcutNumbers(ShortcutNumbers),
    BuffPosition(BuffPosition),
    ChangeTab(SettingsTab),
    Close,
    AdjustMousePan(u32),
    AdjustMouseZoom(u32),
    AdjustCameraClamp(u32),
    ToggleZoomInvert(bool),
    ToggleMouseYInvert(bool),
    ToggleControllerYInvert(bool),
    ToggleSmoothPan(bool),
    AdjustViewDistance(u32),
    AdjustSpriteRenderDistance(u32),
    AdjustFigureLoDRenderDistance(u32),
    AdjustFOV(u16),
    AdjustLodDetail(u32),
    AdjustGamma(f32),
    AdjustExposure(f32),
    AdjustAmbiance(f32),
    AdjustWindowSize([u16; 2]),
    ChangeFullscreenMode(FullScreenSettings),
    ToggleParticlesEnabled(bool),
    ChangeRenderMode(Box<RenderMode>),
    AdjustMusicVolume(f32),
    AdjustSfxVolume(f32),
    //ChangeAudioDevice(String),
    MaximumFPS(Fps),
    CrosshairTransp(f32),
    CrosshairType(CrosshairType),
    UiScale(ScaleChange),
    ChatTransp(f32),
    ChatCharName(bool),
    Sct(bool),
    SctPlayerBatch(bool),
    SctDamageBatch(bool),
    SpeechBubbleDarkMode(bool),
    SpeechBubbleIcon(bool),
    ChangeLanguage(Box<LanguageMetadata>),
    ChangeBinding(GameInput),
    ResetInterfaceSettings,
    ResetGameplaySettings,
    ResetKeyBindings,
    ResetGraphicsSettings,
    ResetAudioSettings,
    ChangeFreeLookBehavior(PressBehavior),
    ChangeAutoWalkBehavior(PressBehavior),
    ChangeCameraClampBehavior(PressBehavior),
    ChangeStopAutoWalkOnInput(bool),
    ChangeAutoCamera(bool),
}

#[derive(Clone)]
pub enum ScaleChange {
    ToAbsolute,
    ToRelative,
    Adjust(f64),
}

impl<'a> Widget for SettingsWindow<'a> {
    type Event = Vec<Event>;
    type State = State;
    type Style = ();

    fn init_state(&self, id_gen: widget::id::Generator) -> Self::State {
        State {
            ids: Ids::new(id_gen),
        }
    }

    #[allow(clippy::unused_unit)] // TODO: Pending review in #587
    fn style(&self) -> Self::Style { () }

    fn update(self, args: widget::UpdateArgs<Self>) -> Self::Event {
        let widget::UpdateArgs { state, ui, .. } = args;

        let mut events = Vec::new();
        let tab_font_scale = 18;

        // Frame
        Image::new(self.imgs.settings_bg)
            .w_h(1052.0, 886.0)
            .mid_top_with_margin_on(ui.window, 5.0)
            .color(Some(UI_MAIN))
            .set(state.ids.settings_bg, ui);

        Image::new(self.imgs.settings_frame)
            .w_h(1052.0, 886.0)
            .middle_of(state.ids.settings_bg)
            .color(Some(UI_HIGHLIGHT_0))
            .set(state.ids.frame, ui);

        // Content Alignment
        Rectangle::fill_with([814.0, 834.0], color::TRANSPARENT)
            .top_right_with_margins_on(state.ids.frame, 46.0, 2.0)
            .set(state.ids.settings_content_align, ui);

        // Tabs Content Alignment
        Rectangle::fill_with([232.0, 814.0], color::TRANSPARENT)
            .top_left_with_margins_on(state.ids.frame, 44.0, 2.0)
            .scroll_kids()
            .scroll_kids_vertically()
            .set(state.ids.tabs_align, ui);

        // Icon
        Image::new(self.imgs.settings)
            .w_h(29.0 * 1.5, 25.0 * 1.5)
            .top_left_with_margins_on(state.ids.frame, 2.0, 1.0)
            .set(state.ids.icon, ui);
        // Title
        Text::new(
            self.localized_strings
                .get(self.show.settings_tab.title_key()),
        )
        .mid_top_with_margin_on(state.ids.frame, 3.0)
        .font_id(self.fonts.cyri.conrod_id)
        .font_size(self.fonts.cyri.scale(29))
        .color(TEXT_COLOR)
        .set(state.ids.settings_title, ui);

        // X-Button
        if Button::image(self.imgs.close_button)
            .w_h(24.0, 25.0)
            .hover_image(self.imgs.close_btn_hover)
            .press_image(self.imgs.close_btn_press)
            .top_right_with_margins_on(state.ids.frame, 0.0, 0.0)
            .set(state.ids.settings_close, ui)
            .was_clicked()
        {
            events.push(Event::Close);
        }

        // Tabs
        if state.ids.tabs.len() < SettingsTab::iter().len() {
            state.update(|s| {
                s.ids
                    .tabs
                    .resize(SettingsTab::iter().len(), &mut ui.widget_id_generator())
            });
        }
        for (i, settings_tab) in SettingsTab::iter().enumerate() {
            let mut button = Button::image(if self.show.settings_tab == settings_tab {
                self.imgs.selection
            } else {
                self.imgs.nothing
            })
            .w_h(230.0, 48.0)
            .hover_image(self.imgs.selection_hover)
            .press_image(self.imgs.selection_press)
            .label(self.localized_strings.get(settings_tab.name_key()))
            .label_font_size(self.fonts.cyri.scale(tab_font_scale))
            .label_font_id(self.fonts.cyri.conrod_id)
            .label_color(TEXT_COLOR);

            button = if i == 0 {
                button.mid_top_with_margin_on(state.ids.tabs_align, 28.0)
            } else {
                button.down_from(state.ids.tabs[i - 1], 0.0)
            };

            if button.set(state.ids.tabs[i], ui).was_clicked() {
                events.push(Event::ChangeTab(settings_tab));
            }
        }

        // Content Area
        macro_rules! setup_content_area {
            ($parent:expr, $ui:expr, $widget:expr, $id:expr) => {
                $widget
                    .top_left_with_margins_on($parent, 0.0, 0.0)
                    .wh_of($parent)
                    .set($id, $ui)
            };
        }
        let global_state = self.global_state;
        let show = self.show;
        let imgs = self.imgs;
        let fonts = self.fonts;
        let localized_strings = self.localized_strings;
        for event in match self.show.settings_tab {
            SettingsTab::Interface => setup_content_area!(
                state.ids.settings_content_align,
                ui,
                interface::Interface::new(global_state, show, imgs, fonts, localized_strings,),
                state.ids.interface
            ),
            SettingsTab::Gameplay => setup_content_area!(
                state.ids.settings_content_align,
                ui,
                gameplay::Gameplay::new(global_state, imgs, fonts, localized_strings,),
                state.ids.gameplay
            ),
            SettingsTab::Controls => setup_content_area!(
                state.ids.settings_content_align,
                ui,
                controls::Controls::new(global_state, imgs, fonts, localized_strings,),
                state.ids.controls
            ),
            SettingsTab::Video => setup_content_area!(
                state.ids.settings_content_align,
                ui,
                video::Video::new(global_state, imgs, fonts, localized_strings, self.fps,),
                state.ids.video
            ),
            SettingsTab::Sound => setup_content_area!(
                state.ids.settings_content_align,
                ui,
                sound::Sound::new(global_state, imgs, fonts, localized_strings,),
                state.ids.sound
            ),
            SettingsTab::Lang => setup_content_area!(
                state.ids.settings_content_align,
                ui,
                language::Language::new(global_state, imgs, fonts),
                state.ids.language
            ),
        } {
            events.push(event);
        }

        events
    }
}
