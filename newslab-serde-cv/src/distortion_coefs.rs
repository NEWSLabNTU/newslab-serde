use noisy_float::prelude::*;
use serde::{Deserialize, Serialize};

/// The camera distortion coefficients in `[k1, k2, p1, p2, k3]` format.
///
/// ```rust
/// # use newslab_serde_cv::DistortionCoefs;
/// let json = "[0.0, 1.0, 0.4, 0.0, 0.0]";
/// let coefs: DistortionCoefs = serde_json::from_str(json).unwrap();
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DistortionCoefs(pub [R64; 5]);

impl DistortionCoefs {
    pub fn zeros() -> Self {
        DistortionCoefs([r64(0.0), r64(0.0), r64(0.0), r64(0.0), r64(0.0)])
    }

    pub fn k1(&self) -> R64 {
        self.0[0]
    }

    pub fn k2(&self) -> R64 {
        self.0[1]
    }

    pub fn p1(&self) -> R64 {
        self.0[2]
    }

    pub fn p2(&self) -> R64 {
        self.0[3]
    }

    pub fn k3(&self) -> R64 {
        self.0[4]
    }
}

impl Default for DistortionCoefs {
    fn default() -> Self {
        Self::zeros()
    }
}

#[cfg(feature = "with-nalgebra")]
#[allow(unused)]
pub use with_nalgebra::*;

#[cfg(feature = "with-nalgebra")]
mod with_nalgebra {
    use super::*;

    // impl From<&DistortionCoefs> for opencv_ros_camera::Distortion<f64> {
    //     fn from(from: &DistortionCoefs) -> Self {
    //         let coefs: [f64; 5] = unsafe { mem::transmute(from.0) };
    //         let slice: &[f64] = coefs.as_ref();
    //         opencv_ros_camera::Distortion::from_opencv_vec(na::Vector5::from_column_slice(
    //             slice,
    //         ))
    //     }
    // }

    // impl From<DistortionCoefs> for opencv_ros_camera::Distortion<f64> {
    //     fn from(from: DistortionCoefs) -> Self {
    //         (&from).into()
    //     }
    // }

    impl From<&DistortionCoefs> for nalgebra::Vector5<f64> {
        fn from(from: &DistortionCoefs) -> Self {
            nalgebra::Vector5::from_iterator(from.0.iter().map(|val| val.raw()))
        }
    }
}

#[cfg(feature = "with-opencv")]
impl From<&DistortionCoefs> for opencv::core::Mat {
    fn from(from: &DistortionCoefs) -> Self {
        opencv::core::Mat::from_exact_iter(from.0.iter().map(|val| val.raw())).unwrap()
    }
}

#[cfg(feature = "with-opencv")]
impl From<DistortionCoefs> for opencv::core::Mat {
    fn from(from: DistortionCoefs) -> Self {
        (&from).into()
    }
}
