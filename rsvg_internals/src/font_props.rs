//! CSS font properties.

use cssparser::{BasicParseError, Parser};

use crate::drawing_ctx::ViewParams;
use crate::error::*;
use crate::length::*;
use crate::parsers::ParseToParseError;
use crate::properties::ComputedValues;

// https://www.w3.org/TR/2008/REC-CSS2-20080411/fonts.html#propdef-font-size
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum FontSizeSpec {
    Smaller,
    Larger,
    XXSmall,
    XSmall,
    Small,
    Medium,
    Large,
    XLarge,
    XXLarge,
    Value(Length<Both>),
}

impl FontSizeSpec {
    pub fn value(&self) -> Length<Both> {
        match self {
            FontSizeSpec::Value(s) => *s,
            _ => unreachable!(),
        }
    }

    pub fn compute(&self, v: &ComputedValues) -> Self {
        let compute_points = |p| 12.0 * 1.2f64.powf(p) / POINTS_PER_INCH;

        let size = v.font_size.0.value();

        let new_size = match self {
            FontSizeSpec::Smaller => Length::<Both>::new(size.length / 1.2, size.unit),
            FontSizeSpec::Larger => Length::<Both>::new(size.length * 1.2, size.unit),
            FontSizeSpec::XXSmall => Length::<Both>::new(compute_points(-3.0), LengthUnit::In),
            FontSizeSpec::XSmall => Length::<Both>::new(compute_points(-2.0), LengthUnit::In),
            FontSizeSpec::Small => Length::<Both>::new(compute_points(-1.0), LengthUnit::In),
            FontSizeSpec::Medium => Length::<Both>::new(compute_points(0.0), LengthUnit::In),
            FontSizeSpec::Large => Length::<Both>::new(compute_points(1.0), LengthUnit::In),
            FontSizeSpec::XLarge => Length::<Both>::new(compute_points(2.0), LengthUnit::In),
            FontSizeSpec::XXLarge => Length::<Both>::new(compute_points(3.0), LengthUnit::In),
            FontSizeSpec::Value(s) if s.unit == LengthUnit::Percent => {
                Length::<Both>::new(size.length * s.length, size.unit)
            }
            FontSizeSpec::Value(s) => *s,
        };

        FontSizeSpec::Value(new_size)
    }

    pub fn normalize(&self, values: &ComputedValues, params: &ViewParams) -> f64 {
        self.value().normalize(values, params)
    }
}

impl ParseToParseError for FontSizeSpec {
    fn parse_to_parse_error<'i>(
        parser: &mut Parser<'i, '_>,
    ) -> Result<FontSizeSpec, CssParseError<'i>> {
        parser
            .try_parse(|p| Length::<Both>::parse_to_parse_error(p))
            .and_then(|l| Ok(FontSizeSpec::Value(l)))
            .or_else(|_| {
                Ok(parse_identifiers!(
                    parser,
                    "smaller" => FontSizeSpec::Smaller,
                    "larger" => FontSizeSpec::Larger,
                    "xx-small" => FontSizeSpec::XXSmall,
                    "x-small" => FontSizeSpec::XSmall,
                    "small" => FontSizeSpec::Small,
                    "medium" => FontSizeSpec::Medium,
                    "large" => FontSizeSpec::Large,
                    "x-large" => FontSizeSpec::XLarge,
                    "xx-large" => FontSizeSpec::XXLarge,
                )?)
            })
    }
}

// https://www.w3.org/TR/2008/REC-CSS2-20080411/fonts.html#propdef-font-weight
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum FontWeightSpec {
    Normal,
    Bold,
    Bolder,
    Lighter,
    W100,
    W200,
    W300,
    W400,
    W500,
    W600,
    W700,
    W800,
    W900,
}

impl ParseToParseError for FontWeightSpec {
    fn parse_to_parse_error<'i>(parser: &mut Parser<'i, '_>) -> Result<FontWeightSpec, CssParseError<'i>> {
        parser
            .try_parse(|p| {
                Ok(parse_identifiers!(
                    p,
                    "normal" => FontWeightSpec::Normal,
                    "bold" => FontWeightSpec::Bold,
                    "bolder" => FontWeightSpec::Bolder,
                    "lighter" => FontWeightSpec::Lighter,
                )?)
            })
            .or_else(|_: CssParseError| {
                let loc = parser.current_source_location();
                parser
                    .expect_integer()
                    .map_err(|e: BasicParseError| e.into())
                    .and_then(|i| match i {
                        100 => Ok(FontWeightSpec::W100),
                        200 => Ok(FontWeightSpec::W200),
                        300 => Ok(FontWeightSpec::W300),
                        400 => Ok(FontWeightSpec::W400),
                        500 => Ok(FontWeightSpec::W500),
                        600 => Ok(FontWeightSpec::W600),
                        700 => Ok(FontWeightSpec::W700),
                        800 => Ok(FontWeightSpec::W800),
                        900 => Ok(FontWeightSpec::W900),
                        _ => Err(loc.new_custom_error(ValueErrorKind::parse_error("parse error")))
                    })
            })
    }
}

// https://www.w3.org/TR/css-text-3/#letter-spacing-property
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum LetterSpacingSpec {
    Normal,
    Value(Length<Horizontal>),
}

impl LetterSpacingSpec {
    pub fn value(&self) -> Length<Horizontal> {
        match self {
            LetterSpacingSpec::Value(s) => *s,
            _ => unreachable!(),
        }
    }

    pub fn compute(&self) -> Self {
        let spacing = match self {
            LetterSpacingSpec::Normal => Length::<Horizontal>::new(0.0, LengthUnit::Px),
            LetterSpacingSpec::Value(s) => *s,
        };

        LetterSpacingSpec::Value(spacing)
    }

