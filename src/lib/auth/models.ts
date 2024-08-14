// pub struct SerializedUserAccount {
//     pub first_name: String,
//     pub last_name: String,
//     pub is_room_manager: bool,
//     pub is_config_manager: bool,
//     pub is_auditor: bool,
//     pub is_user_manager: bool,
//     pub is_group_manager: bool,
//     pub email: String,
//     pub user_id: u64,
// }

export interface UserAccount {
    firstName: string;
    lastName: string;
    isRoomManager: boolean;
    isConfigManager: boolean;
    isAuditor: boolean;
    isUserManager: boolean;
    isGroupManager: boolean;
    email: string;
    userId: number;
    isCloud: boolean;
}