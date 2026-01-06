use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::mem;
use windows::core::PCWSTR;
use windows::Win32::Foundation::HWND;
use windows::Win32::Graphics::Gdi::{
    ChangeDisplaySettingsExW, EnumDisplayDevicesW, EnumDisplaySettingsW, CDS_GLOBAL, CDS_UPDATEREGISTRY,
    DEVMODEW, DISPLAY_DEVICEW, DISP_CHANGE_SUCCESSFUL, DM_BITSPERPEL, DM_DISPLAYFREQUENCY,
    DM_PELSWIDTH, DM_PELSHEIGHT, ENUM_CURRENT_SETTINGS, ENUM_DISPLAY_SETTINGS_MODE,
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Resolution {
    pub width: u32,
    pub height: u32,
    pub frequency: u32,
    pub bits_per_pixel: u32,
}

impl std::fmt::Display for Resolution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}x{} @ {}Hz ({}bit)",
            self.width, self.height, self.frequency, self.bits_per_pixel
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Monitor {
    pub id: String,
    pub name: String,
    pub device_name: String,
    pub current_resolution: Resolution,
    pub position: (i32, i32),
    #[serde(skip)]
    pub available_resolutions: Vec<Resolution>,
}

pub struct DisplayManager;

impl DisplayManager {
    pub fn enumerate_monitors() -> Result<Vec<Monitor>> {
        let mut monitors = Vec::new();
        let mut dev_num = 0;

        loop {
            let mut display_device = DISPLAY_DEVICEW {
                cb: mem::size_of::<DISPLAY_DEVICEW>() as u32,
                ..Default::default()
            };

            let result = unsafe { EnumDisplayDevicesW(None, dev_num, &mut display_device, 0) };

            if !result.as_bool() {
                break;
            }

            // Check if attached to desktop
            if (display_device.StateFlags & windows::Win32::Graphics::Gdi::DISPLAY_DEVICE_ATTACHED_TO_DESKTOP) != 0 {
                let device_name_os = display_device.DeviceName;
                let device_name_str = String::from_utf16_lossy(&device_name_os)
                    .trim_matches(char::from(0))
                    .to_string();

                // Get current settings
                let mut dev_mode = DEVMODEW {
                    dmSize: mem::size_of::<DEVMODEW>() as u16,
                    ..Default::default()
                };
                unsafe {
                    let _ = EnumDisplaySettingsW(
                        PCWSTR::from_raw(display_device.DeviceName.as_ptr()),
                        ENUM_CURRENT_SETTINGS,
                        &mut dev_mode,
                    );
                };

                let current_res = Resolution {
                    width: dev_mode.dmPelsWidth,
                    height: dev_mode.dmPelsHeight,
                    frequency: dev_mode.dmDisplayFrequency,
                    bits_per_pixel: dev_mode.dmBitsPerPel,
                };

                let position = unsafe {
                    (
                        dev_mode.Anonymous1.Anonymous2.dmPosition.x,
                        dev_mode.Anonymous1.Anonymous2.dmPosition.y,
                    )
                };

                // Get available resolutions
                let mut resolutions = Vec::new();
                let mut mode_num = 0;
                loop {
                    let mut mode = DEVMODEW {
                        dmSize: mem::size_of::<DEVMODEW>() as u16,
                        ..Default::default()
                    };
                    let success = unsafe {
                        EnumDisplaySettingsW(
                            PCWSTR::from_raw(display_device.DeviceName.as_ptr()),
                            ENUM_DISPLAY_SETTINGS_MODE(mode_num),
                            &mut mode,
                        )
                    };

                    if !success.as_bool() {
                        break;
                    }

                    let res = Resolution {
                        width: mode.dmPelsWidth,
                        height: mode.dmPelsHeight,
                        frequency: mode.dmDisplayFrequency,
                        bits_per_pixel: mode.dmBitsPerPel,
                    };
                    
                    // Basic de-duplication
                    if !resolutions.contains(&res) {
                         resolutions.push(res);
                    }
                   
                    mode_num += 1;
                }

                // Sorting
                resolutions.sort_by(|a, b| {
                    b.width.cmp(&a.width)
                        .then(b.height.cmp(&a.height))
                        .then(b.frequency.cmp(&a.frequency))
                });

                monitors.push(Monitor {
                    id: device_name_str.clone(), // Using device name as ID for simplicity
                    name: format!("Display {}", dev_num + 1), // Simple naming
                    device_name: device_name_str,
                    current_resolution: current_res,
                    position,
                    available_resolutions: resolutions,
                });
            }

            dev_num += 1;
        }

        Ok(monitors)
    }

    pub fn set_resolution(device_name: &str, resolution: &Resolution) -> Result<()> {
        let mut dev_mode = DEVMODEW {
            dmSize: mem::size_of::<DEVMODEW>() as u16,
            ..Default::default()
        };
        
        let device_name_w: Vec<u16> = device_name.encode_utf16().chain(Some(0)).collect();
        let device_name_pcwstr = PCWSTR::from_raw(device_name_w.as_ptr());

        // Get current settings first to fill in other fields
        unsafe {
             let _ = EnumDisplaySettingsW(
                device_name_pcwstr,
                ENUM_CURRENT_SETTINGS,
                &mut dev_mode,
            );
        }

        dev_mode.dmPelsWidth = resolution.width;
        dev_mode.dmPelsHeight = resolution.height;
        dev_mode.dmDisplayFrequency = resolution.frequency;
        dev_mode.dmBitsPerPel = resolution.bits_per_pixel;
        
        dev_mode.dmFields = DM_PELSWIDTH | DM_PELSHEIGHT | DM_DISPLAYFREQUENCY | DM_BITSPERPEL;

        let result = unsafe {
            ChangeDisplaySettingsExW(
                device_name_pcwstr,
                Some(&dev_mode),
                HWND(std::ptr::null_mut()),
                CDS_UPDATEREGISTRY | CDS_GLOBAL,
                None,
            )
        };

        if result == DISP_CHANGE_SUCCESSFUL {
            Ok(())
        } else {
            Err(anyhow!("Failed to change display settings. Error code: {:?}", result))
        }
    }
}