    pub fn normalize(&self, values: &ComputedValues, params: &ViewParams) -> f64 {
        self.value().normalize(values, params)
    }
}

impl ParseToParseError for LetterSpacingSpec {
    fn parse_to_parse_error<'i>(
        parser: &mut Parser<'i, '_>,
    ) -> Result<LetterSpacingSpec, CssParseError<'i>> {
        parser
            .try_parse(|p| Length::<Horizontal>::parse_to_parse_error(p))
            .and_then(|l| Ok(LetterSpacingSpec::Value(l)))
            .or_else(|_| {
                Ok(parse_identifiers!(
                    parser,
                    "normal" => LetterSpacingSpec::Normal,
                )?)
            })
    }
}

/// https://www.w3.org/TR/2008/REC-CSS2-20080411/fonts.html#propdef-font-family
#[derive(Debug, Clone, PartialEq)]
pub struct SingleFontFamily(pub String);

impl ParseToParseError for SingleFontFamily {
    fn parse_to_parse_error<'i>(
        parser: &mut Parser<'i, '_>,
    ) -> Result<SingleFontFamily, CssParseError<'i>> {
        let loc = parser.current_source_location();

        if let Ok(cow) = parser.try_parse(|p| p.expect_string_cloned()) {
            if cow == "" {
                return Err(loc.new_custom_error(ValueErrorKind::value_error(
                    "empty string is not a valid font family name",
                )));
            }

            return Ok(SingleFontFamily((*cow).to_owned()));
        }

        let first_ident = parser.expect_ident()?.clone();

        let mut value = first_ident.as_ref().to_owned();

        while let Ok(cow) = parser.try_parse(|p| p.expect_ident_cloned()) {
            value.push(' ');
            value.push_str(&cow);
        }

        Ok(SingleFontFamily(value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_invalid_invalid_font_size() {
        assert!(FontSizeSpec::parse_str_to_parse_error("furlong").is_err());
    }

    #[test]
    fn parses_font_weight() {
        assert_eq!(
            <FontWeightSpec as ParseToParseError>::parse_str_to_parse_error("normal"),
            Ok(FontWeightSpec::Normal)
        );
        assert_eq!(
            <FontWeightSpec as ParseToParseError>::parse_str_to_parse_error("bold"),
            Ok(FontWeightSpec::Bold)
        );
        assert_eq!(
            <FontWeightSpec as ParseToParseError>::parse_str_to_parse_error("100"),
            Ok(FontWeightSpec::W100)
        );
    }

    #[test]
    fn detects_invalid_font_weight() {
        assert!(<FontWeightSpec as ParseToParseError>::parse_str_to_parse_error("").is_err());
        assert!(<FontWeightSpec as ParseToParseError>::parse_str_to_parse_error("strange").is_err());
        assert!(<FontWeightSpec as ParseToParseError>::parse_str_to_parse_error("314").is_err());
        assert!(<FontWeightSpec as ParseToParseError>::parse_str_to_parse_error("3.14").is_err());
    }

    #[test]
    fn parses_letter_spacing() {
        assert_eq!(
            <LetterSpacingSpec as ParseToParseError>::parse_str_to_parse_error("normal"),
            Ok(LetterSpacingSpec::Normal)
        );
        assert_eq!(
            <LetterSpacingSpec as ParseToParseError>::parse_str_to_parse_error("10em"),
            Ok(LetterSpacingSpec::Value(Length::<Horizontal>::new(
                10.0,
                LengthUnit::Em,
            )))
        );
    }

    #[test]
    fn computes_letter_spacing() {
        assert_eq!(
            <LetterSpacingSpec as ParseToParseError>::parse_str_to_parse_error("normal")
                .map(|s| s.compute()),
            Ok(LetterSpacingSpec::Value(Length::<Horizontal>::new(
                0.0,
                LengthUnit::Px,
            )))
        );
        assert_eq!(
            <LetterSpacingSpec as ParseToParseError>::parse_str_to_parse_error("10em")
                .map(|s| s.compute()),
            Ok(LetterSpacingSpec::Value(Length::<Horizontal>::new(
                10.0,
                LengthUnit::Em,
            )))
        );
    }

    #[test]
    fn detects_invalid_invalid_letter_spacing() {
        assert!(LetterSpacingSpec::parse_str_to_parse_error("furlong").is_err());
    }

    #[test]
    fn parses_font_family() {
        assert_eq!(
            <SingleFontFamily as ParseToParseError>::parse_str_to_parse_error("'Hello world'"),
            Ok(SingleFontFamily("Hello world".to_owned()))
        );

        assert_eq!(
            <SingleFontFamily as ParseToParseError>::parse_str_to_parse_error("\"Hello world\""),
            Ok(SingleFontFamily("Hello world".to_owned()))
        );

        assert_eq!(
            <SingleFontFamily as ParseToParseError>::parse_str_to_parse_error("  Hello  world  "),
            Ok(SingleFontFamily("Hello world".to_owned()))
        );

        assert_eq!(
            <SingleFontFamily as ParseToParseError>::parse_str_to_parse_error("Plonk"),
            Ok(SingleFontFamily("Plonk".to_owned()))
        );
    }

    #[test]
    fn detects_invalid_font_family() {
        assert!(<SingleFontFamily as ParseToParseError>::parse_str_to_parse_error("").is_err());
        assert!(<SingleFontFamily as ParseToParseError>::parse_str_to_parse_error("''").is_err());
        assert!(<SingleFontFamily as ParseToParseError>::parse_str_to_parse_error("42").is_err());
    }
}
