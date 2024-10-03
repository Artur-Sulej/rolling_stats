use rustler::NifStruct;

#[derive(NifStruct)]
#[module = "RollingStats.StatsData"]
pub struct StatsData {
    pub(crate) average: f64,
    pub(crate) variance: f64,
    pub(crate) last: f64,
    pub(crate) min: f64,
    pub(crate) max: f64,
}
