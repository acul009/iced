use std::fmt::Debug;

/// Provides you with information about all available monitors.
#[derive(Debug, Clone)]
pub struct MonitorList {
    monitors: Vec<MonitorData>,
    primary: Option<MonitorIndex>,
}

impl MonitorList {
    /// Iterates over all monitors
    pub fn iter(&self) -> impl Iterator<Item = MonitorInfo<'_>> {
        (0..self.monitors.len()).map(|index| MonitorInfo {
            list: self,
            index: MonitorIndex(index),
        })
    }

    /// Returns the primary monitor, if available
    pub fn primary(&self) -> Option<MonitorInfo<'_>> {
        self.get(self.primary?)
    }

    /// Returns the primary monitor, or the first monitor if no primary is available
    pub fn primary_or_first(&self) -> MonitorInfo<'_> {
        self.primary().unwrap_or_else(|| MonitorInfo {
            list: self,
            index: MonitorIndex(0),
        })
    }

    /// Returns the monitor at the given index, if available
    pub fn get(&self, index: MonitorIndex) -> Option<MonitorInfo<'_>> {
        if index.0 >= self.monitors.len() {
            None
        } else {
            Some(MonitorInfo { list: self, index })
        }
    }

    /// Creates a new empty monitor list
    pub fn empty() -> Self {
        Self {
            monitors: Vec::new(),
            primary: None,
        }
    }

    /// Adds a new monitor to the list
    pub fn add_monitor(
        &mut self,
        is_primary: bool,
        physical_size: crate::Size<u32>,
        physical_position: crate::Point<i32>,
        scale_factor: f64,
        name: Option<String>,
        refresh_rate_millihertz: Option<u32>,
    ) {
        if is_primary {
            self.primary = Some(MonitorIndex(self.monitors.len()));
        }

        self.monitors.push(MonitorData::new(
            physical_size,
            physical_position,
            scale_factor,
            name,
            refresh_rate_millihertz,
        ));
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Represents the index of a monitor in a monitor list.
pub struct MonitorIndex(pub usize);

/// Allows you to access all important information about a monitor.
pub struct MonitorInfo<'a> {
    list: &'a MonitorList,
    index: MonitorIndex,
}

impl<'a> MonitorInfo<'a> {
    fn monitor(&self) -> &MonitorData {
        &self.list.monitors[self.index.0]
    }

    /// Returns the size of the monitor in logical pixels by applying the scale factor of the monitor to it's physical size.
    ///
    /// **WARNING**: this ignores the scale factor you might have set for a **window**.
    pub fn size(&self) -> crate::Size {
        let monitor = self.monitor();
        let width =
            monitor.physical_size.width as f32 * monitor.scale_factor as f32;
        let height =
            monitor.physical_size.height as f32 * monitor.scale_factor as f32;
        crate::Size::new(width, height)
    }

    /// Returns the size of the monitor in physical pixels. In most cases you should use `size` instead.
    pub fn physical_size(&self) -> crate::Size<u32> {
        self.monitor().physical_size
    }

    /// Returns the position of the monitor in physical pixels. This is usually not needed.
    ///
    /// If you want to position your windows on this monitor,
    /// you can instead use the `index()` method and hand that to the window on creation.
    pub fn physical_position(&self) -> crate::Point<i32> {
        self.monitor().physical_position
    }

    /// Returns the index of the monitor. You can use this when creating a new window.
    pub fn index(&self) -> MonitorIndex {
        self.index.clone()
    }

    /// Returns the scale factor of the monitor.
    pub fn scale_factor(&self) -> f64 {
        self.monitor().scale_factor
    }

    /// Returns the name of the monitor.
    pub fn name(&self) -> Option<&str> {
        self.monitor().name.as_deref()
    }

    /// Returns the refresh rate of the monitor in millihertz.
    pub fn refresh_rate_millihertz(&self) -> Option<u32> {
        self.monitor().refresh_rate_millihertz
    }
}

#[derive(Debug, Clone)]
struct MonitorData {
    /// The size of the monitor in physical pixels.
    physical_size: crate::Size<u32>,
    /// The position of the monitor in physical pixels.
    physical_position: crate::Point<i32>,
    /// The scale factor of the monitor.
    scale_factor: f64,
    name: Option<String>,
    refresh_rate_millihertz: Option<u32>,
}

impl MonitorData {
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
}
