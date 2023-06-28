mod eagle;
mod scale_nx;

pub(crate) use eagle::eagle;
pub(crate) use scale_nx::scale_2x;
pub(crate) use scale_nx::scale_3x;

pub enum Algorithm {
    Scale2X,
    Scale3X,
    Eagle,
}
