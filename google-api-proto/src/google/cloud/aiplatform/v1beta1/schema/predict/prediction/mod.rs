// This file is @generated by prost-build.
/// Prediction output format for Time Series Forecasting.
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct TimeSeriesForecastingPredictionResult {
    /// The regression value.
    #[prost(float, tag = "1")]
    pub value: f32,
}
/// Prediction output format for Video Action Recognition.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoActionRecognitionPredictionResult {
    /// The resource ID of the AnnotationSpec that had been identified.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// The display name of the AnnotationSpec that had been identified.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// The beginning, inclusive, of the video's time segment in which the
    /// AnnotationSpec has been identified. Expressed as a number of seconds as
    /// measured from the start of the video, with fractions up to a microsecond
    /// precision, and with "s" appended at the end.
    #[prost(message, optional, tag = "4")]
    pub time_segment_start: ::core::option::Option<::prost_types::Duration>,
    /// The end, exclusive, of the video's time segment in which the
    /// AnnotationSpec has been identified. Expressed as a number of seconds as
    /// measured from the start of the video, with fractions up to a microsecond
    /// precision, and with "s" appended at the end.
    #[prost(message, optional, tag = "5")]
    pub time_segment_end: ::core::option::Option<::prost_types::Duration>,
    /// The Model's confidence in correction of this prediction, higher
    /// value means higher confidence.
    #[prost(message, optional, tag = "6")]
    pub confidence: ::core::option::Option<f32>,
}
/// Prediction output format for Text Extraction.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TextExtractionPredictionResult {
    /// The resource IDs of the AnnotationSpecs that had been identified,
    /// ordered by the confidence score descendingly.
    #[prost(int64, repeated, tag = "1")]
    pub ids: ::prost::alloc::vec::Vec<i64>,
    /// The display names of the AnnotationSpecs that had been identified,
    /// order matches the IDs.
    #[prost(string, repeated, tag = "2")]
    pub display_names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The start offsets, inclusive, of the text segment in which the
    /// AnnotationSpec has been identified. Expressed as a zero-based number
    /// of characters as measured from the start of the text snippet.
    #[prost(int64, repeated, tag = "3")]
    pub text_segment_start_offsets: ::prost::alloc::vec::Vec<i64>,
    /// The end offsets, inclusive, of the text segment in which the
    /// AnnotationSpec has been identified. Expressed as a zero-based number
    /// of characters as measured from the start of the text snippet.
    #[prost(int64, repeated, tag = "4")]
    pub text_segment_end_offsets: ::prost::alloc::vec::Vec<i64>,
    /// The Model's confidences in correctness of the predicted IDs, higher
    /// value means higher confidence. Order matches the Ids.
    #[prost(float, repeated, tag = "5")]
    pub confidences: ::prost::alloc::vec::Vec<f32>,
}
/// Prediction output format for Image Object Detection.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImageObjectDetectionPredictionResult {
    /// The resource IDs of the AnnotationSpecs that had been identified, ordered
    /// by the confidence score descendingly.
    #[prost(int64, repeated, tag = "1")]
    pub ids: ::prost::alloc::vec::Vec<i64>,
    /// The display names of the AnnotationSpecs that had been identified, order
    /// matches the IDs.
    #[prost(string, repeated, tag = "2")]
    pub display_names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The Model's confidences in correctness of the predicted IDs, higher value
    /// means higher confidence. Order matches the Ids.
    #[prost(float, repeated, tag = "3")]
    pub confidences: ::prost::alloc::vec::Vec<f32>,
    /// Bounding boxes, i.e. the rectangles over the image, that pinpoint
    /// the found AnnotationSpecs. Given in order that matches the IDs. Each
    /// bounding box is an array of 4 numbers `xMin`, `xMax`, `yMin`, and
    /// `yMax`, which represent the extremal coordinates of the box. They are
    /// relative to the image size, and the point 0,0 is in the top left
    /// of the image.
    #[prost(message, repeated, tag = "4")]
    pub bboxes: ::prost::alloc::vec::Vec<::prost_types::ListValue>,
}
/// Prediction output format for Video Classification.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoClassificationPredictionResult {
    /// The resource ID of the AnnotationSpec that had been identified.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// The display name of the AnnotationSpec that had been identified.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// The type of the prediction. The requested types can be configured
    /// via parameters. This will be one of
    /// - segment-classification
    /// - shot-classification
    /// - one-sec-interval-classification
    #[prost(string, tag = "3")]
    pub r#type: ::prost::alloc::string::String,
    /// The beginning, inclusive, of the video's time segment in which the
    /// AnnotationSpec has been identified. Expressed as a number of seconds as
    /// measured from the start of the video, with fractions up to a microsecond
    /// precision, and with "s" appended at the end. Note that for
    /// 'segment-classification' prediction type, this equals the original
    /// 'timeSegmentStart' from the input instance, for other types it is the
    /// start of a shot or a 1 second interval respectively.
    #[prost(message, optional, tag = "4")]
    pub time_segment_start: ::core::option::Option<::prost_types::Duration>,
    /// The end, exclusive, of the video's time segment in which the
    /// AnnotationSpec has been identified. Expressed as a number of seconds as
    /// measured from the start of the video, with fractions up to a microsecond
    /// precision, and with "s" appended at the end. Note that for
    /// 'segment-classification' prediction type, this equals the original
    /// 'timeSegmentEnd' from the input instance, for other types it is the end
    /// of a shot or a 1 second interval respectively.
    #[prost(message, optional, tag = "5")]
    pub time_segment_end: ::core::option::Option<::prost_types::Duration>,
    /// The Model's confidence in correction of this prediction, higher
    /// value means higher confidence.
    #[prost(message, optional, tag = "6")]
    pub confidence: ::core::option::Option<f32>,
}
/// Prediction output format for Image Segmentation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImageSegmentationPredictionResult {
    /// A PNG image where each pixel in the mask represents the category in which
    /// the pixel in the original image was predicted to belong to. The size of
    /// this image will be the same as the original image. The mapping between the
    /// AnntoationSpec and the color can be found in model's metadata. The model
    /// will choose the most likely category and if none of the categories reach
    /// the confidence threshold, the pixel will be marked as background.
    #[prost(string, tag = "1")]
    pub category_mask: ::prost::alloc::string::String,
    /// A one channel image which is encoded as an 8bit lossless PNG. The size of
    /// the image will be the same as the original image. For a specific pixel,
    /// darker color means less confidence in correctness of the cateogry in the
    /// categoryMask for the corresponding pixel. Black means no confidence and
    /// white means complete confidence.
    #[prost(string, tag = "2")]
    pub confidence_mask: ::prost::alloc::string::String,
}
/// Prediction output format for Tabular Classification.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TabularClassificationPredictionResult {
    /// The name of the classes being classified, contains all possible values of
    /// the target column.
    #[prost(string, repeated, tag = "1")]
    pub classes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The model's confidence in each class being correct, higher
    /// value means higher confidence. The N-th score corresponds to
    /// the N-th class in classes.
    #[prost(float, repeated, tag = "2")]
    pub scores: ::prost::alloc::vec::Vec<f32>,
}
/// Prediction output format for Tabular Regression.
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct TabularRegressionPredictionResult {
    /// The regression value.
    #[prost(float, tag = "1")]
    pub value: f32,
    /// The lower bound of the prediction interval.
    #[prost(float, tag = "2")]
    pub lower_bound: f32,
    /// The upper bound of the prediction interval.
    #[prost(float, tag = "3")]
    pub upper_bound: f32,
}
/// Prediction output format for Text Sentiment
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct TextSentimentPredictionResult {
    /// The integer sentiment labels between 0 (inclusive) and sentimentMax label
    /// (inclusive), while 0 maps to the least positive sentiment and
    /// sentimentMax maps to the most positive one. The higher the score is, the
    /// more positive the sentiment in the text snippet is. Note: sentimentMax is
    /// an integer value between 1 (inclusive) and 10 (inclusive).
    #[prost(int32, tag = "1")]
    pub sentiment: i32,
}
/// Prediction output format for Image and Text Classification.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClassificationPredictionResult {
    /// The resource IDs of the AnnotationSpecs that had been identified.
    #[prost(int64, repeated, tag = "1")]
    pub ids: ::prost::alloc::vec::Vec<i64>,
    /// The display names of the AnnotationSpecs that had been identified, order
    /// matches the IDs.
    #[prost(string, repeated, tag = "2")]
    pub display_names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The Model's confidences in correctness of the predicted IDs, higher value
    /// means higher confidence. Order matches the Ids.
    #[prost(float, repeated, tag = "3")]
    pub confidences: ::prost::alloc::vec::Vec<f32>,
}
/// Prediction output format for Video Object Tracking.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoObjectTrackingPredictionResult {
    /// The resource ID of the AnnotationSpec that had been identified.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// The display name of the AnnotationSpec that had been identified.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// The beginning, inclusive, of the video's time segment in which the
    /// object instance has been detected. Expressed as a number of seconds as
    /// measured from the start of the video, with fractions up to a microsecond
    /// precision, and with "s" appended at the end.
    #[prost(message, optional, tag = "3")]
    pub time_segment_start: ::core::option::Option<::prost_types::Duration>,
    /// The end, inclusive, of the video's time segment in which the
    /// object instance has been detected. Expressed as a number of seconds as
    /// measured from the start of the video, with fractions up to a microsecond
    /// precision, and with "s" appended at the end.
    #[prost(message, optional, tag = "4")]
    pub time_segment_end: ::core::option::Option<::prost_types::Duration>,
    /// The Model's confidence in correction of this prediction, higher
    /// value means higher confidence.
    #[prost(message, optional, tag = "5")]
    pub confidence: ::core::option::Option<f32>,
    /// All of the frames of the video in which a single object instance has been
    /// detected. The bounding boxes in the frames identify the same object.
    #[prost(message, repeated, tag = "6")]
    pub frames: ::prost::alloc::vec::Vec<video_object_tracking_prediction_result::Frame>,
}
/// Nested message and enum types in `VideoObjectTrackingPredictionResult`.
pub mod video_object_tracking_prediction_result {
    /// The fields `xMin`, `xMax`, `yMin`, and `yMax` refer to a bounding box,
    /// i.e. the rectangle over the video frame pinpointing the found
    /// AnnotationSpec. The coordinates are relative to the frame size, and the
    /// point 0,0 is in the top left of the frame.
    #[derive(Clone, Copy, PartialEq, ::prost::Message)]
    pub struct Frame {
        /// A time (frame) of a video in which the object has been detected.
        /// Expressed as a number of seconds as measured from the
        /// start of the video, with fractions up to a microsecond precision, and
        /// with "s" appended at the end.
        #[prost(message, optional, tag = "1")]
        pub time_offset: ::core::option::Option<::prost_types::Duration>,
        /// The leftmost coordinate of the bounding box.
        #[prost(message, optional, tag = "2")]
        pub x_min: ::core::option::Option<f32>,
        /// The rightmost coordinate of the bounding box.
        #[prost(message, optional, tag = "3")]
        pub x_max: ::core::option::Option<f32>,
        /// The topmost coordinate of the bounding box.
        #[prost(message, optional, tag = "4")]
        pub y_min: ::core::option::Option<f32>,
        /// The bottommost coordinate of the bounding box.
        #[prost(message, optional, tag = "5")]
        pub y_max: ::core::option::Option<f32>,
    }
}
