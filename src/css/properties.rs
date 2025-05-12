use super::values::Value;

#[derive(Debug, PartialEq, Clone)]
pub enum Property {
    Width(Value),
    Height(Value),
    MinWidth(Value),
    MinHeight(Value),
    MaxWidth(Value),
    MaxHeight(Value),
    Margin(Value),
    MarginTop(Value),
    MarginRight(Value),
    MarginBottom(Value),
    MarginLeft(Value),
    Padding(Value),
    PaddingTop(Value),
    PaddingRight(Value),
    PaddingBottom(Value),
    PaddingLeft(Value),
    Border(Value),
    BorderTop(Value),
    BorderRight(Value),
    BorderBottom(Value),
    BorderLeft(Value),
    BorderWidth(Value),
    BorderStyle(Value),
    BorderColor(Value),
    BorderRadius(Value),
    BoxSizing(Value),
    Display(Value),

    // Positioning
    Position(Value),
    Top(Value),
    Right(Value),
    Bottom(Value),
    Left(Value),
    ZIndex(Value),
    Float(Value),
    Clear(Value),

    // Typography
    Color(Value),
    FontFamily(Value),
    FontSize(Value),
    FontStyle(Value),
    FontWeight(Value),
    LineHeight(Value),
    TextAlign(Value),
    TextDecoration(Value),
    TextTransform(Value),
    LetterSpacing(Value),
    WordSpacing(Value),
    WhiteSpace(Value),

    // Visual
    BackgroundColor(Value),
    BackgroundImage(Value),
    BackgroundPosition(Value),
    BackgroundRepeat(Value),
    BackgroundSize(Value),
    Opacity(Value),
    Visibility(Value),

    // Flexbox
    FlexDirection(Value),
    FlexWrap(Value),
    FlexGrow(Value),
    FlexShrink(Value),
    FlexBasis(Value),
    JustifyContent(Value),
    AlignItems(Value),
    AlignSelf(Value),
    AlignContent(Value),

    // Grid
    GridTemplateColumns(Value),
    GridTemplateRows(Value),
    GridColumnGap(Value),
    GridRowGap(Value),
    GridColumn(Value),
    GridRow(Value),

    // Animation
    Transition(Value),
    Animation(Value),
    AnimationName(Value),
    AnimationDuration(Value),
    AnimationTimingFunction(Value),
    AnimationDelay(Value),
    AnimationIterationCount(Value),
    AnimationDirection(Value),

    // Other
    Cursor(Value),
    Overflow(Value),
    Content(Value),
    PointerEvents(Value),
    UserSelect(Value),
}

