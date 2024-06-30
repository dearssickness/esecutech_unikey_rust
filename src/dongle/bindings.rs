pub const UNIKEY_FIND: u32 = 1;
pub const UNIKEY_FIND_NEXT: u32 = 2;
pub const UNIKEY_LOGON: u32 = 3;
pub const UNIKEY_LOGOFF: u32 = 4;
pub const UNIKEY_READ_MEMORY: u32 = 5;
pub const UNIKEY_WRITE_MEMORY: u32 = 6;
pub const UNIKEY_RANDOM: u32 = 7;
pub const UNIKEY_SEED: u32 = 8;
pub const UNIKEY_WRITE_SOFTID: u32 = 9;
pub const UNIKEY_READ_SOFTID: u32 = 10;
pub const UNIKEY_SET_MODULE: u32 = 11;
pub const UNIKEY_CHECK_MODULE: u32 = 12;
pub const UNIKEY_WRITE_ARITHMETIC: u32 = 13;
pub const UNIKEY_CALCULATE1: u32 = 14;
pub const UNIKEY_CALCULATE2: u32 = 15;
pub const UNIKEY_CALCULATE3: u32 = 16;
pub const UNIKEY_MODULE_DECRASE: u32 = 17;
pub const UNIKEY_MODULE_DECREASE: u32 = 17;
pub const UNIKEY_SET_NEW_PASSWORD: u32 = 18;
pub const UNIKEY_GENERATE_KEY: u32 = 19;
pub const UNIKEY_SET_KEY: u32 = 222;
pub const UNIKEY_ENCRYPT: u32 = 20;
pub const UNIKEY_DECRYPT: u32 = 21;
pub const UNIKEY_MD5: u32 = 22;
pub const UNIKEY_READ_UPDATETAG: u32 = 23;
pub const UNIKEY_WRITE_UPDATETAG: u32 = 24;
pub const UNIKEY_GET_MODULE: u32 = 25;
pub const UNIKEY_GET_TIME: u32 = 26;
pub const UNIKEY_SET_TIME: u32 = 27;
pub const UNIKEY_SET_TIME_NOW: u32 = 28;
pub const UNIKEY_ERASE_TIME_MODULE: u32 = 29;
pub const UNIKEY_SET_TIME_MODULE_START_TIME: u32 = 30;
pub const UNIKEY_SET_TIME_MODULE_START_TIME_NOW: u32 = 31;
pub const UNIKEY_SET_TIME_MODULE_START_TIME_NOW_PC: u32 = 32;
pub const UNIKEY_SET_TIME_MODULE_END_TIME: u32 = 33;
pub const UNIKEY_SET_TIME_MODULE_DURATION: u32 = 34;
pub const UNIKEY_CHECK_TIME_MODULE: u32 = 35;
pub const UNIKEY_CHECK_TIME_MODULE_NOW: u32 = 36;
pub const UNIKEY_CHECK_TIME_MODULE_NOW_PC: u32 = 37;
pub const UNIKEY_GET_MODULE_START_TIME: u32 = 38;
pub const UNIKEY_GET_MODULE_END_TIME: u32 = 39;
pub const UNIKEY_LOCK: u32 = 41;
pub const UNIKEY_UNLOCK: u32 = 42;
pub const UNIKEY_GET_TYPE: u32 = 100;
pub const UNIKEY_TYPE_TIME: u32 = 101;
pub const UNIKEY_TYPE_PRO: u32 = 102;
pub const UNIKEY_TYPE_STD: u32 = 103;
pub const NET_UNIKEY_SET_NUM_CLIENT: u32 = 225;
pub const NET_UNIKEY_GET_NUM_CLIENT: u32 = 226;
pub const UNKEY_GET_CLI_NUM: u32 = 101;
pub const UNKEY_GET_MAX_NUM: u32 = 226;
pub const SUCCESS: u32 = 0;
pub const ERROR_UNIKEY_NOT_FOUND: u32 = 200;
pub const ERROR_UNIKEY_INVALID_PASSWORD: u32 = 201;
pub const ERROR_UNIKEY_INVALID_PASSWORD_OR_ID: u32 = 202;
pub const ERROR_UNIKEY_SET_SOFTID_FAILED: u32 = 203;
pub const ERROR_UNIKEY_INVALID_ADDR_OR_SIZE: u32 = 204;
pub const ERROR_UNIKEY_UNKNOWN_COMMAND: u32 = 205;
pub const ERROR_UNIKEY_NOTBELEVEL3: u32 = 206;
pub const ERROR_UNIKEY_READ_MEMORY: u32 = 207;
pub const ERROR_UNIKEY_WRITE_MEMORY: u32 = 208;
pub const ERROR_UNIKEY_RANDOM: u32 = 209;
pub const ERROR_UNIKEY_SEED: u32 = 210;
pub const ERROR_UNIKEY_CALCULATE: u32 = 211;
pub const ERROR_UNIKEY_NEED_OPEN: u32 = 212;
pub const ERROR_UNIKEY_OPEN_OVERFLOW: u32 = 213;
pub const ERROR_UNIKEY_NOMORE: u32 = 214;
pub const ERROR_UNIKEY_NEED_FIND: u32 = 215;
pub const ERROR_UNIKEY_MODULE: u32 = 216;
pub const ERROR_UNIKEY_AR_BAD_COMMAND: u32 = 217;
pub const ERROR_UNIKEY_AR_UNKNOWN_OPCODE: u32 = 218;
pub const ERROR_UNIKEY_AR_WRONG_BEGIN: u32 = 219;
pub const ERROR_UNIKEY_AR_WRONG_END: u32 = 220;
pub const ERROR_UNIKEY_AR_VALUE_OVERFLOW: u32 = 221;
pub const ERROR_UNIKEY_INVALID_KEY: u32 = 222;
pub const ERROR_UNIKEY_VERIFY_ADV_PASSWORD: u32 = 223;
pub const ERROR_UNIKEY_INVALID_KEY_STORE: u32 = 224;
pub const ERROR_UNIKEY_GENERATE_NEW_PASSWORD: u32 = 225;
pub const ERROR_UNIKEY_READ_UPDATETAG: u32 = 226;
pub const ERROR_UNIKEY_WRITE_UPDATETAG: u32 = 227;
pub const ERROR_UNIKEY_ENCRYPT_FAILED: u32 = 228;
pub const ERROR_UNIKEY_DECRYPT_FAILED: u32 = 229;
pub const ERROR_UNIKEY_READ_TIME: u32 = 230;
pub const ERROR_UNIKEY_WRITE_TIME: u32 = 231;
pub const ERROR_UNIKEY_WRITE_TIME_MODULE: u32 = 232;
pub const ERROR_UNIKEY_COMPARE_TIME_MODULE: u32 = 233;
pub const ERROR_UNIKEY_TIME_MODULE_NOT_NULL: u32 = 234;
pub const ERROR_UNIKEY_TIME_MODULE_OVERDUR: u32 = 235;
pub const ERROR_UNIKEY_ALREADY_LOCKED: u32 = 236;
pub const ERROR_UNIKEY_MAX_USERS: u32 = 237;
pub const ERROR_UNIKEY_MAX_KEYS: u32 = 238;
pub const ERROR_UNIKEY_KEY_INDEX: u32 = 239;
pub const ERROR_UNIKEY_FS_FILE_NAME: u32 = 240;
pub const ERROR_UNIKEY_FS_NO_FILE: u32 = 241;
pub const ERROR_UNIKEY_FS_FILE_OFFSET: u32 = 242;
pub const ERROR_UNIKEY_FS_UNKONW: u32 = 243;
pub const ERROR_UNIKEY_FS_NO_MEMORY: u32 = 244;
pub const ERROR_UNIKEY_FS_FILE_EXIST: u32 = 245;
pub const ERROR_UNIKEY_FS_ERR_SYS_UNINIT: u32 = 246;
pub const ERROR_UNIKEY_FS_ERR_OPEN_FILE: u32 = 247;
pub const ERROR_UNIKEY_NO_ENCYYPT: u32 = 248;
pub const ERROR_UNIKEY_PASSWORD: u32 = 249;
pub const ERROR_UNIKEY_USERLOCK: u32 = 250;
pub const ERROR_UNIKEY_LOGOUT: u32 = 251;
pub const ERROR_UNIKEY_UNKNOW: u32 = 252;
pub const ERROR_UNIKEY_WRITE_ARITHMETIC: u32 = 253;
pub const ERROR_UNIKEY_PARAMETER: u32 = 254;
pub const ERROR_UNIKEY_TOO_MUCH_THREAD: u32 = 255;
pub const ERROR_UNIKEY_GET_TYPE: u32 = 256;
pub const ERROR_UNIKEY_FILE_LOCK_OPEN: u32 = 260;
pub const ERROR_UNIKEY_FILE_LOCK_CLOSE: u32 = 261;
pub const NET_UNIKEY_ERROR_BASE: u32 = 100;
pub const NET_UNIKEY_MEMORY_ERROR: u32 = 101;
pub const NET_UNIKEY_SEND_ERROR: u32 = 102;
pub const NET_UNIKEY_RECEIVE_ERROR: u32 = 103;
pub const NET_UNIKEY_MESSAGE_WRONG: u32 = 104;
pub const NET_UNIKEY_SETUP_SOCKET_ERROR: u32 = 105;
pub const NET_UNIKEY_CLIENT_EXSIT: u32 = 106;
pub const NET_UNIKEY_TOO_MANY_CLIENT: u32 = 107;
pub const NET_UNIKEY_IN_BLACKLIST: u32 = 108;
pub const NET_UNIKEY_OUT_WHITELIST: u32 = 109;
pub const NET_UNIKEY_MESSAGE_CHANGE: u32 = 110;
pub const NET_UNIKEY_AREADY_START: u32 = 111;
pub const NET_UNIKEY_SOCKET_INIT_FAILED: u32 = 112;
pub const NET_UNIKEY_SOCKET_BIND_FAILED: u32 = 113;
pub const NET_UNIKEY_SOCKET_LISTEN_FAILED: u32 = 114;
pub const NET_UNIKEY_START_UDP_SERVER_FAILED: u32 = 115;
pub const NET_UNIKEY_TOO_LONG_MESSAGE: u32 = 116;
pub const NET_UNIKEY_NOT_WORKING: u32 = 117;
pub const NET_UNIKEY_DISCARD_BY_SERVER: u32 = 118;
pub const NET_UNIKEY_SERVER_RESOURCE_INADEQUACY: u32 = 119;
pub const NET_UNIKEY_INIFILE_NOT_EXISTS: u32 = 120;

