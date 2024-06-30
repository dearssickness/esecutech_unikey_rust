use openssl::ssl::{SslMethod, SslAcceptor, SslStream, SslFiletype};
use std::net::{TcpListener, TcpStream};
use chrono::{NaiveDate, NaiveDateTime, Utc};
use serde_json::{Result, Value};

pub fn handle_client(license_data: String, mut stream: SslStream<TcpStream>){
    
    let license_data_json: Value = serde_json::from_str(&license_data).expect("JSON was not well-formatted");

//    println!("Camera count is {}", license_data_json["7ba01c1cb389724085004d6e246c0345"]);
//    println!("Number of faces is {}", license_data_json["4d45769c85b4703a8810315ff4480117"]);
//    println!("Expire date is {}", license_data_json["50fbc66dbc4561964a9e074d2677bb47"]);
//    println!("Number of extraction APIs is {}", license_data_json["db06018ad137d38d03203f90e2e74374"]);


//    let license_expire_time: NaiveDateTime = NaiveDate::from_ymd(2040, 01, 01).and_hms(12, 00, 00);
//    let _license_face_count = 7777777;
//    let license_camera_count = 100;
//    let license_api_count = 254;
//    let mut license_data: [u8; 45] = [0; 45];
//    let exp_timestamp_le = license_expire_time.timestamp().to_le_bytes();
//    let now_timestamp_le = Utc::now().naive_local().timestamp().to_le_bytes(); 

    let mut license_data: [u8; 45] = [0; 45];

    let mut license_expire_time: String = "{}".to_owned();
    let mut license_expire_time = license_data_json["50fbc66dbc4561964a9e074d2677bb47"].to_string();
    let mut license_expire_time_trim = &license_expire_time.replace('"', "");
    let exp_timestamp_le  = NaiveDateTime::parse_from_str(license_expire_time_trim, "%Y-%m-%d %H:%M:%S")
                            .unwrap().timestamp().to_le_bytes();


    let now_timestamp_le = Utc::now().naive_local().timestamp().to_le_bytes(); 

    let mut license_face_count: String = "{}".to_owned();
    license_face_count = license_data_json["4d45769c85b4703a8810315ff4480117"].to_string();
    let license_face_count_int: i32 = license_face_count.parse().unwrap();
    let license_face_count_vec = license_face_count_int.to_le_bytes();

    let mut license_api_count: String = "{}".to_owned();
    license_api_count = license_data_json["db06018ad137d38d03203f90e2e74374"].to_string();
    let license_api_count_u8: u8 = license_api_count.parse().unwrap();

    let mut license_camera_count: String = "{}".to_owned();
    license_camera_count = license_data_json["7ba01c1cb389724085004d6e246c0345"].to_string();
    let license_camera_count_u8: u8 = license_camera_count.parse().unwrap();

    license_data[0] = now_timestamp_le[0];
    license_data[1] = now_timestamp_le[1];
    license_data[2] = now_timestamp_le[2];
    license_data[3] = now_timestamp_le[3];

    license_data[4] = 0x00;
    license_data[5] = 0x00;
    license_data[6] = 0x00;
    license_data[7] = 0x00;

    license_data[8] = exp_timestamp_le[0];
    license_data[9] = exp_timestamp_le[1];
    license_data[10] = exp_timestamp_le[2];
    license_data[11] = exp_timestamp_le[3];

    license_data[12] = 0x00;
    license_data[13] = 0x00;
    license_data[14] = 0x00;
    license_data[15] = 0x00;
    license_data[16] = 0x01;

    // TODO:   license_data[17] = license_face_count;
    // From 17 to 20
//    license_data[17] = 0xff;
//    license_data[18] = 0xc9;
//    license_data[19] = 0x9a;
//    license_data[20] = 0x3b;

    license_data[17] = license_face_count_vec[0];
    license_data[18] = license_face_count_vec[1];
    license_data[19] = license_face_count_vec[2];
    license_data[20] = license_face_count_vec[3];

    license_data[21] = 0x00;
    license_data[22] = 0x00;
    license_data[23] = 0x00;
    license_data[24] = 0x00;
    license_data[25] = 0xff;
    license_data[26] = 0x03;
    license_data[27] = 0x00;
    license_data[28] = 0x00;
    license_data[29] = license_camera_count_u8;
    license_data[30] = 0x00;
    license_data[31] = 0x00;
    license_data[32] = 0x00;
    license_data[33] = 0x60;
    license_data[34] = 0x54;
    license_data[35] = 0x00;
    license_data[36] = 0x00;
    license_data[37] = 0x00;
    license_data[38] = 0x00;
    license_data[39] = 0x00;
    license_data[40] = 0x00;
    license_data[41] = license_api_count_u8;
    license_data[42] = 0x00;
    license_data[43] = 0x00;
    license_data[44] = 0x00;

    stream.ssl_write(&license_data).unwrap();
}
