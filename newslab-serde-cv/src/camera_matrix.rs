use anyhow::ensure;
use noisy_float::prelude::*;
use serde::{Deserialize, Serialize};

/// The camera matrix describes the mapping from 3D world points to 2D
/// image points. The format is audited during ser/deserialization.
///
/// ```rust
/// # use newslab_serde_cv::CameraMatrix;
/// let json = "[
///     [1.0, 0.0, 4.0],
///     [0.0, 1.5, 7.0],
///     [0.0, 0.0, 1.0]
/// ]";
/// let coefs: CameraMatrix = serde_json::from_str(json).unwrap();
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(try_from = "CameraMatrixUnchecked", into = "CameraMatrixUnchecked")]
pub struct CameraMatrix(pub [[R64; 3]; 3]);

impl Default for CameraMatrix {
    fn default() -> Self {
        Self::identity()
    }
}

impl CameraMatrix {
    pub fn identity() -> Self {
        CameraMatrix([
            [r64(1.0), r64(0.0), r64(0.0)],
            [r64(0.0), r64(1.0), r64(0.0)],
            [r64(0.0), r64(0.0), r64(1.0)],
        ])
    }

    pub fn fx(&self) -> R64 {
        self.0[0][0]
    }

    pub fn fy(&self) -> R64 {
        self.0[1][1]
    }

    pub fn cx(&self) -> R64 {
        self.0[0][2]
    }

    pub fn cy(&self) -> R64 {
        self.0[1][2]
    }
}

#[cfg(feature = "with-nalgebra")]
impl From<&CameraMatrix> for nalgebra::Matrix3<f64> {
    fn from(from: &CameraMatrix) -> Self {
        use slice_of_array::prelude::*;
        let values: Vec<_> = from.0.flat().iter().map(|val| val.raw()).collect();
        Self::from_row_slice(&values)
    }
}

#[cfg(feature = "with-opencv")]
impl From<&CameraMatrix> for opencv::core::Mat {
    fn from(from: &CameraMatrix) -> Self {
        use opencv::prelude::*;
        use slice_of_array::prelude::*;

        Self::from_exact_iter(from.0.flat().iter().map(|val| val.raw()))
            .unwrap()
            .reshape(1, 3)
            .unwrap()
    }
}

#[cfg(feature = "with-opencv")]
impl From<CameraMatrix> for opencv::core::Mat {
    fn from(from: CameraMatrix) -> Self {
        (&from).into()
    }
}

impl From<CameraMatrix> for CameraMatrixUnchecked {
    fn from(from: CameraMatrix) -> Self {
        Self(from.0)
    }
}

impl TryFrom<CameraMatrixUnchecked> for CameraMatrix {
    type Error = anyhow::Error;

    fn try_from(from: CameraMatrixUnchecked) -> Result<Self, Self::Error> {
        let mat = from.0;
        ensure!(
            mat[0][1] == 0.0
                && mat[1][0] == 0.0
                && mat[2][0] == 0.0
                && mat[2][1] == 0.0
                && mat[2][2] == 1.0
        );
        Ok(Self(mat))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
struct CameraMatrixUnchecked(pub [[R64; 3]; 3]);
