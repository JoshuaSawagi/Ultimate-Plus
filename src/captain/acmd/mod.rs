mod aerials;
mod ground;
mod tilts;
mod smashes;
pub fn install() {
    aerials::install();
    ground::install();
    tilts::install();
    smashes::install();
}