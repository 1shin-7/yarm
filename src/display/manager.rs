use anyhow::{anyhow, Result};
use std::collections::HashMap;
use std::mem;
use windows::core::PCWSTR;
use windows::Win32::Devices::Display::{
    DisplayConfigGetDeviceInfo, GetDisplayConfigBufferSizes, QueryDisplayConfig,
    DISPLAYCONFIG_DEVICE_INFO_GET_SOURCE_NAME, DISPLAYCONFIG_DEVICE_INFO_GET_TARGET_NAME,
    DISPLAYCONFIG_MODE_INFO, DISPLAYCONFIG_PATH_INFO, DISPLAYCONFIG_SOURCE_DEVICE_NAME,
    DISPLAYCONFIG_TARGET_DEVICE_NAME, QDC_ONLY_ACTIVE_PATHS,
};
use windows::Win32::Foundation::{ERROR_SUCCESS, HWND};
use windows::Win32::Graphics::Gdi::{
    ChangeDisplaySettingsExW, EnumDisplayDevicesW, EnumDisplaySettingsW, CDS_GLOBAL,
    CDS_UPDATEREGISTRY, DEVMODEW, DISPLAY_DEVICEW, DISP_CHANGE_SUCCESSFUL, DM_BITSPERPEL,
    DM_DISPLAYFREQUENCY, DM_DISPLAYORIENTATION, DM_PELSHEIGHT, DM_PELSWIDTH, ENUM_CURRENT_SETTINGS,
    ENUM_DISPLAY_SETTINGS_MODE,
};

use super::monitor::Monitor;
use super::orientation::Orientation;
use super::resolution::Resolution;

pub struct DisplayManager;

impl DisplayManager {
    // Helper to get a map of GDI Device Name -> Friendly Name using QueryDisplayConfig
    fn get_display_names_map() -> HashMap<String, String> {
        let mut names_map = HashMap::new();
        let mut num_paths = 0;
        let mut num_modes = 0;

        unsafe {
            if GetDisplayConfigBufferSizes(QDC_ONLY_ACTIVE_PATHS, &mut num_paths, &mut num_modes)
                != ERROR_SUCCESS
            {
                return names_map;
            }

            let mut paths = vec![DISPLAYCONFIG_PATH_INFO::default(); num_paths as usize];
            let mut modes = vec![DISPLAYCONFIG_MODE_INFO::default(); num_modes as usize];

            if QueryDisplayConfig(
                QDC_ONLY_ACTIVE_PATHS,
                &mut num_paths,
                paths.as_mut_ptr(),
                &mut num_modes,
                modes.as_mut_ptr(),
                None,
            ) != ERROR_SUCCESS
            {
                return names_map;
            }

            // Resize vector to actual returned count, just in case
            paths.truncate(num_paths as usize);

            for path in paths {
                // 1. Get Source Name (GDI Device Name)
                let mut source_name = DISPLAYCONFIG_SOURCE_DEVICE_NAME::default();
                source_name.header.r#type = DISPLAYCONFIG_DEVICE_INFO_GET_SOURCE_NAME;
                source_name.header.size = mem::size_of::<DISPLAYCONFIG_SOURCE_DEVICE_NAME>() as u32;
                source_name.header.adapterId = path.sourceInfo.adapterId;
                source_name.header.id = path.sourceInfo.id;

                if DisplayConfigGetDeviceInfo(&mut source_name.header) == ERROR_SUCCESS.0 as i32 {
                    let gdi_device_name = String::from_utf16_lossy(&source_name.viewGdiDeviceName)
                        .trim_matches(char::from(0))
                        .to_string();

                    // 2. Get Target Name (Friendly Name)
                    let mut target_name = DISPLAYCONFIG_TARGET_DEVICE_NAME::default();
                    target_name.header.r#type = DISPLAYCONFIG_DEVICE_INFO_GET_TARGET_NAME;
                    target_name.header.size =
                        mem::size_of::<DISPLAYCONFIG_TARGET_DEVICE_NAME>() as u32;
                    target_name.header.adapterId = path.targetInfo.adapterId;
                    target_name.header.id = path.targetInfo.id;

                    if DisplayConfigGetDeviceInfo(&mut target_name.header) == ERROR_SUCCESS.0 as i32
                    {
                        let friendly_name =
                            String::from_utf16_lossy(&target_name.monitorFriendlyDeviceName)
                                .trim_matches(char::from(0))
                                .to_string();

                        if !friendly_name.is_empty() {
                            names_map.insert(gdi_device_name, friendly_name);
                        }
                    }
                }
            }
        }
        names_map
    }

