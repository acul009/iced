/// Provides information about a monitor
/// Can be requested using [`iced::window::list_monitors`]
pub struct MonitorInfo {
    /// The size of the monitor in physical pixels.
    physical_size: crate::Size<u32>,
    /// The position of the monitor in physical pixels.
    physical_position: crate::Point<i32>,
    /// The scale factor of the monitor.
    scale_factor: f64,
    name: Option<String>,
    refresh_rate_millihertz: Option<u32>,
}

impl MonitorInfo {
    /// Creates a new [`MonitorInfo`] with the given physical size, position, and scale factor.
    pub fn new(
        physical_size: crate::Size<u32>,
        physical_position: crate::Point<i32>,
        scale_factor: f64,
        name: Option<String>,
        refresh_rate_millihertz: Option<u32>,
    ) -> Self {
        Self {
            physical_size,
            physical_position,
            scale_factor,
            name,
            refresh_rate_millihertz,
        }
    }

    /// Returns the size of the monitor in logical pixels.
    ///
    /// **WARNING**: this ignores the scale factor you might have set for a window.
    pub fn size(&self) -> crate::Size {
        let width = self.physical_size.width as f32 * self.scale_factor as f32;
        let height =
            self.physical_size.height as f32 * self.scale_factor as f32;
        crate::Size::new(width, height)
    }

    /// Returns the scale factor of the monitor.
    pub fn scale_factor(&self) -> f64 {
        self.scale_factor
    }

    /// Returns the name of the monitor.
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Returns the refresh rate of the monitor in millihertz.
    pub fn refresh_rate_millihertz(&self) -> Option<u32> {
        self.refresh_rate_millihertz
    }
}
