use crate::base::bxdf::Bxdf;
use crate::util::vecmath::Frame;

#[derive(Default)]
pub struct Bsdf {
    #[allow(dead_code)]
    bxdf: Bxdf,
    #[allow(dead_code)]
    shading_frame: Frame,
}