impl Property {
    pub fn name(&self) -> &'static str {
        match self {
            // Box Model
            Property::Width(_) => "width",
            Property::Height(_) => "height",
            Property::MinWidth(_) => "min-width",
            Property::MinHeight(_) => "min-height",
            Property::MaxWidth(_) => "max-width",
            Property::MaxHeight(_) => "max-height",
            Property::Margin(_) => "margin",
            Property::MarginTop(_) => "margin-top",
            Property::MarginRight(_) => "margin-right",
            Property::MarginBottom(_) => "margin-bottom",
            Property::MarginLeft(_) => "margin-left",
            Property::Padding(_) => "padding",
            Property::PaddingTop(_) => "padding-top",
            Property::PaddingRight(_) => "padding-right",
            Property::PaddingBottom(_) => "padding-bottom",
            Property::PaddingLeft(_) => "padding-left",
            Property::Border(_) => "border",
            Property::BorderTop(_) => "border-top",
            Property::BorderRight(_) => "border-right",
            Property::BorderBottom(_) => "border-bottom",
            Property::BorderLeft(_) => "border-left",
            Property::BorderWidth(_) => "border-width",
            Property::BorderStyle(_) => "border-style",
            Property::BorderColor(_) => "border-color",
            Property::BorderRadius(_) => "border-radius",
            Property::BoxSizing(_) => "box-sizing",
            Property::Display(_) => "display",

            // Positioning
            Property::Position(_) => "position",
            Property::Top(_) => "top",
            Property::Right(_) => "right",
            Property::Bottom(_) => "bottom",
            Property::Left(_) => "left",
            Property::ZIndex(_) => "z-index",
            Property::Float(_) => "float",
            Property::Clear(_) => "clear",

            // Typography
            Property::Color(_) => "color",
            Property::FontFamily(_) => "font-family",
            Property::FontSize(_) => "font-size",
            Property::FontStyle(_) => "font-style",
            Property::FontWeight(_) => "font-weight",
            Property::LineHeight(_) => "line-height",
            Property::TextAlign(_) => "text-align",
            Property::TextDecoration(_) => "text-decoration",
            Property::TextTransform(_) => "text-transform",
            Property::LetterSpacing(_) => "letter-spacing",
            Property::WordSpacing(_) => "word-spacing",
            Property::WhiteSpace(_) => "white-space",

            // Visual
            Property::BackgroundColor(_) => "background-color",
            Property::BackgroundImage(_) => "background-image",
            Property::BackgroundPosition(_) => "background-position",
            Property::BackgroundRepeat(_) => "background-repeat",
            Property::BackgroundSize(_) => "background-size",
            Property::Opacity(_) => "opacity",
            Property::Visibility(_) => "visibility",

            // Flexbox
            Property::FlexDirection(_) => "flex-direction",
            Property::FlexWrap(_) => "flex-wrap",
            Property::FlexGrow(_) => "flex-grow",
            Property::FlexShrink(_) => "flex-shrink",
            Property::FlexBasis(_) => "flex-basis",
            Property::JustifyContent(_) => "justify-content",
            Property::AlignItems(_) => "align-items",
            Property::AlignSelf(_) => "align-self",
            Property::AlignContent(_) => "align-content",

            // Grid
            Property::GridTemplateColumns(_) => "grid-template-columns",
            Property::GridTemplateRows(_) => "grid-template-rows",
            Property::GridColumnGap(_) => "grid-column-gap",
            Property::GridRowGap(_) => "grid-row-gap",
            Property::GridColumn(_) => "grid-column",
            Property::GridRow(_) => "grid-row",

            // Animation
            Property::Transition(_) => "transition",
            Property::Animation(_) => "animation",
            Property::AnimationName(_) => "animation-name",
            Property::AnimationDuration(_) => "animation-duration",
            Property::AnimationTimingFunction(_) => "animation-timing-function",
            Property::AnimationDelay(_) => "animation-delay",
            Property::AnimationIterationCount(_) => "animation-iteration-count",
            Property::AnimationDirection(_) => "animation-direction",

            // Other
            Property::Cursor(_) => "cursor",
            Property::Overflow(_) => "overflow",
            Property::Content(_) => "content",
            Property::PointerEvents(_) => "pointer-events",
            Property::UserSelect(_) => "user-select",
        }
    }

    pub fn parse(name: &str, value: Value) -> Option<Self> {
        match name {
            // Box Model
            "width" => Some(Property::Width(value)),
            "height" => Some(Property::Height(value)),
            "min-width" => Some(Property::MinWidth(value)),
            "min-height" => Some(Property::MinHeight(value)),
            "max-width" => Some(Property::MaxWidth(value)),
            "max-height" => Some(Property::MaxHeight(value)),
            "margin" => Some(Property::Margin(value)),
            "margin-top" => Some(Property::MarginTop(value)),
            "margin-right" => Some(Property::MarginRight(value)),
            "margin-bottom" => Some(Property::MarginBottom(value)),
            "margin-left" => Some(Property::MarginLeft(value)),
            "padding" => Some(Property::Padding(value)),
            "padding-top" => Some(Property::PaddingTop(value)),
            "padding-right" => Some(Property::PaddingRight(value)),
            "padding-bottom" => Some(Property::PaddingBottom(value)),
            "padding-left" => Some(Property::PaddingLeft(value)),
            "border" => Some(Property::Border(value)),
            "border-top" => Some(Property::BorderTop(value)),
            "border-right" => Some(Property::BorderRight(value)),
            "border-bottom" => Some(Property::BorderBottom(value)),
            "border-left" => Some(Property::BorderLeft(value)),
            "border-width" => Some(Property::BorderWidth(value)),
            "border-style" => Some(Property::BorderStyle(value)),
            "border-color" => Some(Property::BorderColor(value)),
            "border-radius" => Some(Property::BorderRadius(value)),
            "box-sizing" => Some(Property::BoxSizing(value)),
            "display" => Some(Property::Display(value)),

            // Positioning
            "position" => Some(Property::Position(value)),
            "top" => Some(Property::Top(value)),
            "right" => Some(Property::Right(value)),
            "bottom" => Some(Property::Bottom(value)),
            "left" => Some(Property::Left(value)),
            "z-index" => Some(Property::ZIndex(value)),
            "float" => Some(Property::Float(value)),
            "clear" => Some(Property::Clear(value)),

            // Typography
            "color" => Some(Property::Color(value)),
            "font-family" => Some(Property::FontFamily(value)),
            "font-size" => Some(Property::FontSize(value)),
            "font-style" => Some(Property::FontStyle(value)),
            "font-weight" => Some(Property::FontWeight(value)),
            "line-height" => Some(Property::LineHeight(value)),
            "text-align" => Some(Property::TextAlign(value)),
            "text-decoration" => Some(Property::TextDecoration(value)),
            "text-transform" => Some(Property::TextTransform(value)),
            "letter-spacing" => Some(Property::LetterSpacing(value)),
            "word-spacing" => Some(Property::WordSpacing(value)),
            "white-space" => Some(Property::WhiteSpace(value)),

            // Visual
            "background-color" => Some(Property::BackgroundColor(value)),
            "background-image" => Some(Property::BackgroundImage(value)),
            "background-position" => Some(Property::BackgroundPosition(value)),
            "background-repeat" => Some(Property::BackgroundRepeat(value)),
            "background-size" => Some(Property::BackgroundSize(value)),
            "opacity" => Some(Property::Opacity(value)),
            "visibility" => Some(Property::Visibility(value)),

            // Flexbox
            "flex-direction" => Some(Property::FlexDirection(value)),
            "flex-wrap" => Some(Property::FlexWrap(value)),
            "flex-grow" => Some(Property::FlexGrow(value)),
            "flex-shrink" => Some(Property::FlexShrink(value)),
            "flex-basis" => Some(Property::FlexBasis(value)),
            "justify-content" => Some(Property::JustifyContent(value)),
            "align-items" => Some(Property::AlignItems(value)),
            "align-self" => Some(Property::AlignSelf(value)),
            "align-content" => Some(Property::AlignContent(value)),

            // Grid
            "grid-template-columns" => Some(Property::GridTemplateColumns(value)),
            "grid-template-rows" => Some(Property::GridTemplateRows(value)),
            "grid-column-gap" => Some(Property::GridColumnGap(value)),
            "grid-row-gap" => Some(Property::GridRowGap(value)),
            "grid-column" => Some(Property::GridColumn(value)),
            "grid-row" => Some(Property::GridRow(value)),

            // Animation
            "transition" => Some(Property::Transition(value)),
            "animation" => Some(Property::Animation(value)),
            "animation-name" => Some(Property::AnimationName(value)),
            "animation-duration" => Some(Property::AnimationDuration(value)),
            "animation-timing-function" => Some(Property::AnimationTimingFunction(value)),
            "animation-delay" => Some(Property::AnimationDelay(value)),
            "animation-iteration-count" => Some(Property::AnimationIterationCount(value)),
            "animation-direction" => Some(Property::AnimationDirection(value)),

            // Other
            "cursor" => Some(Property::Cursor(value)),
            "overflow" => Some(Property::Overflow(value)),
            "content" => Some(Property::Content(value)),
            "pointer-events" => Some(Property::PointerEvents(value)),
            "user-select" => Some(Property::UserSelect(value)),

            _ => None,
        }
    }
}
