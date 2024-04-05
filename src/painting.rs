use crate::css::Color;
use crate::layout::Rect;

type DisplayList = Vec<DisplayCommand>;

enum DisplayCommand {
    SolidColor(Color, Rect),
}
