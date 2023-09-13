use crate::{CameraMatrix, DistortionCoefs};
use serde::{Deserialize, Serialize};

/// Represent intrinsic parameters for a camera.
///
/// ```rust
/// # use newslab_serde_cv::CameraIntrinsicParams;
/// let json = r#"{
///     "camera_matrix": [[1.0, 0.0, 4.0],
///                       [0.0, 1.5, 7.0],
///                       [0.0, 0.0, 1.0]],
///     "distortion_coefs": [1.0, 0.0, 0.0, 0.5, 0.0]
/// }"#;
/// let params: CameraIntrinsicParams = serde_json::from_str(json).unwrap();
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CameraIntrinsicParams {
    pub camera_matrix: CameraMatrix,
    pub distortion_coefs: DistortionCoefs,
}

impl CameraIntrinsicParams {
    pub fn identity() -> Self {
        Self {
            camera_matrix: CameraMatrix::identity(),
            distortion_coefs: DistortionCoefs::zeros(),
        }
    }
}

impl Default for CameraIntrinsicParams {
    fn default() -> Self {
        Self::identity()
    }
}

// #[cfg(feature = "with-nalgebra")]
// impl From<&CameraIntrinsic> for opencv_ros_camera::RosOpenCvIntrinsics<f64> {
//     fn from(from: &CameraIntrinsic) -> Self {
//         let CameraIntrinsic {
//             camera_matrix,
//             distortion_coefs,
//         } = from;

//         opencv_ros_camera::RosOpenCvIntrinsics::from_params_with_distortion(
//             camera_matrix.fx().raw(),
//             0.0, // skew
//             camera_matrix.fy().raw(),
//             camera_matrix.cx().raw(),
//             camera_matrix.cy().raw(),
//             distortion_coefs.into(),
//         )
//     }
// }

// #[cfg(feature = "with-nalgebra")]
// impl From<CameraIntrinsic> for opencv_ros_camera::RosOpenCvIntrinsics<f64> {
//     fn from(from: CameraIntrinsic) -> Self {
//         (&from).into()
//     }
// }
