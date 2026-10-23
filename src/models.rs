use serde::{Deserialize, Serialize};

/// What to read.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RecognizeTextRequest {
    /// A file path or a `content://` URI.
    pub path: String,
}

/// The text found in an image. Positions are in pixels of the image upright,
/// with its rotation tag applied.
#[derive(Debug, Clone, Default, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RecognizedText {
    /// All the text, blocks separated by a blank line.
    pub text: String,
    /// The image as the engine saw it, after its rotation is applied.
    pub image_width: u32,
    pub image_height: u32,
    pub blocks: Vec<TextBlock>,
}

/// A paragraph or a column of text, as the engine groups it.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TextBlock {
    pub text: String,
    /// Where the block is, when the engine can tell.
    pub bounds: Option<Bounds>,
    pub lines: Vec<TextLine>,
}

/// One line of a block.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TextLine {
    pub text: String,
    /// Where the line is, when the engine can tell.
    pub bounds: Option<Bounds>,
    /// From 0 to 1, when the engine gives it.
    pub confidence: Option<f32>,
    /// Degrees, when the engine gives it.
    pub angle: Option<f32>,
}

/// A rectangle in pixels of the image. It can start a little outside the
/// image when the text touches its edge.
#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Bounds {
    pub left: i32,
    pub top: i32,
    pub width: u32,
    pub height: u32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn result_uses_the_names_of_the_typescript_api() {
        let bounds = Bounds {
            left: 1,
            top: 2,
            width: 3,
            height: 4,
        };
        let result = RecognizedText {
            text: "ISBN 978-84-376-0494-7".into(),
            image_width: 640,
            image_height: 480,
            blocks: vec![TextBlock {
                text: "ISBN 978-84-376-0494-7".into(),
                bounds: Some(bounds),
                lines: vec![TextLine {
                    text: "ISBN 978-84-376-0494-7".into(),
                    bounds: Some(bounds),
                    confidence: Some(0.5),
                    angle: None,
                }],
            }],
        };

        let bounds = json!({ "left": 1, "top": 2, "width": 3, "height": 4 });
        assert_eq!(
            serde_json::to_value(&result).unwrap(),
            json!({
                "text": "ISBN 978-84-376-0494-7",
                "imageWidth": 640,
                "imageHeight": 480,
                "blocks": [{
                    "text": "ISBN 978-84-376-0494-7",
                    "bounds": bounds,
                    "lines": [{
                        "text": "ISBN 978-84-376-0494-7",
                        "bounds": bounds,
                        "confidence": 0.5,
                        "angle": null
                    }]
                }]
            })
        );
    }

    #[test]
    fn missing_bounds_confidence_and_angle_are_read_as_none() {
        let line: TextLine = serde_json::from_value(json!({ "text": "" })).unwrap();

        assert_eq!(line.bounds, None);
        assert_eq!(line.confidence, None);
        assert_eq!(line.angle, None);
    }

    #[test]
    fn bounds_can_start_outside_the_image() {
        let bounds: Bounds =
            serde_json::from_value(json!({ "left": -2, "top": -1, "width": 10, "height": 5 }))
                .unwrap();

        assert_eq!((bounds.left, bounds.top), (-2, -1));
    }

    #[test]
    fn request_needs_a_path() {
        assert!(serde_json::from_value::<RecognizeTextRequest>(json!({})).is_err());
    }
}
