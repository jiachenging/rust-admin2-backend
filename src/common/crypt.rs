use std::str;
//use crypto::{symmetriccipher,buffer,aes,blockmodes};
//use crypto::buffer::{ReadBuffer,WriteBuffer,BufferResult};
use super::JsonData;
use serde::{Serialize, Deserialize};
//use crypto::aessafe::*;
//use crypto::blockmodes::*;
//use crypto::symmetriccipher::*;
use short_crypt::ShortCrypt;

const KEY_PR: &'static str = "A391D)@!9sdk#ayS$*#6123#lVB@^?<5"; // key, 32位长度
// const KEY_IV: &'static str = "Y45#^&(!DA93?H6s"; // iv, 16位长度

/// 将结构体进行加密
pub fn encrypt<T: Serialize>(obj: &T) -> Result<JsonData, &'static str> { 
    let value = if let Ok(v) = serde_json::to_string(obj) { v } else { return Err("将结构体序列化时出错"); };
    let sc = ShortCrypt::new(KEY_PR);
    let encrypt_string = sc.encrypt_to_url_component(&value);
    //let data_string = if let Ok(v) = String::from_utf8(encoded_vec) { v } else { return Err("将加密结果转化为字符时出错"); };
    let json_data = JsonData{ 
        data: encrypt_string,
    };
    Ok(json_data)
}

pub fn decrypt_string(encrypt_string: &str) -> Result<String, &'static str> {
    let sc = ShortCrypt::new(KEY_PR);
    match sc.decrypt_url_component(encrypt_string) {
        Ok(v) => {
            match String::from_utf8(v) {
                Ok(s) => Ok(s),
                Err(err) => {
                    Err("反解析字符串时出错")
                }
            }
        },
        Err(err) => {
            Err("反解密字符串时出错")
        }
    }
}

// 对内容进行解密
pub fn decrypt<'a, T: Deserialize<'a>>(data: &'a str) -> Result<T, &'static str> {
    let decrypt_obj: T = if let Ok(v) = serde_json::from_str(data) { v } else { return Err("反解析数据时出错"); };
    Ok(decrypt_obj)
}

// 将数据对象进行组装加密
// pub fn encrypt_data<T: Serialize>(obj: &T) -> Result<JsonData, &'static str> {
//     let value = if let Ok(v) = serde_json::to_string(obj) { v } else { return Err(""); };
//     let encrypt_string = encrypt(&value);
//     Ok(JsonData{ 
//         data: encrypt_string,
//     })
// }

// //将数据对象进行解组解密
// pub fn decrypt_data<T: Deserialize<'static>>(obj: &JsonData) -> Result<T, &'static str> { 
//     let decrypt_string = decrypt(&obj.data);
//     decrypt_json(&decrypt_string)
// }

// 将字符串进行解密得到结构体
// fn  decrypt_data<'a, T: Deserialize<'a>>(data: &'a str) -> Result<T, &'static str> { 
//     //let decrypt_string = decrypt(&obj.data);
//     let decrypt_obj: T = if let Ok(v) = serde_json::from_str(data) { v } else { return Err(""); };
//     Ok(decrypt_obj)
// }

// 对字符串进行加密
// pub fn encrypt(data: &str) -> String { 
//     let key = KEY_PR.as_bytes();
//     let iv = KEY_IV.as_bytes();
//     let encrypted_data = aes256_cbc_encrypt(data.as_bytes(), &key, &iv).ok().unwrap();
//     String::from_utf8(encrypted_data).unwrap()
// }

// 对数据解密
// pub fn decrypt(data: &str) -> String { 
//     let key = KEY_PR.as_bytes();
//     let iv = KEY_IV.as_bytes();
//     let encrypt_data = data.as_bytes();
//     let decrypted_data = aes256_cbc_decrypt(&encrypt_data[..], &key, &iv).ok().unwrap();
//     String::from_utf8(decrypted_data).unwrap()
// }

// Encrypt a buffer with the given key and iv using AES-256/CBC/Pkcs encryption.
// fn aes256_cbc_encrypt(data: &[u8],key: &[u8], iv: &[u8]) -> Result<Vec<u8>, symmetriccipher::SymmetricCipherError> {
//    let mut encryptor = aes:: cbc_encryptor(aes::KeySize::KeySize256, key, iv, blockmodes::PkcsPadding);
//    let mut final_result = Vec::<u8>::new();
//    let mut read_buffer = buffer::RefReadBuffer::new(data);
//    let mut buffer = [0;4096];
//    let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);
//
//    loop {
//        let result = encryptor.encrypt(&mut read_buffer, &mut write_buffer, true).unwrap();
//        final_result.extend(write_buffer.take_read_buffer().take_remaining().iter().map(|&i| i));
//        match result {
//            BufferResult::BufferUnderflow => break,
//            BufferResult::BufferOverflow => {},
//        }
//    }
//
//    Ok(final_result)
// }

// Decrypts a buffer with the given key and iv using AES-256/CBC/Pkcs encryption.
// fn aes256_cbc_decrypt(encrypted_data: &[u8], key: &[u8], iv: &[u8]) -> Result<Vec<u8>, symmetriccipher::SymmetricCipherError> {
//     let mut decryptor = aes::cbc_decryptor(aes::KeySize::KeySize256, key, iv, blockmodes::PkcsPadding);
//     let mut final_result = Vec::<u8>::new();
//     let mut read_buffer = buffer::RefReadBuffer::new(encrypted_data);
//     let mut buffer = [0; 4096];
//     let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);
// 
//     loop {
//         let result = decryptor.decrypt(&mut read_buffer, &mut write_buffer, true).unwrap();
//         final_result.extend(write_buffer.take_read_buffer().take_remaining().iter().map(|&i| i));
//         match result {
//             BufferResult::BufferUnderflow => break,
//             BufferResult::BufferOverflow => { }
//         }
//     }
// 
//     Ok(final_result)
// }
