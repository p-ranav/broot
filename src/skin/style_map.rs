/// Defines the StyleMap structure with its default value.
///
/// A style_map is a collection of termimad compound_style. It's
/// either defined for the focused panel state or the unfocused
/// one (there are thus two instances in the application)
use {
    super::*,
    crossterm::style::{
        Attribute::*,
        Attributes,
        Color::*,
    },
    std::{
        collections::HashMap,
        fmt,
    },
    termimad::CompoundStyle,
};

// this macro, which must be called once, creates
// the StyleMap struct with its creation functions handling
// both default values defined in the macro call and
// overriding values defined in TOML
macro_rules! StyleMap {
    (
        $(
            $name:ident: $fg:expr, $bg:expr, [$($attr:expr)*] $( / $fgu:expr, $bgu:expr , [$($attru:expr)*] )*
        )*
    ) => {
        /// a struct whose fields are the styles to apply to various parts/cases
        pub struct StyleMap {
            $(pub $name: CompoundStyle,)*
        }
        /// a set of two style_maps: one for the focused panel and one for the other panels
        ///
        /// This struct is just a vessel for the skin initialization process.
        pub struct StyleMaps {
            pub focused: StyleMap,
            pub unfocused: StyleMap,
        }
        impl StyleMap {
            /// build a skin without any terminal control character (for file output)
            pub fn no_term() -> Self {
                Self {
                    $($name: CompoundStyle::default(),)*
                }
            }
            /// ensures the "default" skin entry is used as base for all other
            /// entries (this processus is part of the skin initialization)
            fn diffuse_default(&mut self) {
                $(
                    let mut base = self.default.clone();
                    base.overwrite_with(&self.$name);
                    self.$name = base;
                )*
            }
        }
        impl StyleMaps {
            pub fn create(skin_conf: & HashMap<String, SkinEntry>) -> Self {
                let mut focused = StyleMap {
                    $($name: skin_conf
                        .get(stringify!($name))
                        .map(|sec| sec.get_focused().clone())
                        .unwrap_or(
                            CompoundStyle::new(
                                $fg,
                                $bg,
                                Attributes::from(vec![$($attr),*].as_slice()),
                            )
                        )
                    ,)*
                };
                focused.diffuse_default();
                let mut unfocused = StyleMap {
                    $($name: CompoundStyle::default(),)*
                };
                $(
                    unfocused.$name = CompoundStyle::new(
                        $fg,
                        $bg,
                        Attributes::from(vec![$($attr),*].as_slice()),
                    );
                    $(
                        unfocused.$name = CompoundStyle::new(
                            $fgu,
                            $bgu,
                            Attributes::from(vec![$($attru),*].as_slice()),
                        );
                    )*
                    if let Some(sec) = skin_conf.get(stringify!($name)) {
                        unfocused.$name = sec.get_unfocused().clone();
                    }
                )*
                unfocused.diffuse_default();
                Self {
                    focused,
                    unfocused,
                }
            }
        }
        impl Clone for StyleMap {
            fn clone(&self) -> Self {
                Self {
                    $($name: self.$name.clone(),)*
                }
            }
        }
    }
}

// Default styles defined as
//    name: forecolor, backcolor, [attributes]
// The optional part after a '/' is the style for unfocused panels
// (if missing the style is the same than for focused panels)
StyleMap! {
    default: gray(22), gray(1), [] / gray(20), gray(1), []
    tree: gray(5), None, [] / gray(3), None, []
    file: gray(18), None, [] / gray(15), None, []
    directory: ansi(110), None, [Bold] / ansi(110), None, []
    exe: Some(Cyan), None, []
    link: Some(Magenta), None, []
    pruning: gray(12), None, [Italic]
    perm__: gray(5), None, []
    perm_r: ansi(94), None, []
    perm_w: ansi(132), None, []
    perm_x: ansi(65), None, []
    owner: ansi(138), None, []
    group: ansi(131), None, []
    dates: ansi(66), None, []
    sparse: ansi(214), None, []
    git_branch: ansi(178), None, []
    git_insertions: ansi(28), None, []
    git_deletions: ansi(160), None, []
    git_status_current: gray(5), None, []
    git_status_modified: ansi(28), None, []
    git_status_new: ansi(94), None, [Bold]
    git_status_ignored: gray(17), None, []
    git_status_conflicted: ansi(88), None, []
    git_status_other: ansi(88), None, []
    selected_line: None, gray(4), [] / None, gray(2), []
    char_match: Some(Green), None, []
    file_error: Some(Red), None, []
    flag_label: gray(15), None, []
    flag_value: ansi(178), None, [Bold]
    input: Some(White), None, [] / gray(15), None, []
    status_error: gray(22), ansi(124), []
    status_job: ansi(220), gray(5), []
    status_normal: gray(20), gray(3), [] / gray(2), gray(2), []
    status_italic: ansi(178), gray(3), [] / gray(2), gray(2), []
    status_bold: ansi(178), gray(3), [Bold] / gray(2), gray(2), []
    status_code: ansi(229), gray(3), [] / gray(2), gray(2), []
    status_ellipsis: gray(19), gray(1), [] / gray(2), gray(2), []
    purpose_normal: gray(20), gray(2), []
    purpose_italic: ansi(178), gray(2), []
    purpose_bold: ansi(178), gray(2), [Bold]
    scrollbar_track: gray(7), None, [] / gray(4), None, []
    scrollbar_thumb: gray(22), None, [] / gray(14), None, []
    help_paragraph: gray(20), None, []
    help_bold: ansi(178), None, [Bold]
    help_italic: ansi(229), None, []
    help_code: gray(21), gray(3), []
    help_headers: ansi(178), None, []
    help_table_border: ansi(239), None, []
}

impl fmt::Debug for StyleMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Skin")
    }
}