#[link(name = "unikey", kind = "dylib")]

extern "C" {
    pub fn UniKey_SetNETINILocation(
        pHandle: *mut ::std::os::raw::c_ushort,
        pSetting1: *mut ::std::os::raw::c_uint,
        pSetting2: *mut ::std::os::raw::c_uint,
        szINIPath: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn UniKey_Find(
        pHandle: *mut ::std::os::raw::c_ushort,
        pSetting1: *mut ::std::os::raw::c_uint,
        pSetting2: *mut ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn UniKey_Find_Next(
        pHandle: *mut ::std::os::raw::c_ushort,
        pSetting1: *mut ::std::os::raw::c_uint,
        pSetting2: *mut ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn UniKey_User_Logon(
        pHandle: *mut ::std::os::raw::c_ushort,
        pPassword1: *mut ::std::os::raw::c_ushort,
        pPassword2: *mut ::std::os::raw::c_ushort,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn UniKey_Vender_Logon(
        pHandle: *mut ::std::os::raw::c_ushort,
        pPassword1: *mut ::std::os::raw::c_ushort,
        pPassword2: *mut ::std::os::raw::c_ushort,
        pPassword3: *mut ::std::os::raw::c_ushort,
        pPassword4: *mut ::std::os::raw::c_ushort,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn UniKey_Vendor_Logon(
        pHandle: *mut ::std::os::raw::c_ushort,
        pPassword1: *mut ::std::os::raw::c_ushort,
        pPassword2: *mut ::std::os::raw::c_ushort,
        pPassword3: *mut ::std::os::raw::c_ushort,
        pPassword4: *mut ::std::os::raw::c_ushort,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn UniKey_Logoff(pHandle: *mut ::std::os::raw::c_ushort) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn UniKey_Get_Version(
        pHandle: *mut ::std::os::raw::c_ushort,
        pVersion: *mut ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn UniKey_Get_Type(
        pHandle: *mut ::std::os::raw::c_ushort,
        type_: *mut ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn UniKey_Lock(pHandle: *mut ::std::os::raw::c_ushort) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn UniKey_UnLock(pHandle: *mut ::std::os::raw::c_ushort) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn UniKey_Read_Memory(
        pHandle: *mut ::std::os::raw::c_ushort,
        pStartAddress: *mut ::std::os::raw::c_ushort,
        pBufferLength: *mut ::std::os::raw::c_ushort,
        pBuffer: *mut ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn UniKey_Write_Memory(
        pHandle: *mut ::std::os::raw::c_ushort,
        pStartAddress: *mut ::std::os::raw::c_ushort,
        pBufferLength: *mut ::std::os::raw::c_ushort,
        pBuffer: *mut ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn UniKey_Random(
        pHandle: *mut ::std::os::raw::c_ushort,
        pReturn1: *mut ::std::os::raw::c_ushort,
        pReturn2: *mut ::std::os::raw::c_ushort,
        pReturn3: *mut ::std::os::raw::c_ushort,
        pReturn4: *mut ::std::os::raw::c_ushort,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn UniKey_Seed(
        pHandle: *mut ::std::os::raw::c_ushort,
        pSeed: *mut ::std::os::raw::c_uint,
        pReturn1: *mut ::std::os::raw::c_ushort,
        pReturn2: *mut ::std::os::raw::c_ushort,
        pReturn3: *mut ::std::os::raw::c_ushort,
        pReturn4: *mut ::std::os::raw::c_ushort,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn UniKey_Write_SoftID(
        pHandle: *mut ::std::os::raw::c_ushort,
        pSoftID: *mut ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn UniKey_Read_SoftID(
        pHandle: *mut ::std::os::raw::c_ushort,
        pSoftID: *mut ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn UniKey_Set_Module(
        pHandle: *mut ::std::os::raw::c_ushort,
        pModule: *mut ::std::os::raw::c_ushort,
        pValue: *mut ::std::os::raw::c_ushort,
        pDecrease: *mut ::std::os::raw::c_ushort,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn UniKey_Get_Module(
        pHandle: *mut ::std::os::raw::c_ushort,
        pModule: *mut ::std::os::raw::c_ushort,
        pValue: *mut ::std::os::raw::c_ushort,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn UniKey_Check_Module(
        pHandle: *mut ::std::os::raw::c_ushort,
        pModule: *mut ::std::os::raw::c_ushort,
        pValue: *mut ::std::os::raw::c_ushort,
        pDecrease: *mut ::std::os::raw::c_ushort,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn UniKey_Module_Decrase(
        pHandle: *mut ::std::os::raw::c_ushort,
        pModule: *mut ::std::os::raw::c_ushort,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn UniKey_Module_Decrease(
        pHandle: *mut ::std::os::raw::c_ushort,
        pModule: *mut ::std::os::raw::c_ushort,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn UniKey_Write_Arithmetic(
        pHandle: *mut ::std::os::raw::c_ushort,
        pStartAddress: *mut ::std::os::raw::c_ushort,
        pBuffer: *mut ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn UniKey_Calculate1(
        pHandle: *mut ::std::os::raw::c_ushort,
        pStartAddress: *mut ::std::os::raw::c_uint,
        pModule: *mut ::std::os::raw::c_uint,
        pRegA: *mut ::std::os::raw::c_ushort,
        pRegB: *mut ::std::os::raw::c_ushort,
        pRegC: *mut ::std::os::raw::c_ushort,
        pRegD: *mut ::std::os::raw::c_ushort,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn UniKey_Calculate2(
        pHandle: *mut ::std::os::raw::c_ushort,
        pStartAddress: *mut ::std::os::raw::c_uint,
        pSeed: *mut ::std::os::raw::c_uint,
        pRegA: *mut ::std::os::raw::c_ushort,
        pRegB: *mut ::std::os::raw::c_ushort,
        pRegC: *mut ::std::os::raw::c_ushort,
        pRegD: *mut ::std::os::raw::c_ushort,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn UniKey_Calculate3(
        pHandle: *mut ::std::os::raw::c_ushort,
        pStartAddress: *mut ::std::os::raw::c_uint,
        pModule: *mut ::std::os::raw::c_uint,
        pRegA: *mut ::std::os::raw::c_ushort,
        pRegB: *mut ::std::os::raw::c_ushort,
        pRegC: *mut ::std::os::raw::c_ushort,
        pRegD: *mut ::std::os::raw::c_ushort,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn UniKey_Generate_New_Password(
        pHandle: *mut ::std::os::raw::c_ushort,
        pSeed: *mut ::std::os::raw::c_uint,
        pPassword1: *mut ::std::os::raw::c_ushort,
        pPassword2: *mut ::std::os::raw::c_ushort,
        pPassword3: *mut ::std::os::raw::c_ushort,
        pPassword4: *mut ::std::os::raw::c_ushort,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn UniKey_Generate_Key(
        pHandle: *mut ::std::os::raw::c_ushort,
        pKeyNumber: *mut ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn UniKey_Generate_Key_Via_Seed(
        pHandle: *mut ::std::os::raw::c_ushort,
        pKeyNumber: *mut ::std::os::raw::c_uint,
        pSeed1: *mut ::std::os::raw::c_ushort,
        pSeed2: *mut ::std::os::raw::c_ushort,
        pSeed3: *mut ::std::os::raw::c_ushort,
        pSeed4: *mut ::std::os::raw::c_ushort,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn UniKey_Encrypt(
        pHandle: *mut ::std::os::raw::c_ushort,
        pBufferLength: *mut ::std::os::raw::c_uint,
        pKeyNumber: *mut ::std::os::raw::c_uint,
        pBuffer: *mut ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn UniKey_Decrypt(
        pHandle: *mut ::std::os::raw::c_ushort,
        pBufferLength: *mut ::std::os::raw::c_uint,
        pKeyNumber: *mut ::std::os::raw::c_uint,
        pBuffer: *mut ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn UniKey_MD5(
        pHandle: *mut ::std::os::raw::c_ushort,
        pBufferLength: *mut ::std::os::raw::c_uint,
        pBuffer: *mut ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn UniKey_Read_UpdateTag(
        pHandle: *mut ::std::os::raw::c_ushort,
        pUpdateTag: *mut ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn UniKey_Write_UpdateTag(
        pHandle: *mut ::std::os::raw::c_ushort,
        pUpdateTag: *mut ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn UniKey_Get_Time(
        pHandle: *mut ::std::os::raw::c_ushort,
        pYear: *mut ::std::os::raw::c_uint,
        pMonth: *mut ::std::os::raw::c_uint,
        pDay: *mut ::std::os::raw::c_ushort,
        pHour: *mut ::std::os::raw::c_ushort,
        pMinute: *mut ::std::os::raw::c_ushort,
        pSecond: *mut ::std::os::raw::c_ushort,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn UniKey_Set_Time(
        pHandle: *mut ::std::os::raw::c_ushort,
        pYear: *mut ::std::os::raw::c_uint,
        pMonth: *mut ::std::os::raw::c_uint,
        pDay: *mut ::std::os::raw::c_ushort,
        pHour: *mut ::std::os::raw::c_ushort,
        pMinute: *mut ::std::os::raw::c_ushort,
        pSecond: *mut ::std::os::raw::c_ushort,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn UniKey_Set_Time_Now(pHandle: *mut ::std::os::raw::c_ushort) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn UniKey_Erase_Time_Module(
        pHandle: *mut ::std::os::raw::c_ushort,
        pModule: *mut ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn UniKey_Set_Time_Module_Start_Time(
        pHandle: *mut ::std::os::raw::c_ushort,
        pModule: *mut ::std::os::raw::c_uint,
        pYear: *mut ::std::os::raw::c_ushort,
        pMonth: *mut ::std::os::raw::c_ushort,
        pDay: *mut ::std::os::raw::c_ushort,
        pHour: *mut ::std::os::raw::c_ushort,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn UniKey_Set_Time_Module_Start_Time_Now(
        pHandle: *mut ::std::os::raw::c_ushort,
        pModule: *mut ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn UniKey_Set_Time_Module_Start_Time_Now_PC(
        pHandle: *mut ::std::os::raw::c_ushort,
        pModule: *mut ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn UniKey_Set_Time_Module_End_Time(
        pHandle: *mut ::std::os::raw::c_ushort,
        pModule: *mut ::std::os::raw::c_uint,
        pYear: *mut ::std::os::raw::c_ushort,
        pMonth: *mut ::std::os::raw::c_ushort,
        pDay: *mut ::std::os::raw::c_ushort,
        pHour: *mut ::std::os::raw::c_ushort,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn UniKey_Set_Time_Module_Duration(
        pHandle: *mut ::std::os::raw::c_ushort,
        pModule: *mut ::std::os::raw::c_uint,
        pYear: *mut ::std::os::raw::c_uint,
        pDay: *mut ::std::os::raw::c_ushort,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn UniKey_Check_Time_Module(
        pHandle: *mut ::std::os::raw::c_ushort,
        pModule: *mut ::std::os::raw::c_uint,
        pRemainDays: *mut ::std::os::raw::c_uint,
        pYear: *mut ::std::os::raw::c_ushort,
        pMonth: *mut ::std::os::raw::c_ushort,
        pDay: *mut ::std::os::raw::c_ushort,
        pHour: *mut ::std::os::raw::c_ushort,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn UniKey_Check_Time_Module_Now(
        pHandle: *mut ::std::os::raw::c_ushort,
        pModule: *mut ::std::os::raw::c_uint,
        pRemainDays: *mut ::std::os::raw::c_uint,
        pRemainHours: *mut ::std::os::raw::c_ushort,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn UniKey_Check_Time_Module_Now_PC(
        pHandle: *mut ::std::os::raw::c_ushort,
        pModule: *mut ::std::os::raw::c_uint,
        pRemainDays: *mut ::std::os::raw::c_uint,
        pRemainHours: *mut ::std::os::raw::c_ushort,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn UniKey_Get_Module_Start_Time(
        pHandle: *mut ::std::os::raw::c_ushort,
        pModule: *mut ::std::os::raw::c_uint,
        pYear: *mut ::std::os::raw::c_ushort,
        pMonth: *mut ::std::os::raw::c_ushort,
        pDay: *mut ::std::os::raw::c_ushort,
        pHour: *mut ::std::os::raw::c_ushort,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn UniKey_Get_Module_End_Time(
        pHandle: *mut ::std::os::raw::c_ushort,
        pModule: *mut ::std::os::raw::c_uint,
        pYear: *mut ::std::os::raw::c_ushort,
        pMonth: *mut ::std::os::raw::c_ushort,
        pDay: *mut ::std::os::raw::c_ushort,
        pHour: *mut ::std::os::raw::c_ushort,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn UniKey_Get_Dongle_Location(
        pHandle: *mut ::std::os::raw::c_ushort,
        pIPAddress: *mut ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn UniKey_Get_Cli_Num(
        pHandle: *mut ::std::os::raw::c_ushort,
        cnt: *mut ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn UniKey_Get_Max_Num(
        pHandle: *mut ::std::os::raw::c_ushort,
        cnt: *mut ::std::os::raw::c_ushort,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn UniKey_Set_Max_Num(
        pHandle: *mut ::std::os::raw::c_ushort,
        cnt: *mut ::std::os::raw::c_ushort,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn UniKey(
        Function: ::std::os::raw::c_ushort,
        handle: *mut ::std::os::raw::c_ushort,
        lp1: *mut ::std::os::raw::c_uint,
        lp2: *mut ::std::os::raw::c_uint,
        p1: *mut ::std::os::raw::c_ushort,
        p2: *mut ::std::os::raw::c_ushort,
        p3: *mut ::std::os::raw::c_ushort,
        p4: *mut ::std::os::raw::c_ushort,
        buffer: *mut ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_long;
}
