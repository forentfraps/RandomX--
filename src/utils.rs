

#[allow(non_snake_case, dead_code, unused)]
pub mod Gens{
    use aes::Aes128;
    use aes::cipher::{
        BlockCipher, BlockEncrypt, BlockDecrypt, KeyInit,
        generic_array::GenericArray,
    };
    use std::convert::TryInto;

    pub fn AesGenerator1R(){}
    pub fn AesGenerator4R(){}
    pub fn AesHash1R(){}
    pub fn BlakeGenerator(){}
    pub fn newAes128(key: &[u8; 16]) -> Aes128{
        Aes128::new(&GenericArray::from(*key))
    }
    pub fn Aes128Enc(aes128: &Aes128,  data: &[u8; 16]) -> [u8; 16]{
        let mut block = GenericArray::from(*data);
        aes128.encrypt_block(&mut block);
        block.as_slice().to_owned().try_into().unwrap()
    }
    pub fn Aes128Dec( data: &[u8; 16], key: &[u8; 16]) -> [u8; 16]{
        let gen_key = GenericArray::from(key.to_owned());
        let cipher = Aes128::new(&gen_key);
        let mut block = GenericArray::from(data.to_owned());
        cipher.decrypt_block(&mut block);
        block.as_slice().to_owned().try_into().unwrap()
    }
    pub fn Hash512(){}
    pub fn Hash256(){}

}