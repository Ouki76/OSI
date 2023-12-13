use serde_json::{self, json};
use sysinfo::{ComponentExt, CpuExt, DiskExt, SystemExt, UserExt};

pub struct System {
    pub system: sysinfo::System,
}

impl System {
    pub fn get_json_system_info(&self) -> String {
        let sys = &self.system;

        serde_json::json!({
            "available_memory": sys.available_memory(),
            "boot_time": sys.boot_time(),
            "components": sys.components().iter().map(|component| {
                json!({
                    "critical": component.critical(),
                    "label": component.label(),
                    "max": component.max(),
                    "temperature": component.temperature()
                })
            }).collect::<Vec<serde_json::Value>>(),
            "cpus": sys.cpus().iter().map(|cpu| {
                json!({
                    "brand": cpu.brand(),
                    "cpu_usage": cpu.cpu_usage(),
                    "frequency": cpu.frequency(),
                    "name": cpu.name(),
                    "vendor_id": cpu.vendor_id()
                })
            }).collect::<Vec<serde_json::Value>>(),
            "disks": sys.disks().iter().map(|disk| {
                json!({
                    "available_space": disk.available_space(),
                    "file_system": disk.file_system(),
                    "is_removable": disk.is_removable(),
                    "mount_point": disk.mount_point(),
                    "name": disk.name(),
                    "total_space": disk.total_space()
                })
            }).collect::<Vec<serde_json::Value>>(),
            "distribution_id": sys.distribution_id(),
            "free_memory": sys.free_memory(),
            "free_swap": sys.free_swap(),
            "host_name": sys.host_name(),
            "kernel_version": sys.kernel_version(),
            "load_average": {
                "fifteen": sys.load_average().fifteen,
                "five": sys.load_average().five,
                "one": sys.load_average().one
            },
            "long_os_version": sys.long_os_version(),
            "name": sys.name(),
            "os_version": sys.os_version(),
            "total_memory": sys.total_memory(),
            "total_swap": sys.total_swap(),
            "uptime": sys.uptime(),
            "used_memory": sys.used_memory(),
            "used_swap": sys.used_swap(),
            "users": sys.users().iter().map(|user| {
                json!({
                    "group_id": user.group_id().to_string(),
                    "groups": user.groups(),
                    "id": user.id().to_string(),
                    "name": user.name()
                })
            }).collect::<Vec<serde_json::Value>>()
        })
        .to_string()
    }

    pub fn get_by_name(&self, info: &str) -> serde_json::Value {
        let v: serde_json::Value = serde_json::from_str(&self.get_json_system_info()).unwrap();

        json!({
            info: v[info].to_string()
        })
    }

    pub fn refresh(&mut self) {
        self.system.refresh_all();
    }

    pub fn new() -> Self {
        System {
            system: sysinfo::System::new(),
        }
    }
}
