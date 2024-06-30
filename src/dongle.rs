use std::mem;
use std::mem::MaybeUninit;
use std::ptr;
use std::fmt::Write;
use openssl::symm::{Cipher, Mode, Crypter};

mod bindings;

pub fn read_dongle(mut num_p1: u16, mut num_p2: u16) -> String {

        let p1: *mut ::std::os::raw::c_ushort = &mut num_p1;
    
        let p2: *mut ::std::os::raw::c_ushort = &mut num_p2;
    
        let mut num_pStartAddress = 0;
        let pStartAddress: *mut ::std::os::raw::c_ushort = &mut num_pStartAddress;
    
        let mut num_pBufferLength = 4096;
        let pBufferLength: *mut ::std::os::raw::c_ushort = &mut num_pBufferLength;
        
        let mut x_pBuffer: [::std::os::raw::c_uchar; 4096] = [0; 4096];
        let mut pBuffer = x_pBuffer.as_mut_ptr();
        
        let mut num_pSetting1 = 10;
        let pSetting1: *mut ::std::os::raw::c_uint = &mut num_pSetting1;
    
        let mut num_pSetting2 = 10;
        let pSetting2: *mut ::std::os::raw::c_uint = &mut num_pSetting2;
    
        let mut x = std::mem::MaybeUninit::<::std::os::raw::c_ushort>::uninit();
        let mut handle = x.as_mut_ptr();

    unsafe {
        let retcode = bindings::UniKey_Find(handle, pSetting1, pSetting2);
        println!("Find is working");
        println!("retcode is: {}", retcode);
        let retcode = bindings::UniKey_User_Logon(handle, p1, p2);
        println!("logon is working");
        println!("retcode is: {}", retcode);
        let retcode = bindings::UniKey_Read_Memory(handle, pStartAddress, pBufferLength, pBuffer);
        println!("read memory is working");
        println!("retcode is: {}", retcode);
        println!("data is {}", *pBuffer);
        let retcode = bindings::UniKey_Logoff(handle);
        println!("logoff is working");
        println!("retcode is: {}", retcode);
    }

        let slice = unsafe { std::slice::from_raw_parts(pBuffer, 4096)};
    
        let s = std::str::from_utf8(&slice[0..18]).expect("invalid utf-8 sequence");

        let mut hex_thing = [0x24; 18];
        hex_thing.copy_from_slice(&slice[0..18]);

        let mut key_buf = [0x42; 16];
        let mut iv_buf = [0x24; 16];
        key_buf.copy_from_slice(&slice[3072..3088]);
        iv_buf.copy_from_slice(&slice[3088..3104]);

        let mut license_data_buf = [0; 1024];
        license_data_buf.copy_from_slice(&slice[2048..3072]);

        
        let mut decrypter = Crypter::new(
            Cipher::aes_128_cbc(),
            Mode::Decrypt,
            &key_buf,
            Some(&iv_buf)).unwrap();

        decrypter.pad(false);
        
        let block_size = Cipher::aes_128_cbc().block_size();

        println!("data len is {}", license_data_buf.len());

        let mut out = vec![0; license_data_buf.len() + block_size];

        let count = decrypter.update(&license_data_buf, &mut out).unwrap();

        let mut finalized = decrypter.finalize(&mut out).unwrap();

        let lossy_string = String::from_utf8_lossy(&out);

        let mut plaintext_clone = lossy_string.clone();

        let plaintext_split = plaintext_clone.split("}");
        
        let plaintext_vec: Vec<&str> = plaintext_split.collect();

        let mut plaintext = plaintext_vec[0].to_string() + "}";

        println!("plaintext is out is {}", plaintext);

        plaintext
}
