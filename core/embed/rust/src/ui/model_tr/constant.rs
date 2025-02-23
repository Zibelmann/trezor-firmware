use crate::ui::geometry::{Offset, Point, Rect};

pub const WIDTH: i16 = 128;
pub const HEIGHT: i16 = 64;
pub const LINE_SPACE: i16 = 1;
pub const FONT_BPP: i16 = 1;

pub const LOADER_OUTER: i16 = 32;
pub const LOADER_INNER: i16 = 18;
pub const LOADER_ICON_MAX_SIZE: i16 = 8;

pub const MODEL_NAME: &str = "Trezor Model R";

pub const fn size() -> Offset {
    Offset::new(WIDTH, HEIGHT)
}
pub const SIZE: Offset = size();

pub const fn screen() -> Rect {
    Rect::from_top_left_and_size(Point::zero(), SIZE)
}
pub const SCREEN: Rect = screen();
