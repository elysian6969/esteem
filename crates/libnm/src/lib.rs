#![allow(unused_variables)]

use core::ptr;

macro_rules! stub {
    ($name:ident) => {
        #[no_mangle]
        pub extern "C" fn $name() {
            frosting::println!();

            ()
        }
    };
}

stub!(nm_access_point_get_flags);
stub!(nm_access_point_get_rsn_flags);
stub!(nm_access_point_get_ssid);
stub!(nm_access_point_get_strength);
stub!(nm_access_point_get_wpa_flags);
stub!(nm_active_connection_get_default);
stub!(nm_active_connection_get_default6);
stub!(nm_active_connection_get_state);
stub!(nm_client_add_and_activate_connection_async);
stub!(nm_client_add_and_activate_connection_finish);
stub!(nm_client_get_devices);
stub!(nm_client_get_type);
//stub!(nm_client_new);
stub!(nm_client_wireless_get_enabled);
stub!(nm_client_wireless_set_enabled);
stub!(nm_connection_add_setting);
stub!(nm_connection_get_setting_connection);
stub!(nm_connection_get_type);
stub!(nm_connection_need_secrets);
stub!(nm_connection_verify);
stub!(nm_device_disconnect_async);
stub!(nm_device_disconnect_finish);
stub!(nm_device_ethernet_get_carrier);
stub!(nm_device_ethernet_get_permanent_hw_address);
stub!(nm_device_ethernet_get_speed);
stub!(nm_device_ethernet_get_type);
stub!(nm_device_get_active_connection);
stub!(nm_device_get_available_connections);
stub!(nm_device_get_dhcp4_config);
stub!(nm_device_get_dhcp6_config);
stub!(nm_device_get_iface);
stub!(nm_device_get_ip4_config);
stub!(nm_device_get_ip6_config);
stub!(nm_device_get_product);
stub!(nm_device_get_state);
stub!(nm_device_get_state_reason);
stub!(nm_device_get_type);
stub!(nm_device_get_udi);
stub!(nm_device_get_vendor);
stub!(nm_device_wifi_get_access_points);
stub!(nm_device_wifi_get_active_access_point);
stub!(nm_device_wifi_get_capabilities);
stub!(nm_device_wifi_get_permanent_hw_address);
stub!(nm_device_wifi_get_type);
stub!(nm_device_wifi_request_scan_async);
stub!(nm_device_wifi_request_scan_finish);
stub!(nm_ip_address_get_address);
stub!(nm_ip_address_get_prefix);
stub!(nm_ip_address_new);
stub!(nm_ip_address_unref);
stub!(nm_ip_config_get_addresses);
stub!(nm_ip_config_get_gateway);
stub!(nm_ip_config_get_nameservers);
stub!(nm_remote_connection_commit_changes_async);
stub!(nm_remote_connection_commit_changes_finish);
stub!(nm_remote_connection_delete_async);
stub!(nm_remote_connection_delete_finish);
stub!(nm_remote_connection_get_secrets_async);
stub!(nm_remote_connection_get_secrets_finish);
stub!(nm_remote_connection_get_type);
stub!(nm_setting_802_1x_add_eap_method);
stub!(nm_setting_802_1x_new);
stub!(nm_setting_connection_get_autoconnect);
stub!(nm_setting_connection_get_id);
stub!(nm_setting_connection_new);
stub!(nm_setting_get_type);
stub!(nm_setting_ip4_config_new);
stub!(nm_setting_ip6_config_new);
stub!(nm_setting_ip_config_add_address);
stub!(nm_setting_ip_config_add_dns);
stub!(nm_setting_ip_config_get_type);
stub!(nm_setting_wired_new);
stub!(nm_setting_wireless_new);
stub!(nm_setting_wireless_security_new);
stub!(nm_simple_connection_new);
stub!(nm_utils_security_valid);
stub!(nm_utils_ssid_to_utf8);
stub!(nm_utils_uuid_generate);

// https://developer-old.gnome.org/libnm/stable/NMClient.html
// https://developer-old.gnome.org/libnm/stable/NMClient.html#nm-client-new
#[no_mangle]
pub extern "C" fn nm_client_new(cancellable: *const (), error: *mut *const ()) -> *const () {
    frosting::println!();

    ptr::null()
}