    pub fn enumerate_monitors() -> Result<Vec<Monitor>> {
        let mut monitors = Vec::new();
        let mut dev_num = 0;

        // Pre-fetch friendly names mapping
        let names_map = Self::get_display_names_map();

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
            if (display_device.StateFlags
                & windows::Win32::Graphics::Gdi::DISPLAY_DEVICE_ATTACHED_TO_DESKTOP)
                != 0
            {
                let device_name_os = display_device.DeviceName;
                let device_name_str = String::from_utf16_lossy(&device_name_os)
                    .trim_matches(char::from(0))
                    .to_string();

                let is_primary = (display_device.StateFlags
                    & windows::Win32::Graphics::Gdi::DISPLAY_DEVICE_PRIMARY_DEVICE)
                    != 0;

                // Try to get friendly name from QueryDisplayConfig map first
                let mut friendly_name =
                    names_map.get(&device_name_str).cloned().unwrap_or_default();

                // Fallback to EnumDisplayDevices logic if empty
                if friendly_name.is_empty() {
                    let mut monitor_device = DISPLAY_DEVICEW {
                        cb: mem::size_of::<DISPLAY_DEVICEW>() as u32,
                        ..Default::default()
                    };

                    friendly_name = unsafe {
                        if EnumDisplayDevicesW(
                            PCWSTR::from_raw(display_device.DeviceName.as_ptr()),
                            0,
                            &mut monitor_device,
                            0,
                        )
                        .as_bool()
                        {
                            String::from_utf16_lossy(&monitor_device.DeviceString)
                                .trim_matches(char::from(0))
                                .to_string()
                        } else {
                            format!("Display {}", dev_num + 1)
                        }
                    };
                }

                // If empty, default to Display X
                if friendly_name.is_empty() {
                    friendly_name = format!("Display {}", dev_num + 1);
                }

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

                let current_orientation = unsafe {
                    Orientation::from_u32(dev_mode.Anonymous1.Anonymous2.dmDisplayOrientation.0)
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
                    b.width
                        .cmp(&a.width)
                        .then(b.height.cmp(&a.height))
                        .then(b.frequency.cmp(&a.frequency))
                });

                monitors.push(Monitor {
                    id: device_name_str.clone(),
                    name: friendly_name,
                    device_name: device_name_str,
                    current_resolution: current_res,
                    current_orientation,
                    position,
                    is_primary,
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
            let _ = EnumDisplaySettingsW(device_name_pcwstr, ENUM_CURRENT_SETTINGS, &mut dev_mode);
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
            Err(anyhow!(
                "Failed to change display settings. Error code: {:?}",
                result
            ))
        }
    }

    pub fn set_orientation(device_name: &str, orientation: Orientation) -> Result<()> {
        let mut dev_mode = DEVMODEW {
            dmSize: mem::size_of::<DEVMODEW>() as u16,
            ..Default::default()
        };

        let device_name_w: Vec<u16> = device_name.encode_utf16().chain(Some(0)).collect();
        let device_name_pcwstr = PCWSTR::from_raw(device_name_w.as_ptr());

        // Get current settings first
        unsafe {
            let _ = EnumDisplaySettingsW(device_name_pcwstr, ENUM_CURRENT_SETTINGS, &mut dev_mode);
        }

        let old_orientation =
            unsafe { Orientation::from_u32(dev_mode.Anonymous1.Anonymous2.dmDisplayOrientation.0) };

        // Update orientation
        dev_mode.Anonymous1.Anonymous2.dmDisplayOrientation =
            windows::Win32::Graphics::Gdi::DEVMODE_DISPLAY_ORIENTATION(orientation.to_u32());
        dev_mode.dmFields = DM_DISPLAYORIENTATION;

        // Swap width/height if orientation changes between landscape/portrait types
        let is_old_portrait = old_orientation == Orientation::Portrait
            || old_orientation == Orientation::PortraitFlipped;
        let is_new_portrait =
            orientation == Orientation::Portrait || orientation == Orientation::PortraitFlipped;

        if is_old_portrait != is_new_portrait {
            let w = dev_mode.dmPelsWidth;
            let h = dev_mode.dmPelsHeight;
            dev_mode.dmPelsWidth = h;
            dev_mode.dmPelsHeight = w;
            dev_mode.dmFields |= DM_PELSWIDTH | DM_PELSHEIGHT;
        }

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
            Err(anyhow!(
                "Failed to change display orientation. Error code: {:?}",
                result
            ))
        }
    }
}
