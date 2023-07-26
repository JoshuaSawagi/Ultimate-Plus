mod aerials;
mod ground;
mod smashes;
mod specials;
mod throws;
mod tilts;

pub fn install() {
    aerials::install();
    ground::install();
    smashes::install();
    specials::install();
    throws::install();
    tilts::install();
}