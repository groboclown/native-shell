//! Grants access to user and group information.

use std::sync::{LazyLock, Mutex};
use libc::{uid_t, gid_t};
use users::{Users, Groups};

static USER_CACHE: LazyLock<Mutex<users::UsersCache>> = LazyLock::new(|| {
    unsafe { Mutex::new(users::UsersCache::with_all_users()) }
});

pub fn get_user_group(
    user: &Option<String>,
    group: &Option<String>,
    user_id: &Option<f64>,
    group_id: &Option<f64>,
) -> Result<Option<(uid_t, gid_t)>, String> {
    if user.is_none() && group.is_none() && user_id.is_none() && group_id.is_none() {
        return Ok(None); // No user or group specified
    }
    if user.is_some() && user_id.is_some() {
        return Err("Both user and user_id specified".to_string());
    }
    if group.is_some() && group_id.is_some() {
        return Err("Both group and group_id specified".to_string());
    }

    let ret_uid: uid_t;
    if user_id.is_some() {
        ret_uid = get_user_id_from_number(user_id)
            .map_err(|e| format!("Failed to get user ID from float: {}", e))?;
    } else if user.is_some() {
        ret_uid = get_user_id_from_name(user)
            .map_err(|e| format!("Failed to get user ID from string: {}", e))?;
    } else {
        ret_uid = users::get_current_uid();
    }

    let ret_guid: gid_t;
    if group_id.is_some() {
        ret_guid = get_group_id_from_number(group_id)
            .map_err(|e| format!("Failed to get group ID from float: {}", e))?;
    } else if group.is_some() {
        ret_guid = get_group_id_from_name(group)
            .map_err(|e| format!("Failed to get group ID from string: {}", e))?;
    } else {
        ret_guid = users::get_current_gid();
    }

    Ok(Some((ret_uid, ret_guid)))
}

fn get_user_id_from_number(id: &Option<f64>) -> Result<uid_t, String> {
    match id {
        Some(f_id) if *f_id >= 0.0 && *f_id <= i32::MAX as f64 => Ok(*f_id as uid_t),
        Some(_) => Err("user id must be a non-negative integer".to_string()),
        None => Ok(users::get_current_uid()),
    }
}

fn get_user_id_from_name(user: &Option<String>) -> Result<uid_t, String> {
    match user {
        Some(u) => Ok(
            USER_CACHE.lock().as_ref()
            .map_err(|e| format!("Failed to get user ID for {}: {}", u, e))?
            .get_user_by_name(u)
            .unwrap()
            .uid()
        ),

        None => Ok(users::get_current_uid()),
    }
}

fn get_group_id_from_number(id: &Option<f64>) -> Result<gid_t, String> {
    match id {
        Some(f_id) if *f_id >= 0.0 && *f_id <= i32::MAX as f64 => Ok(*f_id as gid_t),
        Some(_) => Err("group id must be a non-negative integer".to_string()),
        None => Ok(users::get_current_gid()),
    }
}

fn get_group_id_from_name(user: &Option<String>) -> Result<gid_t, String> {
    match user {
        Some(u) => Ok(
            USER_CACHE.lock().as_ref()
            .map_err(|e| format!("Failed to get group ID for {}: {}", u, e))?
            .get_group_by_name(u)
            .unwrap()
            .gid()
        ),

        None => Ok(users::get_current_gid()),
    }
}
