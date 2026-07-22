pub struct SystemSettings {
    pub hostname: String,
    pub timezone: String,
    pub locale: String,
    pub kernel: String,
    pub boot_animation: bool,
    pub auto_update: bool,
    pub power_profile: PowerProfile,
}

pub enum PowerProfile {
    Performance,
    Balanced,
    PowerSave,
}
